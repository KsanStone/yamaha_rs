mod discover;
pub mod enums;
pub mod error;
mod structs;

use crate::enums::*;
use serde::de::DeserializeOwned;
use std::{
    io::{Read, Write},
    net::{TcpStream, ToSocketAddrs},
    time::Duration,
};

pub use crate::discover::discover_yamaha_devices;
use crate::error::{Error, InternalError};
pub use crate::structs::*;
use serde::Serialize;

enum Method {
    Get,
    Post,
}

fn send_request(
    host: &str,
    path: &str,
    method: Method,
    body_json: Option<String>,
) -> Result<String, InternalError> {
    let addr = (host, 80).to_socket_addrs()?.next().ok_or_else(|| {
        std::io::Error::new(
            std::io::ErrorKind::AddrNotAvailable,
            "Failed to resolve host",
        )
    })?;

    let mut stream = TcpStream::connect_timeout(&addr, Duration::from_secs(5))?;
    stream.set_read_timeout(Some(Duration::from_secs(5)))?;
    stream.set_write_timeout(Some(Duration::from_secs(5)))?;

    let method_str = match method {
        Method::Get => "GET",
        Method::Post => "POST",
    };

    let mut request = format!(
        "{} /YamahaExtendedControl{} HTTP/1.1\r\n\
         Host: {}\r\n\
         Connection: close\r\n",
        method_str, path, host
    );

    if let Some(body) = body_json {
        request.push_str("Content-Type: application/json\r\n");
        request.push_str(&format!("Content-Length: {}\r\n", body.len()));
        request.push_str("\r\n"); // End of headers
        request.push_str(&body);
    } else {
        request.push_str("\r\n"); // End of headers
    }

    stream.write_all(request.as_bytes())?;

    let mut buffer = Vec::new();
    stream.read_to_end(&mut buffer)?;

    let response_str = String::from_utf8_lossy(&buffer);

    if let Some(body_start) = response_str.find("\r\n\r\n") {
        Ok(response_str[body_start + 4..].to_string())
    } else {
        Ok(String::new())
    }
}

fn execute_get<T: DeserializeOwned>(ip: &str, path: &str) -> Result<T, Error> {
    let body = send_request(ip, path, Method::Get, None)?;
    parse_response(&body)
}

fn execute_post<T: DeserializeOwned, B: Serialize>(
    ip: &str,
    path: &str,
    body_data: &B,
) -> Result<T, Error> {
    let json_str = serde_json::to_string(body_data)?;
    let body = send_request(ip, path, Method::Post, Some(json_str))?;
    parse_response(&body)
}

fn parse_response<T: DeserializeOwned>(body: &str) -> Result<T, Error> {
    let value: serde_json::Value = serde_json::from_str(body)?;

    let code = value
        .get("response_code")
        .and_then(|v| v.as_u64())
        .ok_or_else(|| {
            InternalError::DeserializationError(serde::de::Error::custom("Missing response_code"))
        })? as u32;

    if code == 0 {
        if std::any::type_name::<T>() == "()" {
            return Ok(serde_json::from_value(serde_json::Value::Null).unwrap());
        }
        let data: T = serde_json::from_value(value)?;
        Ok(data)
    } else {
        Err(Error::ResponseError(ResponseCode::from(code)))
    }
}

/// Macro to shorten API calls.
/// Usage:
/// 1. Get Data:   yamaha_req!(ip, "/path", ReturnType)
/// 2. Simple Cmd: yamaha_req!(ip, "/path")
macro_rules! yamaha_req {
    // GET: yamaha_req!(ip, "/path", ReturnType)
    ($ip:expr, $path:expr, $ret:ty) => {
        crate::execute_get::<$ret>($ip, &$path)
    };
    // GET (Void): yamaha_req!(ip, "/path")
    ($ip:expr, $path:expr) => {
        crate::execute_get::<()>($ip, &$path)
    };
}

