use futures_util::StreamExt as _;

pub fn crc16(data: &[u8]) -> u16 {
    static TABLE: [u16; 256] = [
        0x0000, 0xc0c1, 0xc181, 0x0140, 0xc301, 0x03c0, 0x0280, 0xc241, 0xc601, 0x06c0, 0x0780,
        0xc741, 0x0500, 0xc5c1, 0xc481, 0x0440, 0xcc01, 0x0cc0, 0x0d80, 0xcd41, 0x0f00, 0xcfc1,
        0xce81, 0x0e40, 0x0a00, 0xcac1, 0xcb81, 0x0b40, 0xc901, 0x09c0, 0x0880, 0xc841, 0xd801,
        0x18c0, 0x1980, 0xd941, 0x1b00, 0xdbc1, 0xda81, 0x1a40, 0x1e00, 0xdec1, 0xdf81, 0x1f40,
        0xdd01, 0x1dc0, 0x1c80, 0xdc41, 0x1400, 0xd4c1, 0xd581, 0x1540, 0xd701, 0x17c0, 0x1680,
        0xd641, 0xd201, 0x12c0, 0x1380, 0xd341, 0x1100, 0xd1c1, 0xd081, 0x1040, 0xf001, 0x30c0,
        0x3180, 0xf141, 0x3300, 0xf3c1, 0xf281, 0x3240, 0x3600, 0xf6c1, 0xf781, 0x3740, 0xf501,
        0x35c0, 0x3480, 0xf441, 0x3c00, 0xfcc1, 0xfd81, 0x3d40, 0xff01, 0x3fc0, 0x3e80, 0xfe41,
        0xfa01, 0x3ac0, 0x3b80, 0xfb41, 0x3900, 0xf9c1, 0xf881, 0x3840, 0x2800, 0xe8c1, 0xe981,
        0x2940, 0xeb01, 0x2bc0, 0x2a80, 0xea41, 0xee01, 0x2ec0, 0x2f80, 0xef41, 0x2d00, 0xedc1,
        0xec81, 0x2c40, 0xe401, 0x24c0, 0x2580, 0xe541, 0x2700, 0xe7c1, 0xe681, 0x2640, 0x2200,
        0xe2c1, 0xe381, 0x2340, 0xe101, 0x21c0, 0x2080, 0xe041, 0xa001, 0x60c0, 0x6180, 0xa141,
        0x6300, 0xa3c1, 0xa281, 0x6240, 0x6600, 0xa6c1, 0xa781, 0x6740, 0xa501, 0x65c0, 0x6480,
        0xa441, 0x6c00, 0xacc1, 0xad81, 0x6d40, 0xaf01, 0x6fc0, 0x6e80, 0xae41, 0xaa01, 0x6ac0,
        0x6b80, 0xab41, 0x6900, 0xa9c1, 0xa881, 0x6840, 0x7800, 0xb8c1, 0xb981, 0x7940, 0xbb01,
        0x7bc0, 0x7a80, 0xba41, 0xbe01, 0x7ec0, 0x7f80, 0xbf41, 0x7d00, 0xbdc1, 0xbc81, 0x7c40,
        0xb401, 0x74c0, 0x7580, 0xb541, 0x7700, 0xb7c1, 0xb681, 0x7640, 0x7200, 0xb2c1, 0xb381,
        0x7340, 0xb101, 0x71c0, 0x7080, 0xb041, 0x5000, 0x90c1, 0x9181, 0x5140, 0x9301, 0x53c0,
        0x5280, 0x9241, 0x9601, 0x56c0, 0x5780, 0x9741, 0x5500, 0x95c1, 0x9481, 0x5440, 0x9c01,
        0x5cc0, 0x5d80, 0x9d41, 0x5f00, 0x9fc1, 0x9e81, 0x5e40, 0x5a00, 0x9ac1, 0x9b81, 0x5b40,
        0x9901, 0x59c0, 0x5880, 0x9841, 0x8801, 0x48c0, 0x4980, 0x8941, 0x4b00, 0x8bc1, 0x8a81,
        0x4a40, 0x4e00, 0x8ec1, 0x8f81, 0x4f40, 0x8d01, 0x4dc0, 0x4c80, 0x8c41, 0x4400, 0x84c1,
        0x8581, 0x4540, 0x8701, 0x47c0, 0x4680, 0x8641, 0x8201, 0x42c0, 0x4380, 0x8341, 0x4100,
        0x81c1, 0x8081, 0x4040,
    ];

    let mut crc = 0xffff;

    for i in data.iter().copied() {
        let xor = i ^ crc as u8;
        crc >>= 8;
        crc ^= TABLE[xor as usize];
    }

    crc
}

