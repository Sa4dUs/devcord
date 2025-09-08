use std::{collections::HashSet, sync::Arc, time::Duration, vec};

use anyhow::Result;
use axum::extract::ws::Message;
use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tokio::sync::{broadcast, mpsc};
use tracing::{debug, info};
use webrtc::{
    api::{
        APIBuilder, interceptor_registry::register_default_interceptors, media_engine::MediaEngine,
    },
    ice_transport::{ice_candidate::RTCIceCandidateInit, ice_server::RTCIceServer},
    interceptor::registry::Registry,
    peer_connection::{
        RTCPeerConnection, configuration::RTCConfiguration,
        peer_connection_state::RTCPeerConnectionState,
        sdp::session_description::RTCSessionDescription, signaling_state::RTCSignalingState,
    },
    rtp::packet::Packet,
    rtp_transceiver::{
        rtp_codec::RTPCodecType::{Audio, Video},
        rtp_sender::RTCRtpSender,
    },
    track::{
        track_local::{TrackLocalWriter, track_local_static_rtp::TrackLocalStaticRTP},
        track_remote::TrackRemote,
    },
};

use crate::appstate::{AppStateMessage, RoomID};

type UserID = Arc<String>;
type TrackMap = Arc<DashMap<PacketIdentifier, TrackInfo>>;
type ActiveUsersMap = Arc<DashMap<UserID, UserInfo>>;

#[derive(Error, Debug, Clone, Copy)]
pub enum Error {
    #[error("User is not allowed in this room")]
    UserNotAllowedInRoom,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum WSFromUserMessage {
    RTCOffer { offer: RTCSessionDescription },
    RTCAnswer { answer: RTCSessionDescription },
    RTCCandidate { candidate: RTCIceCandidateInit },
    Disconected { user_id: UserID },
    ConnectToRoom { room_id: String },
}

pub enum WSInnerUserMessage {
    Message(Message),
    Close,
}

#[derive(Debug, Clone)]
pub enum TrackPacket {
    UserPacket(Arc<UserPacket>),
    //Opened(Arc<PacketIdentifier>),
    Closed(Arc<PacketIdentifier>),
    //UserLeft(Arc<UserID>),
}

#[derive(Debug)]
pub enum RoomInfo {
    PCStateChanged {
        user_id: UserID,
        state: RTCPeerConnectionState,
    },
    TrackCreated {
        user_id: UserID,
        track: Arc<TrackRemote>,
    },
    TrackRemoved {
        identifier: Arc<PacketIdentifier>,
    },
    UserDisconected {
        user_id: UserID,
    },
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct PacketIdentifier {
    sender: UserID,
    track_id: String,
}

#[derive(Debug)]
pub struct UserPacket {
    identifier: Arc<PacketIdentifier>,
    packet: Packet,
}

#[derive(Debug)]
pub struct UserInfo {
    user_channel_sender: mpsc::Sender<WSInnerUserMessage>,
    //user_channel_receiver: mpsc::Receiver<WSFromUserMessage>,
    connection: Arc<RTCPeerConnection>,
    other_tracks: TrackMap,
    user_tracks: Arc<DashMap<String, Arc<TrackRemote>>>,
}

impl UserInfo {
    pub fn new(
        user_channel_sender: mpsc::Sender<WSInnerUserMessage>,
        //user_channel_receiver: mpsc::Receiver<WSFromUserMessage>,
        connection: Arc<RTCPeerConnection>,
    ) -> Self {
        UserInfo {
            user_channel_sender,
            //user_channel_receiver,
            connection,
            other_tracks: Default::default(),
            user_tracks: Default::default(),
        }
    }
}

#[derive(Debug)]
pub struct TrackInfo {
    track: Arc<TrackLocalStaticRTP>,
    sender: Arc<RTCRtpSender>,
}

#[derive(Debug)]
pub struct Room {
    allowed_users: HashSet<String>,
    active_users: ActiveUsersMap,
    broadcast_sender: broadcast::Sender<TrackPacket>,
    info_sender: mpsc::Sender<RoomInfo>,
}

impl Room {
    pub async fn new(
        allowed_users: Vec<String>,
        sender: mpsc::Sender<AppStateMessage>,
        id: RoomID,
    ) -> Room {
        let mut allowed_users_set = HashSet::default();
        for user in allowed_users {
            allowed_users_set.insert(user);
        }

        let (broadcast_sender, _) = broadcast::channel(10);
        let (info_sender, info_receiver) = mpsc::channel(10);

        let active_users: ActiveUsersMap = Default::default();

        create_room_controller_thread(
            broadcast_sender.clone(),
            info_receiver,
            active_users.clone(),
            sender,
            id,
        );

        Room {
            allowed_users: allowed_users_set,
            active_users,
            broadcast_sender,
            info_sender,
        }
    }

