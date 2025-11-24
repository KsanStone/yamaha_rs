mod discover;
mod structs;

use std::{
    io::{Read, Write},
    net::{TcpStream, ToSocketAddrs},
    time::Duration,
};

use serde_json::Value;

pub use crate::discover::discover_yamaha_devices;

use crate::structs::DeviceFeatures;
pub use crate::structs::{
    DeviceInfo, ResponseCode, SignalInfo, YamahaDevice, ZoneProgramList, ZoneStatus,
};

fn yamaha_get(host: &str, path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let addr = (host, 80)
        .to_socket_addrs()?
        .next()
        .ok_or("Failed to resolve host")?;

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
        let body = &response_str[body_start + 4..];
        Ok(body.to_string())
    } else {
        Ok(String::new())
    }
}

pub fn get_device_info(ip: &str) -> Result<DeviceInfo, ResponseCode> {
    let body = match yamaha_get(ip, "/v1/system/getDeviceInfo") {
        Ok(b) => b,
        Err(_) => return Err(ResponseCode::InternalError),
    };

    let value: Value = serde_json::from_str(&body).map_err(|_| ResponseCode::InternalError)?;
    let code = value
        .get("response_code")
        .and_then(|v| v.as_u64())
        .ok_or(ResponseCode::InternalError)? as u32;
    if code == 0 {
        let info: DeviceInfo =
            serde_json::from_value(value).map_err(|_| ResponseCode::InternalError)?;
        Ok(info)
    } else {
        Err(ResponseCode::from(code))
    }
}

pub fn get_zone_status(ip: &str, zone: &str) -> Result<ZoneStatus, ResponseCode> {
    let body = match yamaha_get(ip, &format!("/v1/{}/getStatus", zone)) {
        Ok(b) => b,
        Err(_) => return Err(ResponseCode::InternalError),
    };

    let value: Value = serde_json::from_str(&body).map_err(|_| ResponseCode::InternalError)?;
    let code = value
        .get("response_code")
        .and_then(|v| v.as_u64())
        .ok_or(ResponseCode::InternalError)? as u32;
    if code == 0 {
        let status: ZoneStatus =
            serde_json::from_value(value).map_err(|_| ResponseCode::InternalError)?;
        Ok(status)
    } else {
        Err(ResponseCode::from(code))
    }
}

pub fn get_zone_program_list(ip: &str, zone: &str) -> Result<ZoneProgramList, ResponseCode> {
    let body = match yamaha_get(ip, &format!("/v1/{}/getSoundProgramList", zone)) {
        Ok(b) => b,
        Err(_) => return Err(ResponseCode::InternalError),
    };

    let value: Value = serde_json::from_str(&body).map_err(|_| ResponseCode::InternalError)?;
    let code = value
        .get("response_code")
        .and_then(|v| v.as_u64())
        .ok_or(ResponseCode::InternalError)? as u32;
    if code == 0 {
        let list: ZoneProgramList =
            serde_json::from_value(value).map_err(|_| ResponseCode::InternalError)?;
        Ok(list)
    } else {
        Err(ResponseCode::from(code))
    }
}

pub fn get_signal_info(ip: &str, zone: &str) -> Result<SignalInfo, ResponseCode> {
    let body = match yamaha_get(ip, &format!("/v1/{}/getSignalInfo", zone)) {
        Ok(b) => b,
        Err(_) => return Err(ResponseCode::InternalError),
    };

    let value: Value = serde_json::from_str(&body).map_err(|_| ResponseCode::InternalError)?;
    let code = value
        .get("response_code")
        .and_then(|v| v.as_u64())
        .ok_or(ResponseCode::InternalError)? as u32;
    if code == 0 {
        let info: SignalInfo =
            serde_json::from_value(value).map_err(|_| ResponseCode::InternalError)?;
        Ok(info)
    } else {
        Err(ResponseCode::from(code))
    }
}

pub fn toggle_zone_power(ip: &str, zone: &str) -> Result<(), ResponseCode> {
    let body = match yamaha_get(ip, &format!("/v1/{}/setPower?power=toggle", zone)) {
        Ok(b) => b,
        Err(_) => return Err(ResponseCode::InternalError),
    };

    let value: Value = serde_json::from_str(&body).map_err(|_| ResponseCode::InternalError)?;
    let code = value
        .get("response_code")
        .and_then(|v| v.as_u64())
        .ok_or(ResponseCode::InternalError)? as u32;
    if code == 0 {
        Ok(())
    } else {
        Err(ResponseCode::from(code))
    }
}