use aes::{
    Aes128, Block,
    cipher::{BlockDecrypt, BlockEncrypt, KeyInit},
};
use btleplug::api::{Characteristic, Peripheral as _, WriteType};
use btleplug::platform::Peripheral;

pub struct Cube {
    perip: Peripheral,
    fff6: Characteristic,
    cipher: Aes128,
}

impl Cube {
    pub fn new(perip: Peripheral, fff6: Characteristic) -> Self {
        Self {
            perip,
            fff6,
            cipher: Aes128::new(
                &[
                    87, 177, 249, 171, 205, 90, 232, 167, 156, 185, 140, 231, 87, 140, 81, 8,
                ]
                .into(),
            ),
        }
    }

    /// Given the bytes of an app->cube command:
    /// - prefixes with `0xfe` and the length;
    /// - computes the checksum and appends it to the end;
    /// - adds zero-padding;
    /// - encrypts the message;
    /// - writes it to the fff6 characteristic
    async fn write_cmd_inner_bytes(&mut self, bytes: &[u8]) {
        // +2 for checksum, +2 for fe/length prefix
        let cmdlen = bytes.len() + 2 + 2;
        let npad = if cmdlen % 16 == 0 {
            0
        } else {
            16 - (cmdlen % 16)
        };
        let total_len = npad + cmdlen;
        assert!(total_len % 16 == 0);

        let mut bytes = {
            let mut v = Vec::<u8>::with_capacity(total_len);
            v.push(0xfe);
            v.push(cmdlen.try_into().expect("Packet len > 255"));
            v.extend_from_slice(bytes);
            v.extend_from_slice(&crc16(&v).to_le_bytes());
            v.resize(total_len, 0);
            v
        };

        // encrypt bytes
        for mut block in bytes.chunks_mut(16).map(Block::from_mut_slice) {
            self.cipher.encrypt_block(&mut block);
        }

        self.perip
            .write(&self.fff6, &bytes, WriteType::WithoutResponse)
            .await
            .unwrap();
    }
}

use libfmc::Spy;

pub async fn run_protocol(mut cube: Cube) {
    cube.perip.subscribe(&cube.fff6).await.unwrap();

    // send App Hello
    cube.write_cmd_inner_bytes(&make_app_hello(
        BDAddr::from_str_delim("CC:A3:00:01:2B:74").unwrap(),
    ))
    .await;

    println!("hello");
    let mut notifs = cube.perip.notifications().await.unwrap();
    let mut spy = Spy::new();
    let mut prev_ts = 0;
    println!("hello");
    while let Some(n) = notifs.next().await {
        println!("bruh");
        assert!(n.uuid == cube.fff6.uuid);
        let mut bytes = n.value;
        assert!(bytes.len() % 16 == 0);

        for mut block in bytes.chunks_mut(16).map(Block::from_mut_slice) {
            cube.cipher.decrypt_block(&mut block);
        }

        let msg = parse_c2a_message(&bytes).unwrap();

        if let C2aBody::StateChange(sc) = msg.body() {
            let cur = vec![(sc.mov, msg.timestamp())];
            let mut history = sc
                .history
                .iter()
                .chain(&cur)
                .filter(|(_, ts)| *ts > prev_ts)
                .collect::<Vec<_>>();
            history.sort_by_key(|(_, ts)| ts);

            if let Some(ts) = history.last().map(|x| x.1) {
                prev_ts = ts;
                for (mv, _) in history {
                    spy.handle(*mv);
                }
            }
        }

        if let Some(pkt) = msg.make_ack() {
            cube.write_cmd_inner_bytes(pkt).await;
        }
    }

    println!("Disconnecting...");
    cube.perip.disconnect().await.unwrap();
    println!("Disconnected.");
}