macro_rules! yamaha_post_req {
    // POST: yamaha_req!(ip, "/path", body_struct, ReturnType)
    ($ip:expr, $path:expr, $body:expr, $ret:ty) => {
        crate::execute_post::<$ret, _>($ip, &$path, &$body)
    };
    // POST (Void): yamaha_req!(ip, "/path", body_struct)
    ($ip:expr, $path:expr, $body:expr) => {
        crate::execute_post::<(), _>($ip, &$path, &$body)
    };
}

pub fn get_device_info(ip: &str) -> Result<DeviceInfo, Error> {
    yamaha_req!(ip, "/v1/system/getDeviceInfo", DeviceInfo)
}

pub fn get_features(ip: &str) -> Result<DeviceFeatures, Error> {
    yamaha_req!(ip, "/v1/system/getFeatures", DeviceFeatures)
}

pub fn get_zone_status(ip: &str, zone: &str) -> Result<ZoneStatus, Error> {
    yamaha_req!(ip, format!("/v1/{}/getStatus", zone), ZoneStatus)
}

pub fn get_zone_program_list(ip: &str, zone: &str) -> Result<ZoneProgramList, Error> {
    yamaha_req!(
        ip,
        format!("/v1/{}/getSoundProgramList", zone),
        ZoneProgramList
    )
}

pub fn get_signal_info(ip: &str, zone: &str) -> Result<SignalInfo, Error> {
    yamaha_req!(ip, format!("/v1/{}/getSignalInfo", zone), SignalInfo)
}

pub fn net_usb_get_play_info(ip: &str) -> Result<NetUsbPlayInfo, Error> {
    yamaha_req!(ip, "/v1/netusb/getPlayInfo", NetUsbPlayInfo)
}

pub fn toggle_zone_power(ip: &str, zone: &str) -> Result<(), Error> {
    yamaha_req!(ip, format!("/v1/{}/setPower?power=toggle", zone))
}

pub fn set_sleep(ip: &str, zone: &str, time: u32) -> Result<(), Error> {
    yamaha_req!(ip, format!("/v1/{}/setSleep?sleep={}", zone, time))
}

pub fn set_volume_up(ip: &str, zone: &str) -> Result<(), Error> {
    yamaha_req!(ip, format!("/v1/{}/setVolume?volume=up", zone))
}

pub fn set_volume_down(ip: &str, zone: &str) -> Result<(), Error> {
    yamaha_req!(ip, format!("/v1/{}/setVolume?volume=down", zone))
}

pub fn set_mute(ip: &str, zone: &str, mute: bool) -> Result<(), Error> {
    yamaha_req!(ip, format!("/v1/{}/setMute?enable={}", zone, mute))
}

pub fn set_input(ip: &str, zone: &str, input: &str) -> Result<(), Error> {
    yamaha_req!(ip, format!("/v1/{}/setInput?input={}", zone, input))
}

pub fn set_sound_program(ip: &str, zone: &str, program: &str) -> Result<(), Error> {
    yamaha_req!(
        ip,
        format!("/v1/{}/setSoundProgram?program={}", zone, program)
    )
}

pub fn set_3d_surround(ip: &str, zone: &str, enable: bool) -> Result<(), Error> {
    yamaha_req!(ip, format!("/v1/{}/set3dSurround?enable={}", zone, enable))
}

pub fn set_direct(ip: &str, zone: &str, direct: bool) -> Result<(), Error> {
    yamaha_req!(ip, format!("/v1/{}/setDirect?enable={}", zone, direct))
}

pub fn set_pure_direct(ip: &str, zone: &str, direct: bool) -> Result<(), Error> {
    yamaha_req!(ip, format!("/v1/{}/setPureDirect?enable={}", zone, direct))
}

pub fn set_enhancer(ip: &str, zone: &str, enhancer: bool) -> Result<(), Error> {
    yamaha_req!(ip, format!("/v1/{}/setEnhancer?enable={}", zone, enhancer))
}