    pub async fn new_user(
        &mut self,
        user_id: &str,
    ) -> Result<(
        mpsc::Sender<WSFromUserMessage>,
        mpsc::Receiver<WSInnerUserMessage>,
    )> {
        if !self.allowed_users.contains(user_id)
            || self.active_users.contains_key(&user_id.to_string())
        {
            return Err(Error::UserNotAllowedInRoom.into());
        }

        let (inner_tx, ws_rx) = mpsc::channel(10);
        let (ws_tx, inner_rx) = mpsc::channel(10);
        let pc = get_peer_conn().await.unwrap(); //TODO! do something with this, but if it fails, F
        let pc = Arc::new(pc);
        let user_id = Arc::new(user_id.to_string());
        setup_peer_conn(
            user_id.clone(),
            pc.clone(),
            self.info_sender.clone(),
            inner_tx.clone(),
            self.broadcast_sender.clone(),
        );

        let info = UserInfo::new(inner_tx, pc.clone());

        create_ws_listener_thread(pc, inner_rx, self.info_sender.clone());
        create_transmiter_thread(
            user_id.clone(),
            info.other_tracks.clone(),
            self.broadcast_sender.subscribe(),
        );

        let info = self.add_existing_tracks_to_user(info).await;
        self.active_users.insert(user_id, info); //TODO! Shouldnt need to check if already exists but should just in case (Reminder)
        Ok((ws_tx, ws_rx))
    }

