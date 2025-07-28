use anyhow::anyhow;
use axum::{
    extract::{
        State, WebSocketUpgrade,
        ws::{Message, WebSocket},
    },
    response::IntoResponse,
};
use dashmap::DashMap;
use futures_util::StreamExt;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::mpsc::{UnboundedSender, unbounded_channel};
use tokio_stream::wrappers::UnboundedReceiverStream;
use tracing::{debug, trace};
use webrtc::{
    api::{
        APIBuilder, interceptor_registry::register_default_interceptors, media_engine::MediaEngine,
    },
    ice_transport::ice_candidate::RTCIceCandidateInit,
    interceptor::registry::Registry,
    peer_connection::{
        RTCPeerConnection, configuration::RTCConfiguration,
        peer_connection_state::RTCPeerConnectionState,
        sdp::session_description::RTCSessionDescription,
    },
    rtp_transceiver::rtp_codec::RTPCodecType::{Audio, Video},
    track::{
        track_local::{TrackLocalWriter, track_local_static_rtp::TrackLocalStaticRTP},
        track_remote::TrackRemote,
    },
};

use crate::{
    app::AppState,
    jwt::Claims,
    room::{RoomEvent, TrackIdentifier, UserPacket},
};

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
enum WSUserMessage {
    RTCOffer { offer: RTCSessionDescription },
    RTCAnswer { answer: RTCSessionDescription },
    RTCCandidate { candidate: RTCIceCandidateInit },
}

fn on_socket_rcv(msg: Message) -> anyhow::Result<WSUserMessage> {
    let msg_str = match msg {
        Message::Text(s) => s,
        _ => return Err(anyhow!("Recieved msg is not valid: {:?}", msg)),
    };
    let msg_str = msg_str.as_str();

    let msg = serde_json::from_str::<WSUserMessage>(msg_str)
        .map_err(|e| anyhow!("Recieved msg is not valid: {e}"))?;

    Ok(msg)
}

async fn send_offer(
    pc: Arc<RTCPeerConnection>,
    sender: &mut UnboundedSender<Message>,
) -> anyhow::Result<()> {
    let offer = pc.create_offer(None).await?;
    let offer_json = serde_json::to_string(&offer)?;
    pc.set_local_description(offer).await?;
    sender.send(offer_json.into())?;

    Ok(())
}

async fn answer_offer(
    pc: Arc<RTCPeerConnection>,
    sender: &mut UnboundedSender<Message>,
    offer: RTCSessionDescription,
) -> anyhow::Result<()> {
    pc.set_remote_description(offer).await?;

    let answer = pc.create_answer(None).await?;
    let answer_json = serde_json::to_string(&answer)?;
    pc.set_local_description(answer).await?;
    sender.send(answer_json.into())?;

    Ok(())
}

async fn accept_answer(
    pc: Arc<RTCPeerConnection>,
    answer: RTCSessionDescription,
) -> anyhow::Result<()> {
    pc.set_remote_description(answer).await?;
    Ok(())
}

async fn add_candidate(
    pc: Arc<RTCPeerConnection>,
    candidate: RTCIceCandidateInit,
) -> anyhow::Result<()> {
    pc.add_ice_candidate(candidate).await?;
    Ok(())
}

pub async fn handle_upgrade(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
    //Add claims
) -> impl IntoResponse {
    ws.on_upgrade(move |ws| {
        handle_ws(
            ws,
            state,
            Claims {
                exp: 0,
                user_id: "a".to_string(),
            },
        )
    })
}

