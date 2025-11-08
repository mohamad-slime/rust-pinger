use std::env;
use std::net::IpAddr;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
/// Asynchronous main function that implements a ping utility
/// It takes optional command line arguments for host and payload size
/// Defaults to "google.com" and 32 bytes if not provided
async fn main() {
    // Collect command line arguments into a vector of strings
    let args: Vec<String> = env::args().collect();
    // Get host from arguments, default to "google.com" if not provided
    let host = args.get(1).map(String::as_str).unwrap_or("google.com");
    // Parse payload size from arguments, default to 32 if not provided or invalid
    let payload_size: usize = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(32);

    // Define maximum allowed payload size constant
    const MAX_PAYLOAD_SIZE: usize = 64_000;
    // Check if payload size exceeds maximum allowed
    if payload_size > MAX_PAYLOAD_SIZE {
        eprintln!("Payload size {} is too large (max is {}).", payload_size, MAX_PAYLOAD_SIZE);
        return;
    }


    // Perform DNS lookup for the host
    let addrs = match tokio::net::lookup_host((host, 0)).await {
        Ok(a) => a,
        Err(e) => {
            eprintln!("DNS lookup failed {}", e);
            return;
        }
    };

    // Extract IP address from the lookup results
    let ip = match addrs
        .filter_map(|address| match address.ip() {
            IpAddr::V4(v4) => Some(IpAddr::V4(v4)),
            IpAddr::V6(v6) => Some(IpAddr::V6(v6)),
        })
        .next()
    {
        Some(ip) => ip,
        None => {
            eprintln!("Could not find a valid IP address for {}", host);
            return;
        }
    };

    // Print the target host and IP address
    println!("Pinging {} ({})", host, ip);

    // Create payload buffer with specified size
    let payload = vec![0; payload_size];


    for round in 0..4 {
        match surge_ping::ping(ip, &payload).await {
            Ok((_packet, duration)) => println!(
                "duration: {:.2?} with {:.2?} bytes size",
                duration,
                payload.len()
            ),
            Err(e) => println!("{:?}", e),
        };
        if round < 3 {
            sleep(Duration::from_secs(1)).await;
        }
    }
}
