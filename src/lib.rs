mod structs;
mod discover;

use std::{
    io::{Read, Write},
    net::TcpStream,
};

use serde_json::Value;

pub use crate::structs::{DeviceInfo, ResponseCode, SignalInfo, ZoneProgramList, ZoneStatus};

fn yamaha_get(host: &str, path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut stream = TcpStream::connect((host, 80))?;

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

pub fn get_zone_status(ip: &str) -> Result<ZoneStatus, ResponseCode> {
    let body = match yamaha_get(ip, "/v1/main/getStatus") {
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

pub fn get_zone_program_list(ip: &str) -> Result<ZoneProgramList, ResponseCode> {
    let body = match yamaha_get(ip, "/v1/main/getSoundProgramList") {
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

pub fn get_signal_info(ip: &str) -> Result<SignalInfo, ResponseCode> {
    let body = match yamaha_get(ip, "/v1/main/getSignalInfo") {
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

pub fn set_mute(ip: &str, mute: bool) -> Result<(), ResponseCode> {
    let body = match yamaha_get(
        ip,
        &format!(
            "/v1/main/setMute?enable={}",
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

pub fn set_pure_direct(ip: &str, direct: bool) -> Result<(), ResponseCode> {
    let body = match yamaha_get(
        ip,
        &format!(
            "/v1/main/setPureDirect?enable={}",
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

pub fn set_enhancer(ip: &str, enhancer: bool) -> Result<(), ResponseCode> {
    let body = match yamaha_get(
        ip,
        &format!(
            "/v1/main/setEnhancer?enable={}",
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

pub fn set_extra_bass(ip: &str, bass: bool) -> Result<(), ResponseCode> {
    let body = match yamaha_get(
        ip,
        &format!(
            "/v1/main/setExtraBass?enable={}",
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

pub fn set_sound_program(ip: &str, program: &str) -> Result<(), ResponseCode> {
    let body = match yamaha_get(ip, &format!("/v1/main/setSoundProgram?program={}", program)) {
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
