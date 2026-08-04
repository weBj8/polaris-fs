#![deny(unsafe_code)]
// allow: SIZE_OK — framing, connection state, and vectors form one protocol core.

use std::collections::VecDeque;

pub const MAX_PACKET_SIZE: u32 = 1_500_000;
pub const META_DL_BLOCK: u32 = 1_000_000;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Packet {
    pub packet_type: u32,
    pub payload: Vec<u8>,
}

impl Packet {
    pub fn new(packet_type: u32, payload: Vec<u8>) -> Self {
        Self {
            packet_type,
            payload,
        }
    }

    pub fn framed(packet_type: u32, payload: &[u8]) -> Vec<u8> {
        let mut frame = Vec::with_capacity(8 + payload.len());
        frame.extend_from_slice(&packet_type.to_be_bytes());
        frame.extend_from_slice(&(payload.len() as u32).to_be_bytes());
        frame.extend_from_slice(payload);
        frame
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FrameError {
    PacketTooLong { packet_type: u32, length: u32 },
}

pub struct FrameDecoder {
    buffered: Vec<u8>,
    packets: VecDeque<Packet>,
}

impl FrameDecoder {
    pub const fn new() -> Self {
        Self {
            buffered: Vec::new(),
            packets: VecDeque::new(),
        }
    }

    pub fn feed(&mut self, bytes: &[u8]) -> Result<(), FrameError> {
        self.buffered.extend_from_slice(bytes);
        let mut consumed = 0;
        while self.buffered.len().saturating_sub(consumed) >= 8 {
            let header = &self.buffered[consumed..consumed + 8];
            let packet_type = u32::from_be_bytes([header[0], header[1], header[2], header[3]]);
            let length = u32::from_be_bytes([header[4], header[5], header[6], header[7]]);
            if length > MAX_PACKET_SIZE {
                return Err(FrameError::PacketTooLong {
                    packet_type,
                    length,
                });
            }
            let frame_len = 8usize + length as usize;
            if self.buffered.len() - consumed < frame_len {
                break;
            }
            let payload = self.buffered[consumed + 8..consumed + frame_len].to_vec();
            self.packets.push_back(Packet::new(packet_type, payload));
            consumed += frame_len;
        }
        if consumed > 0 {
            self.buffered.drain(..consumed);
        }
        Ok(())
    }

    pub fn pop(&mut self) -> Option<Packet> {
        self.packets.pop_front()
    }

    pub fn is_empty(&self) -> bool {
        self.packets.is_empty()
    }

    pub fn clear(&mut self) {
        self.buffered.clear();
        self.packets.clear();
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum Mode {
    Free = 0,
    Connecting = 1,
    Data = 2,
    Kill = 3,
}

impl Mode {
    pub const fn label(self) -> &'static [u8] {
        match self {
            Self::Free => b"NOT CONNECTED\0",
            Self::Connecting => b"CONNECTING IN PROGRESS\0",
            Self::Data => b"CONNECTED\0",
            Self::Kill => b"DISCONNECTING\0",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProtocolError {
    WrongSize,
    UnknownCommand,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Control {
    Handled,
    Dispatch,
}

pub struct OutPacket {
    pub bytes: Vec<u8>,
    pub sent: usize,
}

pub struct Connection {
    pub(super) mode: Mode,
    pub(super) timeout: u16,
    pub decoder: FrameDecoder,
    pub output: VecDeque<OutPacket>,
    pub input_end: bool,
    pub last_read: f64,
    pub last_write: f64,
    pub connect_time: f64,
}

impl Connection {
    pub const fn new(timeout: u16) -> Self {
        Self {
            mode: Mode::Free,
            timeout,
            decoder: FrameDecoder::new(),
            output: VecDeque::new(),
            input_end: false,
            last_read: 0.0,
            last_write: 0.0,
            connect_time: 0.0,
        }
    }

    pub fn connected(&mut self, now: f64, last_log_version: u64) {
        self.mode = Mode::Data;
        self.last_read = now;
        self.last_write = now;
        self.input_end = false;
        self.decoder.clear();
        self.output.clear();
        self.queue(register_packet(self.timeout, last_log_version));
    }

    pub fn queue(&mut self, bytes: Vec<u8>) {
        self.output.push_back(OutPacket { bytes, sent: 0 });
    }

    pub fn queue_packet(&mut self, packet_type: u32, payload: &[u8]) {
        self.queue(Packet::framed(packet_type, payload));
    }

    pub fn apply_control(
        &mut self,
        packet_type: u32,
        payload: &[u8],
    ) -> Result<Control, ProtocolError> {
        let result = match packet_type {
            0..=2 => Ok(Control::Handled),
            5 if payload.len() == 2 => {
                self.timeout = u16::from_be_bytes([payload[0], payload[1]]).max(10);
                Ok(Control::Handled)
            }
            5 => Err(ProtocolError::WrongSize),
            52 if payload.len() == 5 => {
                if payload[0] >= 2 {
                    self.mode = Mode::Kill;
                }
                Ok(Control::Handled)
            }
            52 => Err(ProtocolError::WrongSize),
            51 | 61 | 63 => Ok(Control::Dispatch),
            _ => Err(ProtocolError::UnknownCommand),
        };
        if result.is_err() {
            self.mode = Mode::Kill;
        }
        result
    }

    pub fn disconnect(&mut self) {
        self.mode = Mode::Free;
        self.input_end = false;
        self.decoder.clear();
        self.output.clear();
    }
}

pub fn register_packet(timeout: u16, last_log_version: u64) -> Vec<u8> {
    let mut payload = Vec::with_capacity(if last_log_version > 0 { 15 } else { 7 });
    payload.push(if last_log_version > 0 { 2 } else { 1 });
    payload.extend_from_slice(&4u16.to_be_bytes());
    payload.extend_from_slice(&[59, 4]);
    payload.extend_from_slice(&timeout.to_be_bytes());
    if last_log_version > 0 {
        payload.extend_from_slice(&last_log_version.wrapping_add(1).to_be_bytes());
    }
    Packet::framed(50, &payload)
}

pub struct DownloadBlock<'a> {
    pub offset: u64,
    pub crc: u32,
    pub data: &'a [u8],
}

pub fn parse_download_block(payload: &[u8]) -> Result<DownloadBlock<'_>, ProtocolError> {
    let payload_len = payload.len();
    let (offset, payload) = payload
        .split_first_chunk::<8>()
        .ok_or(ProtocolError::WrongSize)?;
    let (length, payload) = payload
        .split_first_chunk::<4>()
        .ok_or(ProtocolError::WrongSize)?;
    let (crc, data) = payload
        .split_first_chunk::<4>()
        .ok_or(ProtocolError::WrongSize)?;
    if (u32::from_be_bytes(*length) as usize).wrapping_add(16) != payload_len {
        return Err(ProtocolError::WrongSize);
    }
    Ok(DownloadBlock {
        offset: u64::from_be_bytes(*offset),
        crc: u32::from_be_bytes(*crc),
        data,
    })
}

pub fn clamp_config(timeout: u32, back_logs: u32, meta_frequency: u32) -> (u16, u32, u32) {
    let timeout = timeout.clamp(10, u16::MAX as u32) as u16;
    let back_logs = back_logs.clamp(5, 10_000);
    let meta_frequency = meta_frequency.min(back_logs / 2);
    (timeout, back_logs, meta_frequency)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn register_vectors_match_moosefs_4592() {
        assert_eq!(
            register_packet(10, 0),
            [0, 0, 0, 50, 0, 0, 0, 7, 1, 0, 4, 59, 4, 0, 10]
        );
        assert_eq!(
            register_packet(10, 41),
            [
                0, 0, 0, 50, 0, 0, 0, 15, 2, 0, 4, 59, 4, 0, 10, 0, 0, 0, 0, 0, 0, 0, 42,
            ]
        );
    }

    #[test]
    fn decoder_accepts_fragmented_header_and_payload() {
        let frame = [0, 0, 0, 5, 0, 0, 0, 2, 0, 9];
        let mut decoder = FrameDecoder::new();
        for byte in frame {
            decoder.feed(&[byte]).unwrap();
        }
        assert_eq!(decoder.pop(), Some(Packet::new(5, vec![0, 9])));
    }

    #[test]
    fn decoder_rejects_oversized_packet_without_allocating_payload() {
        let mut decoder = FrameDecoder::new();
        assert_eq!(
            decoder.feed(&[0, 0, 0, 63, 0, 22, 227, 97]),
            Err(FrameError::PacketTooLong {
                packet_type: 63,
                length: 1_500_001,
            })
        );
        assert!(decoder.pop().is_none());
    }

    #[test]
    fn control_packets_preserve_kill_and_timeout_transitions() {
        let mut connection = Connection::new(20);
        connection.connected(1.0, 0);
        assert_eq!(connection.apply_control(5, &[0, 3]), Ok(Control::Handled));
        assert_eq!(connection.timeout, 10);
        assert_eq!(
            connection.apply_control(52, &[2, 0, 0, 0, 0]),
            Ok(Control::Handled)
        );
        assert_eq!(connection.mode, Mode::Kill);
    }

    #[test]
    fn malformed_control_packet_kills_connection() {
        let mut connection = Connection::new(10);
        connection.connected(1.0, 0);
        assert_eq!(
            connection.apply_control(5, &[0]),
            Err(ProtocolError::WrongSize)
        );
        assert_eq!(connection.mode, Mode::Kill);
    }
}
