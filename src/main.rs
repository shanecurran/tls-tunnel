#![warn(rust_2018_idioms)]

use tokio::io::AsyncWriteExt;
use futures::FutureExt;

const DEFAULT_TARGET_ADDR: &'static str = "httpbin.org:443";

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
    let target_host = match std::env::args().nth(1) {
        Some(host) => host,
        None => DEFAULT_TARGET_ADDR.to_string()
    };

    let mut config = tokio_rustls::rustls::ClientConfig::new();
    config.root_store.add_server_trust_anchors(&webpki_roots::TLS_SERVER_ROOTS);
    
    let listener = get_listener().await?;

    while let Ok((inbound, _)) = listener.accept().await {
        let transfer = transfer(inbound, config.clone(), target_host.clone()).map(|_| {});

        tokio::spawn(transfer);
    }

    Ok(())
}

async fn transfer(mut inbound: tokio::net::TcpStream, config: tokio_rustls::rustls::ClientConfig, target_host: String) -> Result<(), Box<dyn std::error::Error>> {
    let hostname = target_host.split(":").collect::<Vec<&str>>()[0];
    let config = tokio_rustls::TlsConnector::from(std::sync::Arc::new(config));
    let dnsname = webpki::DNSNameRef::try_from_ascii_str(&hostname)?;
    let stream = tokio::net::TcpStream::connect(&target_host).await?;
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