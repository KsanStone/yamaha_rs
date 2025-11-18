mod structs;

use std::{
    io::{Read, Write},
    net::TcpStream,
};

pub use crate::structs::{DeviceInfo, SignalInfo, ZoneProgramList, ZoneStatus};

fn yamaha_http_get(host: &str, path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let full_path = format!("/YamahaExtendedControl{}", path);

    let mut stream = TcpStream::connect((host, 80))?;

    let request = format!("GET {} HTTP/1.1\r\nConnection: close\r\n\r\n", full_path);

    stream.write_all(request.as_bytes())?;

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

pub fn get_device_info(ip: &str) -> DeviceInfo {
    let body = match yamaha_http_get(ip, "/v1/system/getDeviceInfo") {
        Ok(b) => b,
        Err(_) => return DeviceInfo::default(),
    };

    serde_json::from_str(&body).unwrap_or_default()
}

pub fn get_zone_status(ip: &str) -> ZoneStatus {
    let body = match yamaha_http_get(ip, "/v1/main/getStatus") {
        Ok(b) => b,
        Err(_) => return ZoneStatus::default(),
    };

    serde_json::from_str(&body).unwrap_or_default()
}

pub fn get_zone_program_list(ip: &str) -> ZoneProgramList {
    let body = match yamaha_http_get(ip, "/v1/main/getSoundProgramList") {
        Ok(b) => b,
        Err(_) => return ZoneProgramList::default(),
    };

    serde_json::from_str(&body).unwrap_or_default()
}

pub fn get_signal_info(ip: &str) -> SignalInfo {
    let body = match yamaha_http_get(ip, "/v1/main/getSignalInfo") {
        Ok(b) => b,
        Err(_) => return SignalInfo::default(),
    };

    serde_json::from_str(&body).unwrap_or_default()
}

pub fn set_zone_power(ip: &str) {
    let _ = yamaha_http_get(ip, "/v1/main/setPower?power=toggle");
}

pub fn set_volume_up(ip: &str) {
    let _ = yamaha_http_get(ip, "/v1/main/setVolume?volume=up");
}

pub fn set_volume_down(ip: &str) {
    let _ = yamaha_http_get(ip, "/v1/main/setVolume?volume=down");
}

pub fn set_mute(ip: &str, mute: bool) {
    let _ = yamaha_http_get(
        ip,
        &format!(
            "/v1/main/setMute?enable={}",
            if mute { "true" } else { "false" }
        ),
    );
}

pub fn set_pure_direct(ip: &str, direct: bool) {
    let _ = yamaha_http_get(
        ip,
        &format!(
            "/v1/main/setPureDirect?enable={}",
            if direct { "true" } else { "false" }
        ),
    );
}

pub fn set_enhancer(ip: &str, enhancer: bool) {
    let _ = yamaha_http_get(
        ip,
        &format!(
            "/v1/main/setEnhancer?enable={}",
            if enhancer { "true" } else { "false" }
        ),
    );
}

pub fn set_extra_bass(ip: &str, bass: bool) {
    let _ = yamaha_http_get(
        ip,
        &format!(
            "/v1/main/setExtraBass?enable={}",
            if bass { "true" } else { "false" }
        ),
    );
}

pub fn set_sound_program(ip: &str, program: &str) {
    let _ = yamaha_http_get(ip, &format!("/v1/main/setSoundProgram?program={}", program));
}