async fn handle_ws(ws: WebSocket, state: Arc<AppState>, claims: Claims) {
    let user_id = claims.user_id;

    let (ws_tx, mut ws_rx) = ws.split();

    let (ws_sender, ws_receiver) = unbounded_channel();
    let ws_stream = UnboundedReceiverStream::new(ws_receiver);

    tokio::spawn(ws_stream.map(Ok).forward(ws_tx));

    //Cambiar esto para que sea algo que envie el usuario
    let room = state.rooms.entry("1234".to_owned()).or_default();

    let mut media_engine: MediaEngine = MediaEngine::default();
    media_engine.register_default_codecs().unwrap(); //TODO! Do something with the error :D

    let mut registry = Registry::new();
    registry = register_default_interceptors(registry, &mut media_engine).unwrap(); //TODO! Do something with the error :D

    let api = APIBuilder::new()
        .with_media_engine(media_engine)
        .with_interceptor_registry(registry)
        .build();

    let config = RTCConfiguration::default();

    let peer_connection = Arc::new(api.new_peer_connection(config).await.unwrap()); //TODO! Do something with the error :D

    peer_connection
        .add_transceiver_from_kind(Audio, None)
        .await
        .unwrap(); //TODO! Do something with the error :D
    peer_connection
        .add_transceiver_from_kind(Video, None)
        .await
        .unwrap(); //TODO! Do something with the error :D

    {
        let user_id = user_id.clone();
        peer_connection.on_peer_connection_state_change(Box::new(move |state| {
            debug!("RTCPeerConnection state changed for: {user_id} to: {state:?}");
            Box::pin(async {})
        }));
    }

    let ws_tx = ws_sender.clone();
    peer_connection.on_ice_candidate(Box::new(move |candidate| {
        trace!("New ice candidate: {candidate:?}");
        let Some(candidate) = candidate else {
            return Box::pin(async {});
        };
        let ws_tx = ws_tx.clone();

        Box::pin(async move {
            let candidate_json = serde_json::to_string(&candidate).unwrap();
            ws_tx.send(candidate_json.into()).unwrap();
        })
    }));

    let ws_tx = ws_sender.clone();
    let pc = peer_connection.clone();

    peer_connection.on_negotiation_needed(Box::new(move || {
        if pc.connection_state() != RTCPeerConnectionState::Connected {
            return Box::pin(async move {});
        }
        let mut ws_tx = ws_tx.clone();
        let pc = pc.clone();

        Box::pin(async move {
            send_offer(pc.clone(), &mut ws_tx).await.unwrap(); //TODO! Do something with the error :D
        })
    }));

    {
        let bc_tx = room.broadcast_tx.clone();
        let user_id = user_id.clone();
        peer_connection.on_track(Box::new(move |track: Arc<TrackRemote>, _, _| {
            debug!("On track fired for {:?}", track.kind());
            let bc_tx = bc_tx.clone();
            let user_id = user_id.clone();
            //TODO! Añadir check para solo video y audio quiza xd

            Box::pin(async move {
                tokio::spawn(async move {
                    while let Ok((pckg, _)) = track.read_rtp().await {
                        //TODO! Do something with the error :D
                        let identifier = TrackIdentifier {
                            username: user_id.clone(),
                            track_type: track.kind().to_string(),
                        };

                        let packet = UserPacket {
                            identifier,
                            packet: pckg,
                        };
                        bc_tx.send(packet).unwrap(); //TODO! Do something with the error :Dssss
                    }
                });
            })
        }));
    }

    let mut ws_tx = ws_sender.clone();
    let pc = peer_connection.clone();

    tokio::spawn(async move {
        loop {
            let Some(Ok(msg)) = ws_rx.next().await else {
                continue; //TODO! Do something with the error :D
            };

            let Ok(msg) = on_socket_rcv(msg) else {
                break;
            }; //TODO! Do something with the error :D

            match msg {
                WSUserMessage::RTCOffer { offer } => {
                    answer_offer(pc.clone(), &mut ws_tx, offer).await.unwrap(); //TODO! Do something with the error :D
                }
                WSUserMessage::RTCAnswer { answer } => {
                    accept_answer(pc.clone(), answer).await.unwrap() //TODO! Do something with the error :D
                }
                WSUserMessage::RTCCandidate { candidate } => {
                    add_candidate(pc.clone(), candidate).await.unwrap(); //TODO! Do something with the error :D
                }
            }
        }
    });

    let track_map: Arc<DashMap<TrackIdentifier, Arc<TrackLocalStaticRTP>, _>> =
        Arc::new(DashMap::new());
    {
        let track_map = track_map.clone();
        let mut bc_rx = room.broadcast_tx.subscribe();
        tokio::spawn(async move {
            while let Ok(packet) = bc_rx.recv().await {
                let Some(track) = track_map.get(&packet.identifier) else {
                    continue;
                };

                if packet.identifier.username == user_id {
                    continue;
                }

                track.write_rtp(&packet.packet).await.unwrap(); //TODO! Do something with the error :D
            }
        });
    }

    let mut info_rx = room.info_tx.subscribe();
    tokio::spawn(async move {
        while let Ok(info) = info_rx.recv().await {
            match info {
                RoomEvent::TrackCreated {
                    username,
                    track_type,
                    codec,
                } => {
                    let local_track = Arc::new(TrackLocalStaticRTP::new(
                        codec,
                        track_type.clone(),
                        username.clone(),
                    ));
                    peer_connection
                        .add_track(local_track.clone())
                        .await
                        .unwrap(); //TODO! Do something with the error :D

                    let identifier = TrackIdentifier {
                        username,
                        track_type,
                    };

                    track_map.insert(identifier, local_track).unwrap(); //TODO! Do something with the error :D
                }
            }
        }
    });
}