pub fn set_sleep(ip: &str, zone: &str, time: u32) -> Result<(), ResponseCode> {
    let body = match yamaha_get(ip, &format!("/v1/{}/setSleep?sleep={}", zone, time)) {
        Ok(b) => b,
        Err(_) => return Err(ResponseCode::InternalError),
    };

    let value: Value = serde_json::from_str(&body).map_err(|_| ResponseCode::InternalError)?;
    let code = value
        .get("response_code")
        .and_then(|v| v.as_u64())
        .ok_or(ResponseCode::InternalError)? as u32;
    if code == 0 {
        Ok(())
    } else {
        Err(ResponseCode::from(code))
    }
}

pub fn set_volume_up(ip: &str, zone: &str) -> Result<(), ResponseCode> {
    let body = match yamaha_get(ip, &format!("/v1/{}/setVolume?volume=up", zone)) {
        Ok(b) => b,
        Err(_) => return Err(ResponseCode::InternalError),
    };

    let value: Value = serde_json::from_str(&body).map_err(|_| ResponseCode::InternalError)?;
    let code = value
        .get("response_code")
        .and_then(|v| v.as_u64())
        .ok_or(ResponseCode::InternalError)? as u32;
    if code == 0 {
        Ok(())
    } else {
        Err(ResponseCode::from(code))
    }
}

pub fn set_volume_down(ip: &str, zone: &str) -> Result<(), ResponseCode> {
    let body = match yamaha_get(ip, &format!("/v1/{}/setVolume?volume=down", zone)) {
        Ok(b) => b,
        Err(_) => return Err(ResponseCode::InternalError),
    };

    let value: Value = serde_json::from_str(&body).map_err(|_| ResponseCode::InternalError)?;
    let code = value
        .get("response_code")
        .and_then(|v| v.as_u64())
        .ok_or(ResponseCode::InternalError)? as u32;
    if code == 0 {
        Ok(())
    } else {
        Err(ResponseCode::from(code))
    }
}

pub fn set_mute(ip: &str, zone: &str, mute: bool) -> Result<(), ResponseCode> {
    let body = match yamaha_get(
        ip,
        &format!(
            "/v1/{}/setMute?enable={}",
            zone,
            if mute { "true" } else { "false" }
        ),
    ) {
        Ok(b) => b,
        Err(_) => return Err(ResponseCode::InternalError),
    };

    let value: Value = serde_json::from_str(&body).map_err(|_| ResponseCode::InternalError)?;
    let code = value
        .get("response_code")
        .and_then(|v| v.as_u64())
        .ok_or(ResponseCode::InternalError)? as u32;
    if code == 0 {
        Ok(())
    } else {
        Err(ResponseCode::from(code))
    }
}

pub fn set_input(ip: &str, zone: &str, input: &str) -> Result<(), ResponseCode> {
    let body = match yamaha_get(ip, &format!("/v1/{}/setInput?input={}", zone, input)) {
        Ok(b) => b,
        Err(_) => return Err(ResponseCode::InternalError),
    };

    let value: Value = serde_json::from_str(&body).map_err(|_| ResponseCode::InternalError)?;
    let code = value
        .get("response_code")
        .and_then(|v| v.as_u64())
        .ok_or(ResponseCode::InternalError)? as u32;
    if code == 0 {
        Ok(())
    } else {
        Err(ResponseCode::from(code))
    }
}

pub fn set_sound_program(ip: &str, zone: &str, program: &str) -> Result<(), ResponseCode> {
    let body = match yamaha_get(
        ip,
        &format!("/v1/{}/setSoundProgram?program={}", zone, program),
    ) {
        Ok(b) => b,
        Err(_) => return Err(ResponseCode::InternalError),
    };

    let value: Value = serde_json::from_str(&body).map_err(|_| ResponseCode::InternalError)?;
    let code = value
        .get("response_code")
        .and_then(|v| v.as_u64())
        .ok_or(ResponseCode::InternalError)? as u32;
    if code == 0 {
        Ok(())
    } else {
        Err(ResponseCode::from(code))
    }
}

pub fn set_3d_surround(ip: &str, zone: &str, enable: bool) -> Result<(), ResponseCode> {
    let body = match yamaha_get(
        ip,
        &format!(
            "/v1/{}/set3dSurround?enable={}",
            zone,
            if enable { "true" } else { "false" }
        ),
    ) {
        Ok(b) => b,
        Err(_) => return Err(ResponseCode::InternalError),
    };

    let value: Value = serde_json::from_str(&body).map_err(|_| ResponseCode::InternalError)?;
    let code = value
        .get("response_code")
        .and_then(|v| v.as_u64())
        .ok_or(ResponseCode::InternalError)? as u32;
    if code == 0 {
        Ok(())
    } else {
        Err(ResponseCode::from(code))
    }
}

