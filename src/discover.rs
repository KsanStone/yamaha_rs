use std::collections::HashSet;
use std::io::{Read, Write};
use std::net::{TcpStream, ToSocketAddrs, UdpSocket};
use std::time::{Duration, Instant};

use crate::structs::YamahaDevice;

pub fn discover_yamaha_devices() -> Vec<YamahaDevice> {
    let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
    socket
        .set_read_timeout(Some(Duration::from_secs(3)))
        .unwrap();
    socket.send_to("M-SEARCH * HTTP/1.1\r\nHOST: 239.255.255.250:1900\r\nMAN: \"ssdp:discover\"\r\nMX: 2\r\nST: ssdp:all\r\n\r\n".as_bytes(), "239.255.255.250:1900".to_socket_addrs().unwrap().next().unwrap()).unwrap();
    
    let start = Instant::now();
    let mut buf = [0u8; 4096];

    let mut seen = HashSet::new();
    let mut result = Vec::new();

    while start.elapsed() < Duration::from_secs(3) {
        if let Ok((n, src)) = socket.recv_from(&mut buf) {
            let ip = src.ip();

            if seen.contains(&ip) {
                continue;
            }

            let resp = String::from_utf8_lossy(&buf[..n]);

            if let Some(loc) = extract_header(&resp, "LOCATION") {
                if let Some((friendly, manu)) = extract_device_info(&loc) {
                    if manu == "Yamaha Corporation" {
                        seen.insert(ip.clone());
                        result.push(YamahaDevice {
                            ip,
                            name: friendly,
                        });
                    }
                }
            }
        }
    }

    result
}

fn extract_header(resp: &str, header: &str) -> Option<String> {
    let h = header.to_ascii_lowercase();
    for line in resp.lines() {
        let l = line.trim();
        if l.to_ascii_lowercase().starts_with(&h)
            && let Some(v) = l.split_once(':').map(|x| x.1)
        {
            return Some(v.trim().to_string());
        }
    }
    None
}

fn extract_device_info(location: &str) -> Option<(String, String)> {
    let addr = extract_host_port(location)?;
    let mut stream =
        TcpStream::connect_timeout(&addr.parse().ok()?, Duration::from_millis(800)).ok()?;

    let path = extract_path(location).unwrap_or("/".to_string());
    let req = format!(
        "GET {} HTTP/1.1\r\nHost: {}\r\nConnection: close\r\n\r\n",
        path, addr
    );
    stream.write_all(req.as_bytes()).ok()?;

    let mut buf = Vec::new();
    stream.read_to_end(&mut buf).ok()?;
    let xml = String::from_utf8_lossy(&buf);

    let friendly = extract_xml(&xml, "friendlyName")?;
    let manu = extract_xml(&xml, "manufacturer")?;

    Some((friendly, manu))
}

fn extract_xml(xml: &str, tag: &str) -> Option<String> {
    let open = format!("<{}>", tag);
    let close = format!("</{}>", tag);

    let start = xml.find(&open)? + open.len();
    let end = xml[start..].find(&close)? + start;

    Some(xml[start..end].trim().to_string())
}

fn extract_host_port(url: &str) -> Option<String> {
    let no_proto = url.split("://").nth(1)?;
    Some(no_proto.split('/').next()?.to_string())
}

fn extract_path(url: &str) -> Option<String> {
    let no_proto = url.split("://").nth(1)?;
    Some(
        no_proto
            .split_once('/')
            .map(|x| x.1)
            .map(|s| format!("/{}", s))
            .unwrap_or("/".to_string()),
    )
}