use btleplug::api::{Central, CentralEvent, Manager as _, bleuuid::uuid_from_u16};
use btleplug::platform::Manager;
use std::io::{self, Write};

use anyhow::{Result, anyhow, bail};
use btleplug::api::BDAddr;
use thiserror::Error;

#[derive(Debug)]
enum Opcode {
    CubeHello,
    StateChange,
    SyncConfirmation,
}

impl Opcode {
    fn from_u8(x: u8) -> Result<Self> {
        Ok(match x {
            0x2 => Self::CubeHello,
            0x3 => Self::StateChange,
            0x4 => Self::SyncConfirmation,
            _ => bail!(ParseError::BadOpcode { bad_opcode: x }),
        })
    }
}

/// A cube->app message.
#[derive(Debug)]
pub struct C2aMessage<'a> {
    /// Reference to bytes 3-7 for use in ACKs
    ack_head: &'a [u8],
    millis_timestamp: u32,
    body: C2aBody,
}

impl<'a> C2aMessage<'a> {
    fn needs_ack(&self) -> bool {
        true
    }

    /// Returns `Some(ack)` if this message needs to be ACKed;
    /// returns `None` if it doesn't need an ACK.
    // TODO: make structs for app->cube messages instead of returning &[u8] here
    pub fn make_ack(&self) -> Option<&'a [u8]> {
        if self.needs_ack() {
            Some(self.ack_head)
        } else {
            None
        }
    }

    /// Get the timestamp in milliseconds
    pub fn timestamp(&self) -> u32 {
        self.millis_timestamp
    }

    pub fn body(&self) -> &C2aBody {
        &self.body
    }
}

#[derive(Debug)]
pub struct CubeState {
    facelets: [u8; 54],
}

impl CubeState {
    pub fn from_raw(raw: &[u8]) -> Self {
        Self {
            facelets: raw
                .iter()
                .flat_map(|&x| [x & 0xf, (x & 0xF0) >> 4])
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
        }
    }
}

/// The "body" of a cube->app message is the decrypted contents
/// minus the `0xfe` prefix, length, opcode, padding, and checksum.
#[derive(Debug)]
pub enum C2aBody {
    CubeHello(CubeHello),
    StateChange(StateChange),
}

#[derive(Debug)]
pub struct CubeHello {
    pub state: CubeState,
}

fn move_from_byte(x: u8) -> Result<libfmc::Move> {
    use libfmc::Move;
    Ok(match x {
        1 => Move::L3,
        2 => Move::L,
        3 => Move::R3,
        4 => Move::R,
        5 => Move::D3,
        6 => Move::D,
        7 => Move::U3,
        8 => Move::U,
        9 => Move::F3,
        10 => Move::F,
        11 => Move::B3,
        12 => Move::B,
        _ => bail!(ParseError::BadTurn { turn: x }),
    })
}

#[derive(Debug)]
pub struct StateChange {
    pub state: CubeState,
    pub mov: libfmc::Move,
    pub history: Vec<(libfmc::Move, u32)>,
    pub battery: u8,
}

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Missing magic `0xfe` byte at start of message")]
    BadMagic,
    #[error("Expected message to be longer (tried to index outside the message)")]
    TooShort,
    #[error("Invalid checksum")]
    FailedChecksum,
    #[error("Invalid opcode (got {bad_opcode})")]
    BadOpcode { bad_opcode: u8 },
    #[error("Invalid turn ({turn} is not a valid move)")]
    BadTurn { turn: u8 },
}

struct Parser<'a> {
    bytes: &'a [u8],
}

