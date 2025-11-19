use std::net::{UdpSocket, SocketAddr};
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct YamahaDevice {
    pub ip: String,
    pub port: u16,
    pub model_name: Option<String>,
    pub raw_response: String,
}

/// Discovers Yamaha amplifiers/receivers supporting the Extended Control API on the local network.
/// Returns a list of discovered devices.
pub fn discover_yamaha_devices(timeout_secs: u64) -> std::io::Result<Vec<YamahaDevice>> {
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    socket.set_read_timeout(Some(Duration::from_secs(timeout_secs)))?;
    socket.set_broadcast(true)?;

    let discovery_msg = b"YamahaExtendedControlV1\r\n";
    let broadcast_addr: SocketAddr = "255.255.255.255:49153".parse().unwrap();

    // Send discovery packet
    socket.send_to(discovery_msg, broadcast_addr)?;

    let mut devices = Vec::new();
    let mut buf = [0; 2048];

    loop {
        match socket.recv_from(&mut buf) {
            Ok((size, src)) => {
                let raw = String::from_utf8_lossy(&buf[..size]).to_string();
                let model_name = extract_model_name(&raw);
                devices.push(YamahaDevice {
                    ip: src.ip().to_string(),
                    port: src.port(),
                    model_name,
                    raw_response: raw,
                });
            }
            Err(e) if e.kind() == std::io::ErrorKind::TimedOut => {
                break;
            }
            Err(e) => {
                // On WouldBlock or other errors, break to avoid hangs
                break;
            }
        }
    }

    // Deduplicate by IP (in case of multiple responses)
    let mut seen = std::collections::HashSet::new();
    devices.retain(|d| seen.insert(d.ip.clone()));

    Ok(devices)
}

// Simple heuristic to extract model_name from Yamaha JSON-like response
fn extract_model_name(response: &str) -> Option<String> {
    // Yamaha responses are JSON-like but may have extra spaces or formatting quirks.
    // We do a basic substring search for reliability without a full JSON parser.
    const MODEL_KEY: &str = "\"model_name\"";
    if let Some(pos) = response.find(MODEL_KEY) {
        let rest = &response[pos + MODEL_KEY.len()..];
        if let Some(colon_pos) = rest.find(':') {
            let after_colon = &rest[colon_pos + 1..];
            // Trim whitespace and quotes
            let trimmed = after_colon.trim_start_matches(|c: char| c.is_whitespace() || c == '"');
            if let Some(end_quote) = trimmed.find('"') {
                let model = &trimmed[..end_quote];
                if !model.is_empty() {
                    return Some(model.to_string());
                }
            }
        }
    }
    None
}

// Example usage
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_model() {
        let resp = r#"{"response_code":0,"model_name":"RX-A2A"}"#;
        assert_eq!(extract_model_name(resp), Some("RX-A2A".to_string()));
    }

    #[test]
    fn test_extract_model_with_spaces() {
        let resp = r#"{ "model_name" : "RX-V6A" }"#;
        assert_eq!(extract_model_name(resp), Some("RX-V6A".to_string()));
    }
}

// If you want to run it as a simple demo
#[cfg(feature = "demo")]
fn main() -> std::io::Result<()> {
    println!("Discovering Yamaha devices...");
    let devices = discover_yamaha_devices(3)?;
    if devices.is_empty() {
        println!("No Yamaha Extended Control devices found.");
    } else {
        for dev in &devices {
            println!(
                "Found: {} at {}:{}",
                dev.model_name.as_deref().unwrap_or("Unknown model"),
                dev.ip,
                dev.port
            );
        }
    }
    Ok(())
}