mod network_traffic_monitor;
mod network_speed_tester;
mod port_scanner;


use std::env;
// cs50 ai told me to use this and #[tokio::main]
use tokio;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 || args.len() > 2 {
        println!("Usage: cargo run -- <command>. example of command like NTM, NST or PS");
        return;
    }

    if args[1] == "NTM" {
        println!("Network Traffic Monitor");
        network_traffic_monitor::process_packets();
    } 

    else if args[1] == "NST" {
        println!("Network Speed Tester");
        network_speed_tester::process_download_speed().await.unwrap_or_else(|err| {
            eprintln!("Error processing download speed: {}", err);
        });
        network_speed_tester::process_upload_speed().await.unwrap_or_else(|err| {
            eprintln!("Error processing upload speed: {}", err);
        });
        network_speed_tester::process_latency().await.unwrap_or_else(|err| {
            eprintln!("Error processing latency: {}", err);
        });
        network_speed_tester::process_packet_loss().await.unwrap_or_else(|err| {
            eprintln!("Error processing packet loss: {}", err);
        });
    } 

    else if args[1] == "PS" {
        println!("Port Scanner");
        port_scanner::scan_ports();
    }

    else {
        println!("Unknown command: {}", args[1]);
    }
}
