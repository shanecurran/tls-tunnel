#![warn(rust_2018_idioms)]

use tokio::io::AsyncWriteExt;
use futures::FutureExt;

/// The target URL to forward all traffic through.
/// Can be overridden by passing in a URL as the only argument at runtime.
/// Also supports additional protocols as well as explicit port setting.
const DEFAULT_TARGET_ADDR: &'static str = "https://httpbin.org";

/// Returns an active `tokio` TcpListener
/// 
/// Iterates through all ports from `1025` to `65535` until it finds an available port to bind to,
/// other wise returns an `AddrInUse` error.
async fn get_listener() -> Result<tokio::net::TcpListener, std::io::Error> {
    for port in 1025..65535 {
        match tokio::net::TcpListener::bind(
            std::net::SocketAddr::new(
                std::net::IpAddr::V4(
                    std::net::Ipv4Addr::new(0, 0, 0, 0)
                ), 
                port as u16
            )
        ).await {
            Ok(l) => {
                println!("Successfully started TLS over TCP tunnel; port={}", port);
                return Ok(l)
            },
            _ => {}
        }
    }

    Err(
        std::io::Error::new(
            std::io::ErrorKind::AddrInUse, 
            "Could not bind to available port"
        )
    )
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let target_url = match std::env::args().nth(1) {
        Some(host) => host,
        None => DEFAULT_TARGET_ADDR.to_string()
    };

    let parsed_url = url::Url::parse(&target_url).expect("Invalid target URL specified");
    let target_host = parsed_url.host().expect("Invalid target URL specified");
    let target_port = parsed_url.port_or_known_default().expect("Invalid or no target port specified");

    let mut config = tokio_rustls::rustls::ClientConfig::new();
    config.root_store.add_server_trust_anchors(&webpki_roots::TLS_SERVER_ROOTS);
    
    let listener = get_listener().await?;

    while let Ok((inbound, _)) = listener.accept().await {
        let transfer = transfer(inbound, config.clone(), target_host.to_string(), target_port).map(|_| {});

        tokio::spawn(transfer);
    }

    Ok(())
}

/// Takes a `tokio` TcpStream and forwards all bytes to a fresh `tokio_rustls` `TlsStream`.
/// 
/// Also performs TLS certificate validation using the default `webpki-roots` Root CA list.
async fn transfer(mut inbound: tokio::net::TcpStream, config: tokio_rustls::rustls::ClientConfig, target_host: String, target_port: u16) -> Result<(), Box<dyn std::error::Error>> { 
    let config = tokio_rustls::TlsConnector::from(std::sync::Arc::new(config));
    let dnsname = webpki::DNSNameRef::try_from_ascii_str(&target_host)?;
    let stream = tokio::net::TcpStream::connect((target_host.to_string(), target_port)).await?;
    let outbound = config.connect(dnsname, stream).await?;

    let (mut ri, mut wi) = inbound.split();
    let (mut ro, mut wo) = tokio::io::split(outbound);

    let client_to_server = async {
        tokio::io::copy(&mut ri, &mut wo).await?;
        wo.shutdown().await
    };

    let server_to_client = async {
        tokio::io::copy(&mut ro, &mut wi).await?;
        wi.shutdown().await
    };

    tokio::try_join!(client_to_server, server_to_client)?;

    Ok(())
}