pub fn set_balance(ip: &str, zone: &str, balance: i32) -> Result<(), Error> {
    yamaha_req!(ip, format!("/v1/{}/setBalance?value={}", zone, balance))
}

pub fn set_extra_bass(ip: &str, zone: &str, bass: bool) -> Result<(), Error> {
    yamaha_req!(ip, format!("/v1/{}/setExtraBass?enable={}", zone, bass))
}

pub fn net_usb_set_playback(ip: &str, playback: Playback) -> Result<(), Error> {
    yamaha_req!(ip, format!("/v1/netusb/setPlayback?playback={}", playback))
}

pub fn net_usb_set_repeat(ip: &str, mode: Repeat) -> Result<(), Error> {
    yamaha_req!(ip, format!("/v1/netusb/setRepeat?mode={}", mode))
}

pub fn net_usb_set_shuffle(ip: &str, mode: Shuffle) -> Result<(), Error> {
    yamaha_req!(ip, format!("/v1/netusb/setShuffle?mode={}", mode))
}

pub fn net_usb_toggle_repeat(ip: &str) -> Result<(), Error> {
    yamaha_req!(ip, "/v1/netusb/toggleRepeat")
}

pub fn net_usb_toggle_shuffle(ip: &str) -> Result<(), Error> {
    yamaha_req!(ip, "/v1/netusb/toggleShuffle")
}

pub fn net_usb_set_search_string(
    ip: &str,
    list_id: &str,
    search_text: &str,
    index: Option<u32>,
) -> Result<(), Error> {
    let req_body = SearchRequest {
        list_id: list_id.to_string(),
        string: search_text.to_string(),
        index,
    };

    yamaha_post_req!(ip, "/v1/netusb/setSearchString", req_body)
}

pub fn net_usb_get_list_info(
    ip: &str,
    input: &str,
    index: u32,
    size: u32,
    lang: &str,
) -> Result<ListInfo, Error> {
    yamaha_req!(
        ip,
        format!(
            "/v1/netusb/getListInfo?input={}&index={}&size={}&lang={}",
            input, index, size, lang
        ),
        ListInfo
    )
}

pub fn net_usb_set_list_control(
    ip: &str,
    list_id: &str,
    control_type: ListControl,
    index: Option<u32>,
    zone: Option<&str>,
) -> Result<(), Error> {
    let mut url = format!(
        "/v1/netusb/setListControl?list_id={}&type={}",
        list_id, control_type
    );

    if let Some(idx) = index {
        url.push_str(&format!("&index={}", idx));
    }

    if let Some(z) = zone {
        url.push_str(&format!("&zone={}", z));
    }

    yamaha_req!(ip, url)
}

pub fn set_subwoofer_volume(ip: &str, zone: &str, volume: i32) -> Result<(), Error> {
    yamaha_req!(ip, format!("/v1/{}/setSubwooferVolume?volume={}", zone, volume))
}

pub fn set_dialogue_lift(ip: &str, zone: &str, value: i32) -> Result<(), Error> {
    yamaha_req!(ip, format!("/v1/{}/setDialogueLift?value={}", zone, value))
}

pub fn set_dialogue_level(ip: &str, zone: &str, value: i32) -> Result<(), Error> {
    yamaha_req!(ip, format!("/v1/{}/setDialogueLevel?value={}", zone, value))
}

pub fn set_dts_dialogue_control(ip: &str, zone: &str, value: i32) -> Result<(), Error> {
    yamaha_req!(ip, format!("/v1/{}/setDtsDialogueControl?value={}", zone, value))
}

pub fn set_tone_bass(ip: &str, zone: &str, bass: i32) -> Result<(), Error> {
    yamaha_req!(ip, format!("/v1/{}/setToneControl?mode=manual&bass={}", zone, bass))
}

pub fn set_tone_treble(ip: &str, zone: &str, treble: i32) -> Result<(), Error> {
    yamaha_req!(ip, format!("/v1/{}/setToneControl?mode=manual&treble={}", zone, treble))
}