pub fn set_direct(ip: &str, zone: &str, direct: bool) -> Result<(), ResponseCode> {
    let body = match yamaha_get(
        ip,
        &format!(
            "/v1/{}/setDirect?enable={}",
            zone,
            if direct { "true" } else { "false" }
        ),
    ) {
        Ok(b) => b,
        Err(_) => return Err(ResponseCode::InternalError),
    };

    let value: Value = serde_json::from_str(&body).map_err(|_| ResponseCode::InternalError)?;
    let code = value
        .get("response_code")
        .and_then(|v| v.as_u64())
        .ok_or(ResponseCode::InternalError)? as u32;
    if code == 0 {
        Ok(())
    } else {
        Err(ResponseCode::from(code))
    }
}

pub fn set_pure_direct(ip: &str, zone: &str, direct: bool) -> Result<(), ResponseCode> {
    let body = match yamaha_get(
        ip,
        &format!(
            "/v1/{}/setPureDirect?enable={}",
            zone,
            if direct { "true" } else { "false" }
        ),
    ) {
        Ok(b) => b,
        Err(_) => return Err(ResponseCode::InternalError),
    };

    let value: Value = serde_json::from_str(&body).map_err(|_| ResponseCode::InternalError)?;
    let code = value
        .get("response_code")
        .and_then(|v| v.as_u64())
        .ok_or(ResponseCode::InternalError)? as u32;
    if code == 0 {
        Ok(())
    } else {
        Err(ResponseCode::from(code))
    }
}

pub fn set_enhancer(ip: &str, zone: &str, enhancer: bool) -> Result<(), ResponseCode> {
    let body = match yamaha_get(
        ip,
        &format!(
            "/v1/{}/setEnhancer?enable={}",
            zone,
            if enhancer { "true" } else { "false" }
        ),
    ) {
        Ok(b) => b,
        Err(_) => return Err(ResponseCode::InternalError),
    };

    let value: Value = serde_json::from_str(&body).map_err(|_| ResponseCode::InternalError)?;
    let code = value
        .get("response_code")
        .and_then(|v| v.as_u64())
        .ok_or(ResponseCode::InternalError)? as u32;
    if code == 0 {
        Ok(())
    } else {
        Err(ResponseCode::from(code))
    }
}

pub fn set_balance(ip: &str, zone: &str, balance: i32) -> Result<(), ResponseCode> {
    let body = match yamaha_get(ip, &format!("/v1/{}/setBalance?value={}", zone, balance)) {
        Ok(b) => b,
        Err(_) => return Err(ResponseCode::InternalError),
    };

    let value: Value = serde_json::from_str(&body).map_err(|_| ResponseCode::InternalError)?;
    let code = value
        .get("response_code")
        .and_then(|v| v.as_u64())
        .ok_or(ResponseCode::InternalError)? as u32;
    if code == 0 {
        Ok(())
    } else {
        Err(ResponseCode::from(code))
    }
}

pub fn set_extra_bass(ip: &str, zone: &str, bass: bool) -> Result<(), ResponseCode> {
    let body = match yamaha_get(
        ip,
        &format!(
            "/v1/{}/setExtraBass?enable={}",
            zone,
            if bass { "true" } else { "false" }
        ),
    ) {
        Ok(b) => b,
        Err(_) => return Err(ResponseCode::InternalError),
    };

    let value: Value = serde_json::from_str(&body).map_err(|_| ResponseCode::InternalError)?;
    let code = value
        .get("response_code")
        .and_then(|v| v.as_u64())
        .ok_or(ResponseCode::InternalError)? as u32;
    if code == 0 {
        Ok(())
    } else {
        Err(ResponseCode::from(code))
    }
}

pub fn get_features(ip: &str) -> Result<DeviceFeatures, ResponseCode> {
    let body = match yamaha_get(ip, "/v1/system/getFeatures") {
        Ok(b) => b,
        Err(_) => return Err(ResponseCode::InternalError),
    };

    let value: Value = serde_json::from_str(&body).map_err(|_| ResponseCode::InternalError)?;
    let code = value
        .get("response_code")
        .and_then(|v| v.as_u64())
        .ok_or(ResponseCode::InternalError)? as u32;
    if code == 0 {
        let info: DeviceFeatures =
            serde_json::from_value(value).map_err(|_| ResponseCode::InternalError)?;
        Ok(info)
    } else {
        Err(ResponseCode::from(code))
    }
}
