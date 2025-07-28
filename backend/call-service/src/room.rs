use std::collections::HashSet;

use tokio::sync::broadcast::{self, Sender};
use webrtc::{
    rtp::packet::Packet,
    rtp_transceiver::rtp_codec::{RTCRtpCodecCapability, RTCRtpCodecParameters},
};

#[derive(Hash, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct TrackIdentifier {
    pub username: String,
    pub track_type: String,
}

#[derive(Clone, Debug)]
pub struct UserPacket {
    pub identifier: TrackIdentifier,
    pub packet: Packet,
}

#[derive(Clone, Debug)]
pub enum RoomEvent {
    TrackCreated {
        username: String,
        track_type: String,
        codec: RTCRtpCodecCapability,
    },
}

pub struct Room {
    pub broadcast_tx: Sender<UserPacket>,
    pub info_tx: Sender<RoomEvent>,
}

pub struct RoomConfig {
    pub id: String,
    pub allowed: HashSet<String>,
}

impl Default for Room {
    fn default() -> Self {
        let (b_tx, _) = broadcast::channel(100);
        let (i_tx, _) = broadcast::channel(100);
        Self {
            broadcast_tx: b_tx,
            info_tx: i_tx,
        }
    }
}
