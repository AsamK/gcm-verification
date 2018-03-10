//! Automatically generated rust module for 'checkin.proto' file

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
#![allow(unknown_lints)]
#![allow(clippy)]
#![cfg_attr(rustfmt, rustfmt_skip)]


use std::io::Write;
use std::borrow::Cow;
use quick_protobuf::{MessageWrite, BytesReader, Writer, Result};
use quick_protobuf::sizeofs::*;
use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct CheckinRequest<'a> {
    pub imei: Option<Cow<'a, str>>,
    pub androidId: Option<i64>,
    pub digest: Option<Cow<'a, str>>,
    pub checkin: mod_CheckinRequest::Checkin<'a>,
    pub desiredBuild: Option<Cow<'a, str>>,
    pub locale: Option<Cow<'a, str>>,
    pub loggingId: Option<i64>,
    pub marketCheckin: Option<Cow<'a, str>>,
    pub macAddress: Vec<Cow<'a, str>>,
    pub meid: Option<Cow<'a, str>>,
    pub accountCookie: Vec<Cow<'a, str>>,
    pub timeZone: Option<Cow<'a, str>>,
    pub securityToken: Option<u64>,
    pub version: Option<i32>,
    pub otaCert: Vec<Cow<'a, str>>,
    pub serial: Option<Cow<'a, str>>,
    pub esn: Option<Cow<'a, str>>,
    pub deviceConfiguration: Option<mod_CheckinRequest::DeviceConfig<'a>>,
    pub macAddressType: Vec<Cow<'a, str>>,
    pub fragment: i32,
    pub userName: Option<Cow<'a, str>>,
    pub userSerialNumber: Option<i32>,
}