    async fn add_existing_tracks_to_user(&self, user_info: UserInfo) -> UserInfo {
        for value in self.active_users.iter() {
            let user_id = value.key();
            for track in value.value().user_tracks.iter() {
                add_remote_track(track.clone(), &user_id, &user_info).await;
            }
        }

        user_info
    }
}

fn create_room_controller_thread(
    broadcast_sender: broadcast::Sender<TrackPacket>,
    mut info_receiver: mpsc::Receiver<RoomInfo>,
    active_users: ActiveUsersMap,
    controller_sender: mpsc::Sender<AppStateMessage>,
    id: RoomID,
) {
    tokio::spawn(async move {
        while let Some(info) = info_receiver.recv().await {
            debug!("Recieved room info: {info:?}");
            match info {
                RoomInfo::PCStateChanged { user_id, state } => {
                    info!("State changed for {} to {}", user_id, state)
                }
                RoomInfo::TrackCreated {
                    user_id: original_id,
                    track,
                } => {
                    for pair in active_users.iter() {
                        let user_info = pair.value();
                        let user_id = pair.key();

                        if *user_id == original_id {
                            user_info.user_tracks.insert(track.id(), track.clone());
                            continue;
                        }

                        add_remote_track(track.clone(), &original_id, user_info).await;
                    }
                }
                RoomInfo::TrackRemoved { identifier } => {
                    for value in active_users.iter() {
                        let user = value.value();
                        if *value.key() == identifier.sender {
                            let removed = user.user_tracks.remove(&identifier.track_id);
                            debug!("Track removed, original user track removed: {:?}", removed);

                            if user.user_tracks.is_empty() {
                                user.user_channel_sender
                                    .send(WSInnerUserMessage::Close)
                                    .await
                                    .unwrap(); //TODO! If this breaks it means the user already disconected and its completelly normal, probs should ignore
                                let msg = TrackPacket::Closed(identifier.clone());
                                broadcast_sender.send(msg).unwrap(); //TODO! If this breaks I would be surprised
                                let removed = active_users.remove(value.key());
                                debug!("Removed active user: {:?}", removed)
                            }
                            continue;
                        }

                        let Some((_, track)) = user.other_tracks.remove(&identifier) else {
                            continue;
                        };

                        let removed = user.connection.remove_track(&track.sender).await.unwrap(); //TODO! This could break if the connection thas weird stuff, cannot let the room thread explode because of that
                        debug!("Track removed, other user track removed: {:?}", removed);
                    }

                    if active_users.is_empty() {
                        info_receiver.close();
                        debug!("ROOM CLOSED");
                        break; //TODO! Do close sequence
                    }
                }
                RoomInfo::UserDisconected { user_id } => {
                    {
                        let Some(user_info) = active_users.get(&user_id) else {
                            continue;
                        };

                        debug!("USER DISCONECTED INFO: {:?}", user_info);
                        debug!("USER INFO TRACKS: {:?}", user_info.user_tracks);

                        for track in user_info.user_tracks.iter() {
                            let identifier = PacketIdentifier {
                                sender: user_id.clone(),
                                track_id: track.id(),
                            };
                            debug!("BROADCASTING DESCONNECTION FOR: {:?}", identifier);
                            let msg = TrackPacket::Closed(Arc::new(identifier.clone()));
                            broadcast_sender.send(msg).unwrap(); //TODO!
                            debug!("BROADCASTED DESCONNECTION FOR: {:?}", identifier);
                        }
                        debug!("FINISHED BROADCASTING LOOP");

                        user_info.connection.close().await.unwrap(); //TODO! ?
                    }

                    let removed = active_users.remove(&user_id);
                    debug!("REMOVED USER BECAUSE OF DISCONNECTION: {:?}", removed);
                    debug!("{:?} USERS LEFT", active_users.len());

                    if active_users.is_empty() {
                        info_receiver.close();
                        debug!("ROOM CLOSED");
                        break; //TODO! Do close sequence
                    }
                }
            }
        }

        controller_sender
            .send(AppStateMessage::RoomClosed(id))
            .await
            .unwrap(); //TODO!
        debug!("ROOM SENT TO REMOVE");
    });
}

async fn add_remote_track(track: Arc<TrackRemote>, user_id: &UserID, user_info: &UserInfo) {
    let local_track = Arc::new(TrackLocalStaticRTP::new(
        track.codec().capability,
        track.id(),
        user_id.to_string(),
    ));

    let sender = user_info
        .connection
        .add_track(local_track.clone())
        .await
        .unwrap(); //TODO! Do something 

    let identifier = PacketIdentifier {
        sender: user_id.clone(),
        track_id: track.id(),
    };

    let info = TrackInfo {
        track: local_track,
        sender,
    };

    user_info.other_tracks.insert(identifier, info);
}

fn create_transmiter_thread(
    user_id: UserID,
    tracks: TrackMap,
    mut receiver: broadcast::Receiver<TrackPacket>,
) {
    let user_id = user_id.clone();

    tokio::spawn(async move {
        while let Ok(packet) = receiver.recv().await {
            match packet {
                TrackPacket::UserPacket(user_packet) => {
                    if user_id == user_packet.identifier.sender {
                        //Dont send our own packages
                        continue;
                    }
                    if let Some(track) = tracks.get(&user_packet.identifier) {
                        track.track.write_rtp(&user_packet.packet).await.unwrap(); //TODO! if err probs close track
                    }
                }
                TrackPacket::Closed(packet_identifier) => {
                    if packet_identifier.sender == user_id {
                        break;
                    }
                }
            }
        }
    });
}

fn create_ws_listener_thread(
    pc: Arc<RTCPeerConnection>,
    mut receiver: mpsc::Receiver<WSFromUserMessage>,
    info_sender: mpsc::Sender<RoomInfo>,
) {
    tokio::spawn(async move {
        while let Some(msg) = receiver.recv().await {
            match msg {
                WSFromUserMessage::RTCOffer { offer } => {
                    debug!("WSMSG OFFER");
                    pc.set_remote_description(offer).await.unwrap(); //TODO! This error is important
                }
                WSFromUserMessage::RTCAnswer { answer } => {
                    debug!("WSMSG ANSWER");
                    pc.set_remote_description(answer).await.unwrap(); //TODO! This error is important
                }
                WSFromUserMessage::RTCCandidate { candidate } => {
                    debug!("WSMSG CANDIDATE");
                    if pc.current_remote_description().await.is_none() {
                        continue;
                    }
                    pc.add_ice_candidate(candidate).await.unwrap(); //TODO! This error is important
                }
                WSFromUserMessage::Disconected { user_id } => {
                    debug!("WSMSG DISCONECTED: {:?}", user_id);
                    let msg = RoomInfo::UserDisconected { user_id };
                    info_sender.send(msg).await.unwrap(); //TODO! Probs just ignore
                }
                _ => (),
            }
        }
    });
}

fn setup_peer_conn(
    user_id: UserID,
    pc: Arc<RTCPeerConnection>,
    info_sender: mpsc::Sender<RoomInfo>,
    ws_sender: mpsc::Sender<WSInnerUserMessage>,
    packet_sender: broadcast::Sender<TrackPacket>,
) {
    setup_on_peer_conn_state_change(user_id.clone(), pc.clone(), info_sender.clone());
    setup_on_ice_candidate(pc.clone(), ws_sender.clone());
    setup_on_track(user_id.clone(), pc.clone(), info_sender, packet_sender);
    setup_on_negotiation_needed(pc.clone(), ws_sender.clone());
    setup_on_signaling_state_change(pc.clone(), ws_sender.clone());
}

fn setup_on_peer_conn_state_change(
    user_id: UserID,
    pc: Arc<RTCPeerConnection>,
    sender: mpsc::Sender<RoomInfo>,
) {
    pc.on_peer_connection_state_change(Box::new(move |state| {
        let info = RoomInfo::PCStateChanged {
            user_id: user_id.clone(),
            state,
        };

        let sender = sender.clone();
        Box::pin(async move {
            sender.send(info).await.unwrap(); //TODO handle this error, probs ignore
        })
    }));
}

fn setup_on_ice_candidate(pc: Arc<RTCPeerConnection>, sender: mpsc::Sender<WSInnerUserMessage>) {
    pc.on_ice_candidate(Box::new(move |candidate| {
        let Some(candidate) = candidate else {
            return Box::pin(async {});
        };

        let candidate_str = serde_json::json!({
            "type": "candidate",
            "candidate": candidate.to_json().unwrap()
        })
        .to_string();
        let sender = sender.clone();
        Box::pin(async move {
            sender
                .send(WSInnerUserMessage::Message(candidate_str.into()))
                .await
                .unwrap() //TODO! Probs ignore, if channel is closed this does not trigger
        })
    }));
}

fn setup_on_track(
    user_id: UserID,
    pc: Arc<RTCPeerConnection>,
    info_sender: mpsc::Sender<RoomInfo>,
    broadcast_sender: broadcast::Sender<TrackPacket>,
) {
    pc.on_track(Box::new(move |track, _, _| {
        let info = RoomInfo::TrackCreated {
            user_id: user_id.clone(),
            track: track.clone(),
        };

        let identifier = PacketIdentifier {
            sender: user_id.clone(),
            track_id: track.id(),
        };

        let identifier = Arc::new(identifier);
        let broadcast_sender = broadcast_sender.clone();
        let info_sender = info_sender.clone();

        tokio::spawn(async move {
            info_sender.send(info).await.unwrap(); //TODO!

            debug!("On track started reading");
            while let Ok(Ok((packet, _))) =
                tokio::time::timeout(Duration::from_secs(5), track.read_rtp()).await
            {
                //TODO! Change this timeout for an env or something idk
                let identifier = identifier.clone();
                let packet = UserPacket { identifier, packet };
                let packet = Arc::new(packet);
                let packet = TrackPacket::UserPacket(packet);

                broadcast_sender.send(packet).unwrap();
            }

            debug!("On track finished reading");
            let msg = RoomInfo::TrackRemoved { identifier };
            info_sender.send(msg).await.unwrap(); //TODO! An error here is indeed a problem
        });
        Box::pin(async move {})
    }));
}

fn setup_on_negotiation_needed(
    pc: Arc<RTCPeerConnection>,
    sender: mpsc::Sender<WSInnerUserMessage>,
) {
    let outer_pc = pc.clone();
    outer_pc.on_negotiation_needed(Box::new(move || {
        // if pc.connection_state() != RTCPeerConnectionState::Connected {
        //     return Box::pin(async move {});
        // }

        let pc = pc.clone();
        let sender = sender.clone();
        Box::pin(async move {
            let offer = pc.create_offer(None).await.unwrap(); //Actually do something with this
            let offer_str = serde_json::to_string(&offer).unwrap(); //Should not fail
            pc.set_local_description(offer).await.unwrap(); //Actually do something with this
            let msg = WSInnerUserMessage::Message(offer_str.into());
            sender.send(msg).await.unwrap(); //TODO! This = bad
        })
    }));
}

fn setup_on_signaling_state_change(
    pc: Arc<RTCPeerConnection>,
    sender: mpsc::Sender<WSInnerUserMessage>,
) {
    let outer_pc = pc.clone();
    outer_pc.on_signaling_state_change(Box::new(move |state| {
        if state != RTCSignalingState::HaveRemoteOffer {
            return Box::pin(async move {});
        }

        let pc = pc.clone();
        let sender = sender.clone();

        Box::pin(async move {
            let answer = pc.create_answer(None).await.unwrap(); //TODO Importatnt error
            let answer_str = serde_json::to_string(&answer).unwrap(); //TODO! Idk
            pc.set_local_description(answer).await.unwrap(); //TODO! Important error, should do something about it
            sender
                .send(WSInnerUserMessage::Message(answer_str.into()))
                .await
                .unwrap(); //TODO! Gues user disconected?
        })
    }));
}

async fn get_peer_conn() -> Result<RTCPeerConnection> {
    //TODO! Change this so you can configure rooms and save the config maybe
    let mut media_engine: MediaEngine = MediaEngine::default();
    media_engine.register_default_codecs().unwrap(); //TODO! Do something with the error :D

    let mut registry = Registry::new();
    registry = register_default_interceptors(registry, &mut media_engine)?;

    let api = APIBuilder::new()
        .with_media_engine(media_engine)
        .with_interceptor_registry(registry)
        .build();

    let config = RTCConfiguration {
        ice_servers: vec![RTCIceServer {
            urls: vec!["stun:stun.l.google.com:19302".to_string()],
            ..Default::default()
        }],
        ..Default::default()
    };

    let peer_connection = api.new_peer_connection(config).await?;

    peer_connection
        .add_transceiver_from_kind(Audio, None)
        .await?;
    peer_connection
        .add_transceiver_from_kind(Video, None)
        .await?;

    Ok(peer_connection)
}
