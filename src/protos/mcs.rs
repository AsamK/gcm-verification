//! Automatically generated rust module for 'mcs.proto' file

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
#![allow(unknown_lints)]
#![allow(clippy)]
#![cfg_attr(rustfmt, rustfmt_skip)]


use std::io::Write;
use std::borrow::Cow;
use quick_protobuf::{MessageRead, MessageWrite, BytesReader, Writer, Result};
use quick_protobuf::sizeofs::*;
use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct HeartbeatPing {
    pub stream_id: Option<i32>,
    pub last_stream_id_received: Option<i32>,
    pub status: Option<i64>,
}

impl<'a> MessageRead<'a> for HeartbeatPing {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.stream_id = Some(r.read_int32(bytes)?),
                Ok(16) => msg.last_stream_id_received = Some(r.read_int32(bytes)?),
                Ok(24) => msg.status = Some(r.read_int64(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for HeartbeatPing {
    fn get_size(&self) -> usize {
        0
        + self.stream_id.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.last_stream_id_received.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.status.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.stream_id { w.write_with_tag(8, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.last_stream_id_received { w.write_with_tag(16, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.status { w.write_with_tag(24, |w| w.write_int64(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct HeartbeatAck {
    pub stream_id: Option<i32>,
    pub last_stream_id_received: Option<i32>,
    pub status: Option<i64>,
}

impl<'a> MessageRead<'a> for HeartbeatAck {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.stream_id = Some(r.read_int32(bytes)?),
                Ok(16) => msg.last_stream_id_received = Some(r.read_int32(bytes)?),
                Ok(24) => msg.status = Some(r.read_int64(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for HeartbeatAck {
    fn get_size(&self) -> usize {
        0
        + self.stream_id.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.last_stream_id_received.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.status.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.stream_id { w.write_with_tag(8, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.last_stream_id_received { w.write_with_tag(16, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.status { w.write_with_tag(24, |w| w.write_int64(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ErrorInfo<'a> {
    pub code: i32,
    pub message: Option<Cow<'a, str>>,
    pub type_pb: Option<Cow<'a, str>>,
    pub extension: Option<Extension<'a>>,
}

impl<'a> MessageRead<'a> for ErrorInfo<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.code = r.read_int32(bytes)?,
                Ok(18) => msg.message = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(26) => msg.type_pb = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(34) => msg.extension = Some(r.read_message::<Extension>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ErrorInfo<'a> {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_varint(*(&self.code) as u64)
        + self.message.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.type_pb.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.extension.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(8, |w| w.write_int32(*&self.code))?;
        if let Some(ref s) = self.message { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.type_pb { w.write_with_tag(26, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.extension { w.write_with_tag(34, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Setting<'a> {
    pub name: Cow<'a, str>,
    pub value: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for Setting<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.name = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.value = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Setting<'a> {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_len((&self.name).len())
        + 1 + sizeof_len((&self.value).len())
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(10, |w| w.write_string(&**&self.name))?;
        w.write_with_tag(18, |w| w.write_string(&**&self.value))?;
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct HeartbeatStat<'a> {
    pub ip: Cow<'a, str>,
    pub timeout: bool,
    pub interval_ms: i32,
}

impl<'a> MessageRead<'a> for HeartbeatStat<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.ip = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(16) => msg.timeout = r.read_bool(bytes)?,
                Ok(24) => msg.interval_ms = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for HeartbeatStat<'a> {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_len((&self.ip).len())
        + 1 + sizeof_varint(*(&self.timeout) as u64)
        + 1 + sizeof_varint(*(&self.interval_ms) as u64)
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(10, |w| w.write_string(&**&self.ip))?;
        w.write_with_tag(16, |w| w.write_bool(*&self.timeout))?;
        w.write_with_tag(24, |w| w.write_int32(*&self.interval_ms))?;
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct HeartbeatConfig<'a> {
    pub upload_stat: Option<bool>,
    pub ip: Option<Cow<'a, str>>,
    pub interval_ms: Option<i32>,
}

impl<'a> MessageRead<'a> for HeartbeatConfig<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.upload_stat = Some(r.read_bool(bytes)?),
                Ok(18) => msg.ip = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(24) => msg.interval_ms = Some(r.read_int32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for HeartbeatConfig<'a> {
    fn get_size(&self) -> usize {
        0
        + self.upload_stat.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.ip.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.interval_ms.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.upload_stat { w.write_with_tag(8, |w| w.write_bool(*s))?; }
        if let Some(ref s) = self.ip { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.interval_ms { w.write_with_tag(24, |w| w.write_int32(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct LoginRequest<'a> {
    pub id: Cow<'a, str>,
    pub domain: Cow<'a, str>,
    pub user: Cow<'a, str>,
    pub resource: Cow<'a, str>,
    pub auth_token: Cow<'a, str>,
    pub device_id: Option<Cow<'a, str>>,
    pub last_rmq_id: Option<i64>,
    pub setting: Vec<Setting<'a>>,
    pub compress: Option<i32>,
    pub received_persistent_id: Vec<Cow<'a, str>>,
    pub include_stream_ids: Option<bool>,
    pub adaptive_heartbeat: Option<bool>,
    pub heartbeat_stat: Option<HeartbeatStat<'a>>,
    pub use_rmq2: Option<bool>,
    pub account_id: Option<i64>,
    pub auth_service: Option<mod_LoginRequest::AuthService>,
    pub network_type: Option<i32>,
    pub status: Option<i64>,
}

impl<'a> MessageRead<'a> for LoginRequest<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.id = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.domain = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(26) => msg.user = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(34) => msg.resource = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(42) => msg.auth_token = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(50) => msg.device_id = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(56) => msg.last_rmq_id = Some(r.read_int64(bytes)?),
                Ok(66) => msg.setting.push(r.read_message::<Setting>(bytes)?),
                Ok(72) => msg.compress = Some(r.read_int32(bytes)?),
                Ok(82) => msg.received_persistent_id.push(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(88) => msg.include_stream_ids = Some(r.read_bool(bytes)?),
                Ok(96) => msg.adaptive_heartbeat = Some(r.read_bool(bytes)?),
                Ok(106) => msg.heartbeat_stat = Some(r.read_message::<HeartbeatStat>(bytes)?),
                Ok(112) => msg.use_rmq2 = Some(r.read_bool(bytes)?),
                Ok(120) => msg.account_id = Some(r.read_int64(bytes)?),
                Ok(128) => msg.auth_service = Some(r.read_enum(bytes)?),
                Ok(136) => msg.network_type = Some(r.read_int32(bytes)?),
                Ok(144) => msg.status = Some(r.read_int64(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for LoginRequest<'a> {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_len((&self.id).len())
        + 1 + sizeof_len((&self.domain).len())
        + 1 + sizeof_len((&self.user).len())
        + 1 + sizeof_len((&self.resource).len())
        + 1 + sizeof_len((&self.auth_token).len())
        + self.device_id.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.last_rmq_id.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.setting.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.compress.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.received_persistent_id.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
        + self.include_stream_ids.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.adaptive_heartbeat.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.heartbeat_stat.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.use_rmq2.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.account_id.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.auth_service.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.network_type.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.status.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(10, |w| w.write_string(&**&self.id))?;
        w.write_with_tag(18, |w| w.write_string(&**&self.domain))?;
        w.write_with_tag(26, |w| w.write_string(&**&self.user))?;
        w.write_with_tag(34, |w| w.write_string(&**&self.resource))?;
        w.write_with_tag(42, |w| w.write_string(&**&self.auth_token))?;
        if let Some(ref s) = self.device_id { w.write_with_tag(50, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.last_rmq_id { w.write_with_tag(56, |w| w.write_int64(*s))?; }
        for s in &self.setting { w.write_with_tag(66, |w| w.write_message(s))?; }
        if let Some(ref s) = self.compress { w.write_with_tag(72, |w| w.write_int32(*s))?; }
        for s in &self.received_persistent_id { w.write_with_tag(82, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.include_stream_ids { w.write_with_tag(88, |w| w.write_bool(*s))?; }
        if let Some(ref s) = self.adaptive_heartbeat { w.write_with_tag(96, |w| w.write_bool(*s))?; }
        if let Some(ref s) = self.heartbeat_stat { w.write_with_tag(106, |w| w.write_message(s))?; }
        if let Some(ref s) = self.use_rmq2 { w.write_with_tag(112, |w| w.write_bool(*s))?; }
        if let Some(ref s) = self.account_id { w.write_with_tag(120, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.auth_service { w.write_with_tag(128, |w| w.write_enum(*s as i32))?; }
        if let Some(ref s) = self.network_type { w.write_with_tag(136, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.status { w.write_with_tag(144, |w| w.write_int64(*s))?; }
        Ok(())
    }
}

pub mod mod_LoginRequest {


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum AuthService {
    ANDROID_ID = 2,
}

impl Default for AuthService {
    fn default() -> Self {
        AuthService::ANDROID_ID
    }
}

impl From<i32> for AuthService {
    fn from(i: i32) -> Self {
        match i {
            2 => AuthService::ANDROID_ID,
            _ => Self::default(),
        }
    }
}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct LoginResponse<'a> {
    pub id: Cow<'a, str>,
    pub jid: Option<Cow<'a, str>>,
    pub error: Option<ErrorInfo<'a>>,
    pub setting: Vec<Setting<'a>>,
    pub stream_id: Option<i32>,
    pub last_stream_id_received: Option<i32>,
    pub heartbeat_config: Option<HeartbeatConfig<'a>>,
    pub server_timestamp: Option<i64>,
}

impl<'a> MessageRead<'a> for LoginResponse<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.id = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.jid = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(26) => msg.error = Some(r.read_message::<ErrorInfo>(bytes)?),
                Ok(34) => msg.setting.push(r.read_message::<Setting>(bytes)?),
                Ok(40) => msg.stream_id = Some(r.read_int32(bytes)?),
                Ok(48) => msg.last_stream_id_received = Some(r.read_int32(bytes)?),
                Ok(58) => msg.heartbeat_config = Some(r.read_message::<HeartbeatConfig>(bytes)?),
                Ok(64) => msg.server_timestamp = Some(r.read_int64(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for LoginResponse<'a> {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_len((&self.id).len())
        + self.jid.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.error.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.setting.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.stream_id.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.last_stream_id_received.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.heartbeat_config.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.server_timestamp.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(10, |w| w.write_string(&**&self.id))?;
        if let Some(ref s) = self.jid { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.error { w.write_with_tag(26, |w| w.write_message(s))?; }
        for s in &self.setting { w.write_with_tag(34, |w| w.write_message(s))?; }
        if let Some(ref s) = self.stream_id { w.write_with_tag(40, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.last_stream_id_received { w.write_with_tag(48, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.heartbeat_config { w.write_with_tag(58, |w| w.write_message(s))?; }
        if let Some(ref s) = self.server_timestamp { w.write_with_tag(64, |w| w.write_int64(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct StreamErrorStanza<'a> {
    pub type_pb: Cow<'a, str>,
    pub text: Option<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for StreamErrorStanza<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.type_pb = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.text = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for StreamErrorStanza<'a> {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_len((&self.type_pb).len())
        + self.text.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(10, |w| w.write_string(&**&self.type_pb))?;
        if let Some(ref s) = self.text { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Close { }

impl<'a> MessageRead<'a> for Close {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for Close { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Extension<'a> {
    pub id: i32,
    pub data: Cow<'a, [u8]>,
}

impl<'a> MessageRead<'a> for Extension<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.id = r.read_int32(bytes)?,
                Ok(18) => msg.data = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Extension<'a> {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_varint(*(&self.id) as u64)
        + 1 + sizeof_len((&self.data).len())
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(8, |w| w.write_int32(*&self.id))?;
        w.write_with_tag(18, |w| w.write_bytes(&**&self.data))?;
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct IqStanza<'a> {
    pub rmq_id: Option<i64>,
    pub type_pb: mod_IqStanza::IqType,
    pub id: Cow<'a, str>,
    pub from: Option<Cow<'a, str>>,
    pub to: Option<Cow<'a, str>>,
    pub error: Option<ErrorInfo<'a>>,
    pub extension: Option<Extension<'a>>,
    pub persistent_id: Option<Cow<'a, str>>,
    pub stream_id: Option<i32>,
    pub last_stream_id_received: Option<i32>,
    pub account_id: Option<i64>,
    pub status: Option<i64>,
}

impl<'a> MessageRead<'a> for IqStanza<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.rmq_id = Some(r.read_int64(bytes)?),
                Ok(16) => msg.type_pb = r.read_enum(bytes)?,
                Ok(26) => msg.id = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(34) => msg.from = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(42) => msg.to = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(50) => msg.error = Some(r.read_message::<ErrorInfo>(bytes)?),
                Ok(58) => msg.extension = Some(r.read_message::<Extension>(bytes)?),
                Ok(66) => msg.persistent_id = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(72) => msg.stream_id = Some(r.read_int32(bytes)?),
                Ok(80) => msg.last_stream_id_received = Some(r.read_int32(bytes)?),
                Ok(88) => msg.account_id = Some(r.read_int64(bytes)?),
                Ok(96) => msg.status = Some(r.read_int64(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for IqStanza<'a> {
    fn get_size(&self) -> usize {
        0
        + self.rmq_id.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + 1 + sizeof_varint(*(&self.type_pb) as u64)
        + 1 + sizeof_len((&self.id).len())
        + self.from.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.to.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.error.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.extension.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.persistent_id.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.stream_id.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.last_stream_id_received.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.account_id.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.status.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.rmq_id { w.write_with_tag(8, |w| w.write_int64(*s))?; }
        w.write_with_tag(16, |w| w.write_enum(*&self.type_pb as i32))?;
        w.write_with_tag(26, |w| w.write_string(&**&self.id))?;
        if let Some(ref s) = self.from { w.write_with_tag(34, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.to { w.write_with_tag(42, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.error { w.write_with_tag(50, |w| w.write_message(s))?; }
        if let Some(ref s) = self.extension { w.write_with_tag(58, |w| w.write_message(s))?; }
        if let Some(ref s) = self.persistent_id { w.write_with_tag(66, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.stream_id { w.write_with_tag(72, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.last_stream_id_received { w.write_with_tag(80, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.account_id { w.write_with_tag(88, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.status { w.write_with_tag(96, |w| w.write_int64(*s))?; }
        Ok(())
    }
}

pub mod mod_IqStanza {


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum IqType {
    GET = 0,
    SET = 1,
    RESULT = 2,
    IQ_ERROR = 3,
}

impl Default for IqType {
    fn default() -> Self {
        IqType::GET
    }
}

impl From<i32> for IqType {
    fn from(i: i32) -> Self {
        match i {
            0 => IqType::GET,
            1 => IqType::SET,
            2 => IqType::RESULT,
            3 => IqType::IQ_ERROR,
            _ => Self::default(),
        }
    }
}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct AppData<'a> {
    pub key: Cow<'a, str>,
    pub value: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for AppData<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.key = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.value = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for AppData<'a> {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_len((&self.key).len())
        + 1 + sizeof_len((&self.value).len())
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(10, |w| w.write_string(&**&self.key))?;
        w.write_with_tag(18, |w| w.write_string(&**&self.value))?;
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct DataMessageStanza<'a> {
    pub rmq_id: Option<i64>,
    pub id: Option<Cow<'a, str>>,
    pub from: Cow<'a, str>,
    pub to: Option<Cow<'a, str>>,
    pub category: Cow<'a, str>,
    pub token: Option<Cow<'a, str>>,
    pub app_data: Vec<AppData<'a>>,
    pub from_trusted_server: Option<bool>,
    pub persistent_id: Option<Cow<'a, str>>,
    pub stream_id: Option<i32>,
    pub last_stream_id_received: Option<i32>,
    pub permission: Option<Cow<'a, str>>,
    pub reg_id: Option<Cow<'a, str>>,
    pub pkg_signature: Option<Cow<'a, str>>,
    pub client_id: Option<Cow<'a, str>>,
    pub device_user_id: Option<i64>,
    pub ttl: Option<i32>,
    pub sent: Option<i64>,
    pub queued: Option<i32>,
    pub status: Option<i64>,
    pub raw_data: Option<Cow<'a, [u8]>>,
    pub delay: Option<i32>,
}

impl<'a> MessageRead<'a> for DataMessageStanza<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.rmq_id = Some(r.read_int64(bytes)?),
                Ok(18) => msg.id = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(26) => msg.from = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(34) => msg.to = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(42) => msg.category = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(50) => msg.token = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(58) => msg.app_data.push(r.read_message::<AppData>(bytes)?),
                Ok(64) => msg.from_trusted_server = Some(r.read_bool(bytes)?),
                Ok(74) => msg.persistent_id = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(80) => msg.stream_id = Some(r.read_int32(bytes)?),
                Ok(88) => msg.last_stream_id_received = Some(r.read_int32(bytes)?),
                Ok(98) => msg.permission = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(106) => msg.reg_id = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(114) => msg.pkg_signature = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(122) => msg.client_id = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(128) => msg.device_user_id = Some(r.read_int64(bytes)?),
                Ok(136) => msg.ttl = Some(r.read_int32(bytes)?),
                Ok(144) => msg.sent = Some(r.read_int64(bytes)?),
                Ok(152) => msg.queued = Some(r.read_int32(bytes)?),
                Ok(160) => msg.status = Some(r.read_int64(bytes)?),
                Ok(170) => msg.raw_data = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(176) => msg.delay = Some(r.read_int32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for DataMessageStanza<'a> {
    fn get_size(&self) -> usize {
        0
        + self.rmq_id.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.id.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + 1 + sizeof_len((&self.from).len())
        + self.to.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + 1 + sizeof_len((&self.category).len())
        + self.token.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.app_data.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.from_trusted_server.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.persistent_id.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.stream_id.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.last_stream_id_received.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.permission.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.reg_id.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.pkg_signature.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.client_id.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.device_user_id.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.ttl.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.sent.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.queued.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.status.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.raw_data.as_ref().map_or(0, |m| 2 + sizeof_len((m).len()))
        + self.delay.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.rmq_id { w.write_with_tag(8, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.id { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        w.write_with_tag(26, |w| w.write_string(&**&self.from))?;
        if let Some(ref s) = self.to { w.write_with_tag(34, |w| w.write_string(&**s))?; }
        w.write_with_tag(42, |w| w.write_string(&**&self.category))?;
        if let Some(ref s) = self.token { w.write_with_tag(50, |w| w.write_string(&**s))?; }
        for s in &self.app_data { w.write_with_tag(58, |w| w.write_message(s))?; }
        if let Some(ref s) = self.from_trusted_server { w.write_with_tag(64, |w| w.write_bool(*s))?; }
        if let Some(ref s) = self.persistent_id { w.write_with_tag(74, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.stream_id { w.write_with_tag(80, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.last_stream_id_received { w.write_with_tag(88, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.permission { w.write_with_tag(98, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.reg_id { w.write_with_tag(106, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.pkg_signature { w.write_with_tag(114, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.client_id { w.write_with_tag(122, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.device_user_id { w.write_with_tag(128, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.ttl { w.write_with_tag(136, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.sent { w.write_with_tag(144, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.queued { w.write_with_tag(152, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.status { w.write_with_tag(160, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.raw_data { w.write_with_tag(170, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.delay { w.write_with_tag(176, |w| w.write_int32(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct StreamAck { }

impl<'a> MessageRead<'a> for StreamAck {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for StreamAck { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct SelectiveAck<'a> {
    pub id: Vec<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for SelectiveAck<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.id.push(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for SelectiveAck<'a> {
    fn get_size(&self) -> usize {
        0
        + self.id.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.id { w.write_with_tag(10, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct BindAccountRequest<'a> {
    pub packetid: Option<Cow<'a, str>>,
    pub domain: Option<Cow<'a, str>>,
    pub user: Option<Cow<'a, str>>,
    pub resource: Option<Cow<'a, str>>,
    pub accountid: Option<i64>,
    pub token: Option<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for BindAccountRequest<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.packetid = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(18) => msg.domain = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(26) => msg.user = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(34) => msg.resource = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(72) => msg.accountid = Some(r.read_int64(bytes)?),
                Ok(42) => msg.token = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for BindAccountRequest<'a> {
    fn get_size(&self) -> usize {
        0
        + self.packetid.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.domain.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.user.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.resource.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.accountid.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.token.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.packetid { w.write_with_tag(10, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.domain { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.user { w.write_with_tag(26, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.resource { w.write_with_tag(34, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.accountid { w.write_with_tag(72, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.token { w.write_with_tag(42, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct BindAccountResponse<'a> {
    pub packetid: Option<Cow<'a, str>>,
    pub jid: Option<Cow<'a, str>>,
    pub laststreamid: Option<i32>,
    pub streamid: Option<i32>,
    pub error: Option<ErrorInfo<'a>>,
}

impl<'a> MessageRead<'a> for BindAccountResponse<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.packetid = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(18) => msg.jid = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(40) => msg.laststreamid = Some(r.read_int32(bytes)?),
                Ok(32) => msg.streamid = Some(r.read_int32(bytes)?),
                Ok(26) => msg.error = Some(r.read_message::<ErrorInfo>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for BindAccountResponse<'a> {
    fn get_size(&self) -> usize {
        0
        + self.packetid.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.jid.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.laststreamid.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.streamid.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.error.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.packetid { w.write_with_tag(10, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.jid { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.laststreamid { w.write_with_tag(40, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.streamid { w.write_with_tag(32, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.error { w.write_with_tag(26, |w| w.write_message(s))?; }
        Ok(())
    }
}

