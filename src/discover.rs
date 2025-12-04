use crate::structs::YamahaDevice;
use log::debug;
use std::collections::HashSet;
use std::io::{Read, Write};
use std::net::{IpAddr, TcpStream, ToSocketAddrs, UdpSocket};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

pub fn discover_yamaha_devices() -> Vec<YamahaDevice> {
    let mut candidates = vec![];
    // Note: only windows need special treatment in order to receive back the UDP packets.
    // it needs to be bound to the interface in the same network as the devices.
    #[cfg(target_os = "windows")]
    {
        debug!("Windows detected, discovering devices from all interfaces.");
        use if_addrs::get_if_addrs;
        use log::error;

        match get_if_addrs() {
            Ok(ifaces) => {
                let mut handles = Vec::new();
                let candidates_arc = Arc::new(Mutex::new(Vec::new()));

                for iface in ifaces {
                    if !iface.ip().is_ipv4() {
                        debug!("Skipping non-IPv4 iface {}", iface.name);
                        continue;
                    }
                    if iface.is_loopback() {
                        debug!("Skipping loopback iface {}", iface.name);
                        continue;
                    }
                    if iface.name.starts_with("vEthernet") {
                        debug!("Skipping virtual iface {}", iface.name);
                        continue;
                    }
                    debug!("Found iface {} {}", iface.name, iface.ip());

                    let ip_str = iface.ip().to_string();
                    let candidates_arc = Arc::clone(&candidates_arc);

                    let handle = thread::spawn(move || {
                        let iface_candidates = discover_candidates_from_iface_addr(&ip_str);
                        let mut c = candidates_arc.lock().unwrap();
                        c.extend(iface_candidates);
                    });

                    handles.push(handle);
                }

                for handle in handles {
                    handle.join().unwrap();
                }
                candidates = candidates_arc.lock().unwrap().clone();
            }
            Err(e) => error!("Failed to get interfaces: {}", e),
        }
    }
    // Mac/linux can receive the broadcast packets from any interface.
    #[cfg(not(target_os = "windows"))]
    {
        debug!("Non-Windows detected, discovering devices from 0.0.0.0");
        candidates.extend(crate::discover::discover_candidates_from_iface_addr(
            "0.0.0.0",
        ));
    }

    let result = Arc::new(Mutex::new(Vec::new()));
    let mut handles = Vec::new();

    for (ip, loc) in candidates {
        let result = Arc::clone(&result);
        let handle = thread::spawn(move || {
            if let Some((friendly, manu)) = extract_device_info(&loc)
                && manu == "Yamaha Corporation"
            {
                let mut r = result.lock().unwrap();
                r.push(YamahaDevice { ip, name: friendly });
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    Arc::try_unwrap(result).unwrap().into_inner().unwrap()
}

fn discover_candidates_from_iface_addr(iface: &str) -> Vec<(IpAddr, String)> {
    let socket = UdpSocket::bind(format!("{iface}:0")).unwrap();
    socket
        .set_read_timeout(Some(Duration::from_secs(3)))
        .unwrap();
    socket
        .set_write_timeout(Some(Duration::from_secs(1)))
        .unwrap();
    socket.send_to("M-SEARCH * HTTP/1.1\r\nHOST: 239.255.255.250:1900\r\nMAN: \"ssdp:discover\"\r\nMX: 2\r\nST: ssdp:all\r\n\r\n".as_bytes(), "239.255.255.250:1900".to_socket_addrs().unwrap().next().unwrap()).unwrap();

    let start = Instant::now();
    let mut buf = [0u8; 4096];

    let mut seen = HashSet::new();
    let mut candidates = Vec::new();

    while start.elapsed() < Duration::from_secs(3) {
        if let Ok((n, src)) = socket.recv_from(&mut buf) {
            let ip = src.ip();

            if seen.contains(&ip) {
                continue;
            }

            let resp = String::from_utf8_lossy(&buf[..n]);

            if let Some(loc) = extract_header(&resp, "LOCATION") {
                seen.insert(ip);
                candidates.push((ip, loc));
            }
        }
    }

    candidates
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

    stream
        .set_write_timeout(Some(Duration::from_secs(1)))
        .unwrap();
    stream
        .set_read_timeout(Some(Duration::from_secs(1)))
        .unwrap();

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
