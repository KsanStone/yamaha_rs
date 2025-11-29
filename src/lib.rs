mod discover;
pub mod error;
mod structs;

use serde::de::DeserializeOwned;
use serde_json::Value;
use std::{
    io::{Read, Write},
    net::{TcpStream, ToSocketAddrs},
    time::Duration,
};

pub use crate::discover::discover_yamaha_devices;
use crate::error::{Error, InternalError};
pub use crate::structs::*;

/// Sends a GET request to the Yamaha device at the given IP address and path.
fn yamaha_get(host: &str, path: &str) -> Result<String, InternalError> {
    let addr = (host, 80).to_socket_addrs()?.next().ok_or_else(|| {
        std::io::Error::new(
            std::io::ErrorKind::AddrNotAvailable,
            "Failed to resolve host",
        )
    })?;

    let mut stream = TcpStream::connect_timeout(&addr, Duration::from_secs(5))?;

    stream.set_read_timeout(Some(Duration::from_secs(5)))?;
    stream.set_write_timeout(Some(Duration::from_secs(5)))?;

    stream.write_all(
        format!(
            "GET /YamahaExtendedControl{} HTTP/1.1\r\nConnection: close\r\n\r\n",
            path
        )
        .as_bytes(),
    )?;

    let mut buffer = Vec::new();
    stream.read_to_end(&mut buffer)?;

    let response_str = String::from_utf8_lossy(&buffer);

    if let Some(body_start) = response_str.find("\r\n\r\n") {
        Ok(response_str[body_start + 4..].to_string())
    } else {
        Ok(String::new())
    }
}

/// Generic function to handle parsing JSON and checking response codes
fn execute_request<T: DeserializeOwned>(ip: &str, path: &str) -> Result<T, Error> {
    let body = yamaha_get(ip, path)?;
    let value: Value = serde_json::from_str(&body)?;

    let code = value
        .get("response_code")
        .and_then(|v| v.as_u64())
        .ok_or_else(|| {
            InternalError::DeserializationError(serde::de::Error::custom("Missing response_code field in json response"))
        })? as u32;

    if code == 0 {
        // If T is (), we don't need to try to deserialize the rest of the body
        if std::any::type_name::<T>() == "()" {
            // Unsafe hack to allow returning unit type from this generic without Clone
            // (serde_json::from_value(Null) returns () successfully)
            return Ok(serde_json::from_value(Value::Null).unwrap());
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
    // Case: Command returning specific struct
    ($ip:expr, $path:expr, $ret:ty) => {
        crate::execute_request::<$ret>($ip, &$path)
    };
    // Case: Command returning void/nothing (Success checking only)
    ($ip:expr, $path:expr) => {
        crate::execute_request::<()>($ip, &$path)
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