impl<'a> CheckinRequest<'a> {
    pub fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.imei = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(16) => msg.androidId = Some(r.read_int64(bytes)?),
                Ok(26) => msg.digest = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(34) => msg.checkin = r.read_message(bytes, mod_CheckinRequest::Checkin::from_reader)?,
                Ok(42) => msg.desiredBuild = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(50) => msg.locale = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(56) => msg.loggingId = Some(r.read_int64(bytes)?),
                Ok(66) => msg.marketCheckin = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(74) => msg.macAddress.push(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(82) => msg.meid = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(90) => msg.accountCookie.push(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(98) => msg.timeZone = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(105) => msg.securityToken = Some(r.read_fixed64(bytes)?),
                Ok(112) => msg.version = Some(r.read_int32(bytes)?),
                Ok(122) => msg.otaCert.push(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(130) => msg.serial = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(138) => msg.esn = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(146) => msg.deviceConfiguration = Some(r.read_message(bytes, mod_CheckinRequest::DeviceConfig::from_reader)?),
                Ok(154) => msg.macAddressType.push(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(160) => msg.fragment = r.read_int32(bytes)?,
                Ok(170) => msg.userName = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(176) => msg.userSerialNumber = Some(r.read_int32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for CheckinRequest<'a> {
    fn get_size(&self) -> usize {
        0
        + self.imei.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.androidId.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.digest.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + 1 + sizeof_len((&self.checkin).get_size())
        + self.desiredBuild.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.locale.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.loggingId.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.marketCheckin.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.macAddress.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
        + self.meid.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.accountCookie.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
        + self.timeZone.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.securityToken.as_ref().map_or(0, |_| 1 + 8)
        + self.version.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.otaCert.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
        + self.serial.as_ref().map_or(0, |m| 2 + sizeof_len((m).len()))
        + self.esn.as_ref().map_or(0, |m| 2 + sizeof_len((m).len()))
        + self.deviceConfiguration.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.macAddressType.iter().map(|s| 2 + sizeof_len((s).len())).sum::<usize>()
        + 2 + sizeof_varint(*(&self.fragment) as u64)
        + self.userName.as_ref().map_or(0, |m| 2 + sizeof_len((m).len()))
        + self.userSerialNumber.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.imei { w.write_with_tag(10, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.androidId { w.write_with_tag(16, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.digest { w.write_with_tag(26, |w| w.write_string(&**s))?; }
        w.write_with_tag(34, |w| w.write_message(&self.checkin))?;
        if let Some(ref s) = self.desiredBuild { w.write_with_tag(42, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.locale { w.write_with_tag(50, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.loggingId { w.write_with_tag(56, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.marketCheckin { w.write_with_tag(66, |w| w.write_string(&**s))?; }
        for s in &self.macAddress { w.write_with_tag(74, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.meid { w.write_with_tag(82, |w| w.write_string(&**s))?; }
        for s in &self.accountCookie { w.write_with_tag(90, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.timeZone { w.write_with_tag(98, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.securityToken { w.write_with_tag(105, |w| w.write_fixed64(*s))?; }
        if let Some(ref s) = self.version { w.write_with_tag(112, |w| w.write_int32(*s))?; }
        for s in &self.otaCert { w.write_with_tag(122, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.serial { w.write_with_tag(130, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.esn { w.write_with_tag(138, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.deviceConfiguration { w.write_with_tag(146, |w| w.write_message(s))?; }
        for s in &self.macAddressType { w.write_with_tag(154, |w| w.write_string(&**s))?; }
        w.write_with_tag(160, |w| w.write_int32(*&self.fragment))?;
        if let Some(ref s) = self.userName { w.write_with_tag(170, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.userSerialNumber { w.write_with_tag(176, |w| w.write_int32(*s))?; }
        Ok(())
    }
}

pub mod mod_CheckinRequest {

use std::borrow::Cow;
use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Checkin<'a> {
    pub build: mod_CheckinRequest::mod_Checkin::Build<'a>,
    pub lastCheckinMs: Option<i64>,
    pub event: Vec<mod_CheckinRequest::mod_Checkin::Event<'a>>,
    pub stat: Vec<mod_CheckinRequest::mod_Checkin::Statistic<'a>>,
    pub requestedGroup: Vec<Cow<'a, str>>,
    pub cellOperator: Option<Cow<'a, str>>,
    pub simOperator: Option<Cow<'a, str>>,
    pub roaming: Option<Cow<'a, str>>,
    pub userNumber: Option<i32>,
}

impl<'a> Checkin<'a> {
    pub fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.build = r.read_message(bytes, mod_CheckinRequest::mod_Checkin::Build::from_reader)?,
                Ok(16) => msg.lastCheckinMs = Some(r.read_int64(bytes)?),
                Ok(26) => msg.event.push(r.read_message(bytes, mod_CheckinRequest::mod_Checkin::Event::from_reader)?),
                Ok(34) => msg.stat.push(r.read_message(bytes, mod_CheckinRequest::mod_Checkin::Statistic::from_reader)?),
                Ok(42) => msg.requestedGroup.push(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(50) => msg.cellOperator = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(58) => msg.simOperator = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(66) => msg.roaming = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(72) => msg.userNumber = Some(r.read_int32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Checkin<'a> {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_len((&self.build).get_size())
        + self.lastCheckinMs.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.event.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.stat.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.requestedGroup.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
        + self.cellOperator.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.simOperator.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.roaming.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.userNumber.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(10, |w| w.write_message(&self.build))?;
        if let Some(ref s) = self.lastCheckinMs { w.write_with_tag(16, |w| w.write_int64(*s))?; }
        for s in &self.event { w.write_with_tag(26, |w| w.write_message(s))?; }
        for s in &self.stat { w.write_with_tag(34, |w| w.write_message(s))?; }
        for s in &self.requestedGroup { w.write_with_tag(42, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.cellOperator { w.write_with_tag(50, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.simOperator { w.write_with_tag(58, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.roaming { w.write_with_tag(66, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.userNumber { w.write_with_tag(72, |w| w.write_int32(*s))?; }
        Ok(())
    }
}

pub mod mod_Checkin {

use std::borrow::Cow;
use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Build<'a> {
    pub fingerprint: Option<Cow<'a, str>>,
    pub hardware: Option<Cow<'a, str>>,
    pub brand: Option<Cow<'a, str>>,
    pub radio: Option<Cow<'a, str>>,
    pub bootloader: Option<Cow<'a, str>>,
    pub clientId: Option<Cow<'a, str>>,
    pub time: Option<i64>,
    pub packageVersionCode: Option<i32>,
    pub device: Option<Cow<'a, str>>,
    pub sdkVersion: Option<i32>,
    pub model: Option<Cow<'a, str>>,
    pub manufacturer: Option<Cow<'a, str>>,
    pub product: Option<Cow<'a, str>>,
    pub otaInstalled: Option<bool>,
}

impl<'a> Build<'a> {
    pub fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.fingerprint = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(18) => msg.hardware = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(26) => msg.brand = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(34) => msg.radio = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(42) => msg.bootloader = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(50) => msg.clientId = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(56) => msg.time = Some(r.read_int64(bytes)?),
                Ok(64) => msg.packageVersionCode = Some(r.read_int32(bytes)?),
                Ok(74) => msg.device = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(80) => msg.sdkVersion = Some(r.read_int32(bytes)?),
                Ok(90) => msg.model = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(98) => msg.manufacturer = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(106) => msg.product = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(112) => msg.otaInstalled = Some(r.read_bool(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Build<'a> {
    fn get_size(&self) -> usize {
        0
        + self.fingerprint.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.hardware.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.brand.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.radio.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.bootloader.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.clientId.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.time.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.packageVersionCode.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.device.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.sdkVersion.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.model.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.manufacturer.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.product.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.otaInstalled.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.fingerprint { w.write_with_tag(10, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.hardware { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.brand { w.write_with_tag(26, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.radio { w.write_with_tag(34, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.bootloader { w.write_with_tag(42, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.clientId { w.write_with_tag(50, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.time { w.write_with_tag(56, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.packageVersionCode { w.write_with_tag(64, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.device { w.write_with_tag(74, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.sdkVersion { w.write_with_tag(80, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.model { w.write_with_tag(90, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.manufacturer { w.write_with_tag(98, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.product { w.write_with_tag(106, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.otaInstalled { w.write_with_tag(112, |w| w.write_bool(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Event<'a> {
    pub tag: Option<Cow<'a, str>>,
    pub value: Option<Cow<'a, str>>,
    pub timeMs: Option<i64>,
}

impl<'a> Event<'a> {
    pub fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.tag = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(18) => msg.value = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(24) => msg.timeMs = Some(r.read_int64(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Event<'a> {
    fn get_size(&self) -> usize {
        0
        + self.tag.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.value.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.timeMs.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.tag { w.write_with_tag(10, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.value { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.timeMs { w.write_with_tag(24, |w| w.write_int64(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Statistic<'a> {
    pub tag: Cow<'a, str>,
    pub count: Option<i32>,
    pub sum: Option<f32>,
}

impl<'a> Statistic<'a> {
    pub fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.tag = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(16) => msg.count = Some(r.read_int32(bytes)?),
                Ok(29) => msg.sum = Some(r.read_float(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Statistic<'a> {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_len((&self.tag).len())
        + self.count.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.sum.as_ref().map_or(0, |_| 1 + 4)
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(10, |w| w.write_string(&**&self.tag))?;
        if let Some(ref s) = self.count { w.write_with_tag(16, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.sum { w.write_with_tag(29, |w| w.write_float(*s))?; }
        Ok(())
    }
}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct DeviceConfig<'a> {
    pub touchScreen: Option<i32>,
    pub keyboardType: Option<i32>,
    pub navigation: Option<i32>,
    pub screenLayout: Option<i32>,
    pub hasHardKeyboard: Option<bool>,
    pub hasFiveWayNavigation: Option<bool>,
    pub densityDpi: Option<i32>,
    pub glEsVersion: Option<i32>,
    pub sharedLibrary: Vec<Cow<'a, str>>,
    pub availableFeature: Vec<Cow<'a, str>>,
    pub nativePlatform: Vec<Cow<'a, str>>,
    pub widthPixels: Option<i32>,
    pub heightPixels: Option<i32>,
    pub locale: Vec<Cow<'a, str>>,
    pub glExtension: Vec<Cow<'a, str>>,
    pub deviceClass: Option<i32>,
    pub maxApkDownloadSizeMb: Option<i32>,
}

impl<'a> DeviceConfig<'a> {
    pub fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.touchScreen = Some(r.read_int32(bytes)?),
                Ok(16) => msg.keyboardType = Some(r.read_int32(bytes)?),
                Ok(24) => msg.navigation = Some(r.read_int32(bytes)?),
                Ok(32) => msg.screenLayout = Some(r.read_int32(bytes)?),
                Ok(40) => msg.hasHardKeyboard = Some(r.read_bool(bytes)?),
                Ok(48) => msg.hasFiveWayNavigation = Some(r.read_bool(bytes)?),
                Ok(56) => msg.densityDpi = Some(r.read_int32(bytes)?),
                Ok(64) => msg.glEsVersion = Some(r.read_int32(bytes)?),
                Ok(74) => msg.sharedLibrary.push(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(82) => msg.availableFeature.push(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(90) => msg.nativePlatform.push(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(96) => msg.widthPixels = Some(r.read_int32(bytes)?),
                Ok(104) => msg.heightPixels = Some(r.read_int32(bytes)?),
                Ok(114) => msg.locale.push(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(122) => msg.glExtension.push(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(128) => msg.deviceClass = Some(r.read_int32(bytes)?),
                Ok(136) => msg.maxApkDownloadSizeMb = Some(r.read_int32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for DeviceConfig<'a> {
    fn get_size(&self) -> usize {
        0
        + self.touchScreen.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.keyboardType.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.navigation.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.screenLayout.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.hasHardKeyboard.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.hasFiveWayNavigation.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.densityDpi.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.glEsVersion.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.sharedLibrary.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
        + self.availableFeature.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
        + self.nativePlatform.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
        + self.widthPixels.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.heightPixels.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.locale.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
        + self.glExtension.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
        + self.deviceClass.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.maxApkDownloadSizeMb.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.touchScreen { w.write_with_tag(8, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.keyboardType { w.write_with_tag(16, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.navigation { w.write_with_tag(24, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.screenLayout { w.write_with_tag(32, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.hasHardKeyboard { w.write_with_tag(40, |w| w.write_bool(*s))?; }
        if let Some(ref s) = self.hasFiveWayNavigation { w.write_with_tag(48, |w| w.write_bool(*s))?; }
        if let Some(ref s) = self.densityDpi { w.write_with_tag(56, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.glEsVersion { w.write_with_tag(64, |w| w.write_int32(*s))?; }
        for s in &self.sharedLibrary { w.write_with_tag(74, |w| w.write_string(&**s))?; }
        for s in &self.availableFeature { w.write_with_tag(82, |w| w.write_string(&**s))?; }
        for s in &self.nativePlatform { w.write_with_tag(90, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.widthPixels { w.write_with_tag(96, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.heightPixels { w.write_with_tag(104, |w| w.write_int32(*s))?; }
        for s in &self.locale { w.write_with_tag(114, |w| w.write_string(&**s))?; }
        for s in &self.glExtension { w.write_with_tag(122, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.deviceClass { w.write_with_tag(128, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.maxApkDownloadSizeMb { w.write_with_tag(136, |w| w.write_int32(*s))?; }
        Ok(())
    }
}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct CheckinResponse<'a> {
    pub statsOk: Option<bool>,
    pub intent: Vec<mod_CheckinResponse::Intent<'a>>,
    pub timeMs: Option<i64>,
    pub digest: Option<Cow<'a, str>>,
    pub setting: Vec<mod_CheckinResponse::GservicesSetting<'a>>,
    pub marketOk: Option<bool>,
    pub androidId: Option<u64>,
    pub securityToken: Option<u64>,
    pub settingsDiff: Option<bool>,
    pub deleteSetting: Vec<Cow<'a, str>>,
    pub versionInfo: Option<Cow<'a, str>>,
    pub deviceDataVersionInfo: Option<Cow<'a, str>>,
}

impl<'a> CheckinResponse<'a> {
    pub fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.statsOk = Some(r.read_bool(bytes)?),
                Ok(18) => msg.intent.push(r.read_message(bytes, mod_CheckinResponse::Intent::from_reader)?),
                Ok(24) => msg.timeMs = Some(r.read_int64(bytes)?),
                Ok(34) => msg.digest = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(42) => msg.setting.push(r.read_message(bytes, mod_CheckinResponse::GservicesSetting::from_reader)?),
                Ok(48) => msg.marketOk = Some(r.read_bool(bytes)?),
                Ok(57) => msg.androidId = Some(r.read_fixed64(bytes)?),
                Ok(65) => msg.securityToken = Some(r.read_fixed64(bytes)?),
                Ok(72) => msg.settingsDiff = Some(r.read_bool(bytes)?),
                Ok(82) => msg.deleteSetting.push(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(90) => msg.versionInfo = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(98) => msg.deviceDataVersionInfo = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for CheckinResponse<'a> {
    fn get_size(&self) -> usize {
        0
        + self.statsOk.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.intent.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.timeMs.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.digest.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.setting.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.marketOk.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.androidId.as_ref().map_or(0, |_| 1 + 8)
        + self.securityToken.as_ref().map_or(0, |_| 1 + 8)
        + self.settingsDiff.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.deleteSetting.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
        + self.versionInfo.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.deviceDataVersionInfo.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.statsOk { w.write_with_tag(8, |w| w.write_bool(*s))?; }
        for s in &self.intent { w.write_with_tag(18, |w| w.write_message(s))?; }
        if let Some(ref s) = self.timeMs { w.write_with_tag(24, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.digest { w.write_with_tag(34, |w| w.write_string(&**s))?; }
        for s in &self.setting { w.write_with_tag(42, |w| w.write_message(s))?; }
        if let Some(ref s) = self.marketOk { w.write_with_tag(48, |w| w.write_bool(*s))?; }
        if let Some(ref s) = self.androidId { w.write_with_tag(57, |w| w.write_fixed64(*s))?; }
        if let Some(ref s) = self.securityToken { w.write_with_tag(65, |w| w.write_fixed64(*s))?; }
        if let Some(ref s) = self.settingsDiff { w.write_with_tag(72, |w| w.write_bool(*s))?; }
        for s in &self.deleteSetting { w.write_with_tag(82, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.versionInfo { w.write_with_tag(90, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.deviceDataVersionInfo { w.write_with_tag(98, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

pub mod mod_CheckinResponse {

use std::borrow::Cow;
use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Intent<'a> {
    pub action: Option<Cow<'a, str>>,
    pub dataUri: Option<Cow<'a, str>>,
    pub mimeType: Option<Cow<'a, str>>,
    pub javaClass: Option<Cow<'a, str>>,
    pub extra: Vec<mod_CheckinResponse::mod_Intent::Extra<'a>>,
}

impl<'a> Intent<'a> {
    pub fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.action = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(18) => msg.dataUri = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(26) => msg.mimeType = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(34) => msg.javaClass = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(42) => msg.extra.push(r.read_message(bytes, mod_CheckinResponse::mod_Intent::Extra::from_reader)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Intent<'a> {
    fn get_size(&self) -> usize {
        0
        + self.action.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.dataUri.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.mimeType.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.javaClass.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.extra.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.action { w.write_with_tag(10, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.dataUri { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.mimeType { w.write_with_tag(26, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.javaClass { w.write_with_tag(34, |w| w.write_string(&**s))?; }
        for s in &self.extra { w.write_with_tag(42, |w| w.write_message(s))?; }
        Ok(())
    }
}

pub mod mod_Intent {

use std::borrow::Cow;
use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Extra<'a> {
    pub name: Option<Cow<'a, str>>,
    pub value: Option<Cow<'a, str>>,
}

impl<'a> Extra<'a> {
    pub fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(50) => msg.name = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(58) => msg.value = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Extra<'a> {
    fn get_size(&self) -> usize {
        0
        + self.name.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.value.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.name { w.write_with_tag(50, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.value { w.write_with_tag(58, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GservicesSetting<'a> {
    pub name: Option<Cow<'a, [u8]>>,
    pub value: Option<Cow<'a, [u8]>>,
}

impl<'a> GservicesSetting<'a> {
    pub fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.name = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(18) => msg.value = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for GservicesSetting<'a> {
    fn get_size(&self) -> usize {
        0
        + self.name.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.value.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.name { w.write_with_tag(10, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.value { w.write_with_tag(18, |w| w.write_bytes(&**s))?; }
        Ok(())
    }
}

}