impl<'a> Parser<'a> {
    fn get_bytes(&self, idx: usize, n: usize) -> Result<&'a [u8]> {
        self.bytes
            .get(idx..idx + n)
            .ok_or(anyhow!(ParseError::TooShort))
    }

    fn trim_padding(&mut self, message_length: u8) {
        self.bytes = &self.bytes[..message_length as usize];
    }

    fn get_u8(&self, idx: usize) -> Result<u8> {
        self.bytes
            .get(idx)
            .copied()
            .ok_or(anyhow!(ParseError::TooShort))
    }

    fn get_u16(&self, idx: usize) -> Result<u16> {
        Ok(u16::from_le_bytes(
            self.get_bytes(idx, 2)?.try_into().unwrap(),
        ))
    }

    fn get_u32_be(&self, idx: usize) -> Result<u32> {
        Ok(u32::from_be_bytes(
            self.get_bytes(idx, 4)?.try_into().unwrap(),
        ))
    }
}

pub fn make_app_hello(mac: BDAddr) -> Vec<u8> {
    // fill the 11-byte unknown field with zeros
    let mut v = vec![0; 11];

    let mut mac = mac.into_inner();
    mac.reverse();

    v.extend_from_slice(&mac);

    v
}

/// Given the bytes of an **decrypted** message, parse them into a cube->app message.
pub fn parse_c2a_message(bytes: &[u8]) -> Result<C2aMessage> {
    let mut p = Parser { bytes };

    if p.get_u8(0)? != 0xfe {
        bail!(ParseError::BadMagic);
    }

    let length = p.get_u8(1)?;
    if p.bytes.len() < length as usize {
        bail!(ParseError::TooShort);
    }
    p.trim_padding(length);
    let checksum = p.get_u16(length as usize - 2)?;
    if crc16(p.get_bytes(0, length as usize - 2)?) != checksum {
        bail!(ParseError::FailedChecksum);
    }

    let opcode = Opcode::from_u8(p.get_u8(2)?)?;
    let millis_timestamp = (p.get_u32_be(3)? as f32 / 1.6) as u32;
    let body = match opcode {
        Opcode::CubeHello => {
            let rawstate = p.get_bytes(7, 27)?;

            C2aBody::CubeHello(CubeHello {
                state: CubeState::from_raw(rawstate),
            })
        }
        Opcode::StateChange => {
            let rawstate = p.get_bytes(7, 27)?;
            let turnbyte = p.get_u8(34)?;
            let battery = p.get_u8(35)?;
            let mut history = Vec::new();
            let mut i = 36;
            while i < 91 {
                let ts = (p.get_u32_be(i)? as f32 / 1.6) as u32;
                if let Ok(mv) = move_from_byte(p.get_u8(i + 4)?) {
                    history.push((mv, ts));
                }
                i += 5;
            }

            C2aBody::StateChange(StateChange {
                state: CubeState::from_raw(rawstate),
                mov: move_from_byte(turnbyte)?,
                battery: battery,
                history: history,
            })
        }
        Opcode::SyncConfirmation => {
            todo!()
        }
    };

    assert!(p.bytes.len() >= 7);

    Ok(C2aMessage {
        ack_head: p.get_bytes(2, 5)?,
        millis_timestamp,
        body,
    })
}

async fn async_main() {
    let manager = Manager::new().await.unwrap();
    let central = manager
        .adapters()
        .await
        .unwrap()
        .into_iter()
        .nth(0)
        .unwrap();

    println!("Searching for cube...");
    let mut events = central.events().await.unwrap();
    central.start_scan(Default::default()).await.unwrap();

    let cube_perip;
    loop {
        if let CentralEvent::DeviceDiscovered(id) = events.next().await.unwrap() {
            let peripheral = central.peripheral(&id).await.unwrap();
            if peripheral
                .properties()
                .await
                .unwrap()
                .unwrap()
                .local_name
                .iter()
                .any(|name| name.starts_with("QY-QYSC"))
            {
                cube_perip = peripheral;
                break;
            }
        }
    }

    println!("Connecting...");
    cube_perip.connect().await.unwrap();
    println!("Connected.");
    cube_perip.discover_services().await.unwrap();

    let fff6_chr = cube_perip
        .characteristics()
        .into_iter()
        .find(|c| c.uuid == uuid_from_u16(0xfff6))
        .unwrap();

    let cube = Cube::new(cube_perip, fff6_chr);
    run_protocol(cube).await;
}

fn main() {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            async_main().await;
        });
}
