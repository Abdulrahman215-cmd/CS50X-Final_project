// cs50 ai told me to use all of these
use std::time::{Duration, Instant};
use std::io::Write;
use reqwest;
use std::fs::read; 
use tokio::net::UdpSocket;
use tokio::time::{sleep, timeout};
use tokio::sync::watch;


// cs50 ai helped me with the line code below like using async and Result
pub async fn process_download_speed() -> Result<(), Box<dyn std::error::Error>> {
    println!("----------------------------------------------------------------");
    println!("Downloading a test file to measure download speed\n");
    // similar code I used from line 15-27 and line 37 to see how it would work and used is in this website: https://docs.rs/reqwest/latest/reqwest/
    let (tx, rx) = watch::channel(());
    tokio::spawn(
    async move {
            loop {
                print!(".");
                // cs50 ai told me to use the line below
                std::io::stdout().flush().unwrap();
                sleep(Duration::from_secs(1)).await; 
                if rx.has_changed().is_err() {
                    break;
                }
            }
        }
    );
    let start_time = Instant::now();
    // deepseek ai told me to use this url below
    let body = reqwest::get("http://speedtest.tele2.net/100MB.zip")
        .await?
        .bytes()
        .await?;


        
        let end_time = Instant::now();
        let _ = tx.send(()); 
        print!("\n");
        let elapsed_time = end_time - start_time;
        let speed_mbps = (body.len() as f64 * 8.0) / (elapsed_time.as_secs_f64() * 1_000_000.0);
        println!("Elapsed time: {:?}s, Download speed: {} MB/s", format!("{:.2}", elapsed_time.as_secs_f64()), format!("{:.2}", speed_mbps));
    
    Ok(())
}

pub async fn process_upload_speed() -> Result<(), Box<dyn std::error::Error>> {
    println!("----------------------------------------------------------------");
    println!("Uploading a test file to measure upload speed");

    let start_time = Instant::now();
    let client = reqwest::Client::new();
    // deepseek ai told me to use this url below
    let _res = client.post("https://httpbin.org/post")
        .body(read("15-MB.pdf")?)
        .send()
        .await?;


        let end_time = Instant::now();
        let elapsed_time = end_time - start_time;
        let speed_mbps = (read("15-MB.pdf")?.len() as f64 * 8.0) / (elapsed_time.as_secs_f64() * 1_000_000.0);
        println!("Elapsed time: {:?}s, Upload speed: {} MB/s", format!("{:.2}", elapsed_time.as_secs_f64()), format!("{:.2}", speed_mbps));
    Ok(())
}
        
pub async fn process_latency() -> Result<(), Box<dyn std::error::Error>> {
    println!("----------------------------------------------------------------");
    println!("Measuring network latency by sending a request to a website");
    let start_time = Instant::now();
    // code I used in these 4 lines of code below to see how it would work and used(same thing goes for the other functions in this file) is in this website: https://docs.rs/reqwest/latest/reqwest/
    let _body = reqwest::get("https://www.rust-lang.org")
        .await?
        .text()
        .await?;
        

        let end_time = Instant::now();
        let elapsed_time = (end_time - start_time) * 1000;
        println!("Latency: {:?}ms", format!("{:.2}", elapsed_time.as_secs_f64()));
    Ok(())
}


pub async fn process_packet_loss() -> Result<(), Box<dyn std::error::Error>> {
    println!("----------------------------------------------------------------");
    println!("Measuring packet loss by sending and receiving UDP packets\n");
    let mut sent_packets = 0;
    let mut received_packets = 0;
    // similar code was used for bind and connect in this website: https://docs.rs/tokio/latest/tokio/net/struct.UdpSocket.html
    async fn client(sent_packets: &mut i32) -> Result<UdpSocket, Box<dyn std::error::Error>> {
        let socket = UdpSocket::bind("127.0.0.1:8000").await?;
        let remote_addr = "127.0.0.1:5500";
        socket.connect(remote_addr).await?;
        let message = reqwest::get("https://www.rust-lang.org")
            .await?
            .text()
            .await?;


            for _ in 0..10 {
                let _ = message.clone();
                let cloned_message = message.clone();
                let len = socket.send(cloned_message.as_bytes()).await?; 
                println!("{:?} bytes sent", len);
                *sent_packets += 1;
            }
        Ok(socket)
    }
    
    
    async fn server(received_packets: &mut i32) -> Result<UdpSocket, Box<dyn std::error::Error>> {
        let socket = UdpSocket::bind("127.0.0.1:5500").await?;
        let mut buf = [0; 18313];
        
        loop {
            
            let _ = match timeout(Duration::from_secs(9), socket.recv_from(&mut buf)).await {
                // deepseek ai told me to to use Ok(Ok((len, addr))) it was the Ok inside the OK part that helped me
                Ok(Ok((len, addr))) => {  
                    println!("{:?} bytes received from {:?}", len, addr);
                    let len = socket.send_to(&buf[..len], addr).await?;
                    println!("{:?} bytes sent back", len);
                    *received_packets += 1;
                }
                Ok(Err(e)) => {
                    return Err(e.into());
                }
                Err(_) => {
                    println!("No packets received for 9 seconds, shutting down");
                    break;
                }
            };
        }
        Ok(socket)
        
    }
    // cs50 ai told me to use tokio::join!
    let both = tokio::join!(
        client(&mut sent_packets),
        server(&mut received_packets)
    );
    let _rx = match both {
        (Ok(_), Ok(_)) => {
            print!("Packets sent: {}, Packets received: {}, ", sent_packets, received_packets);
            print!("Packet loss: {}%", format!("{:.1}", (sent_packets - received_packets) as f64 / sent_packets as f64 * 100.0));
        },
        (Err(e), _) | (_, Err(e)) => {
            eprintln!("Error: {}", e);
            return Err(e);
        }
    };
    println!("\n");
    println!("----------------------------------------------------------------");
    Ok(())
}