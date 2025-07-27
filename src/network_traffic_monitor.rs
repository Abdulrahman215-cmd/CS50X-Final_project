// cs50 ai told me to use all of these
use std::time::Instant;
use pnet::datalink;
use std::collections::HashMap;
use pnet::packet::Packet;
use pnet::packet::ipv4::Ipv4Packet;
use dns_lookup::{lookup_addr};
use std::net::IpAddr;
use pnet::packet::tcp::TcpPacket;


#[derive(Debug)]
#[allow(dead_code)]
struct IPAddress {
    source_ip: String,
    destination_ip: String,
    packet_size: usize,
}


pub fn process_packets () {
    println!("----------------------------------------------------------------");
    println!("Capturing network traffic and measuring download and upload speed\n");
    let mut ip_map: HashMap<String, IPAddress> = HashMap::new();
    // similar code I used to see how it would work and be used(same thing goes for fn capture_packets) is in this website: // https://medium.com/@akmot9/tutorial-rust-packet-protocol-handling-with-pnet-lib-3b5b40fe96ae
    let interfaces = datalink::interfaces();
    for interface in interfaces {
        if interface.name == "\\Device\\NPF_{4F41BC58-BD06-4C22-A53B-38BC1642391A}" {
            capture_packets(interface, &mut ip_map); 
        }
    }
    
    fn capture_packets(interface: datalink::NetworkInterface, ip_map: &mut HashMap<String, IPAddress>) {
        let mut total_length = 0;
        let mut sent_bytes = 0;
        let mut received_bytes = 0;
        let _rx = match datalink::channel(&interface, Default::default()) {
            Ok(datalink::Channel::Ethernet(_tx, mut rx)) => {
                let start_time = Instant::now();
                while let Ok(packet) = rx.next() {
                    // IDK why I didn't thought of it but deepseek ai "suggested" that I use time (specify a duraton to know when to break out of the loop)
                    let elapsed_time = Instant::now().duration_since(start_time);
                    if elapsed_time > std::time::Duration::from_secs(20) {
                        println!("Stopping capture after 20 seconds");
                        break;
                    }

                    // I was having problems with code so deepseek ai told me to use these 4 lines of code
                    if packet.len() <= 14 {
                        eprintln!("Packet too short to process");
                        continue;
                    }

                    // this line of code was through the help of cs50 ai
                    if let Some(ipv4_packet) = Ipv4Packet::new(&packet[14..]) {
                        // cs50 ai told me to use .to_string and .clone() and .parse from line 56-70
                        let source_ip = ipv4_packet.get_source().to_string();
                        let destination_ip = ipv4_packet.get_destination().to_string();
                        let packet_size = packet.len();
                        
                        let ip_address = IPAddress {
                            source_ip: source_ip.clone(),
                            destination_ip: destination_ip.clone(),
                            packet_size,
                        };
                        
                        ip_map.insert(source_ip.clone(), ip_address);

                        let source_ip: IpAddr = source_ip.parse().expect("Invalid IP address");
                        let destination_ip: IpAddr = destination_ip.parse().expect("Invalid IP address");
                        let (source_domain, destination_domain) = (
                            // cs50 ai told me to use lookup_addr
                            lookup_addr(&source_ip).unwrap_or("No domain name found".to_string()),
                            lookup_addr(&destination_ip).unwrap_or("No domain name found".to_string()),
                        );

                        // deepseek ai told me to use .payload()
                        match TcpPacket::new(ipv4_packet.payload()) { 
                            Some(tcp_packet) => {
                                println!("TCP Packet - Source Port: {}, Dest Port: {}", 
                                tcp_packet.get_source(), 
                                tcp_packet.get_destination());
                                
                                if tcp_packet.get_destination() == 443 && source_domain == "alien-mohsin" {
                                    sent_bytes += packet_size;
                                    total_length += packet_size;
                                    println!("HTTPS packet detected and it is sent");
                                    println!("Source domain: {}, Destination domain: {} and Sent size is {} bytes", 
                                        source_domain, destination_ip, packet_size);
                                
                                } else if tcp_packet.get_source() == 443 && destination_domain == "alien-mohsin" {
                                    received_bytes += packet_size;
                                    total_length += packet_size;
                                    println!("HTTPS packet detected and it got received");
                                    println!("Source ip: {}, Destination domain: {} Received size is {} bytes", 
                                        source_ip, destination_domain, packet_size);
                                
                                } else {
                                    println!("Non-HTTPS packet detected");
                                    total_length += packet_size;
                                    println!("packet size is: {} bytes", packet_size);
                                }
                                print!("\n");
                            },
                            None => { 
                                println!("TCP packet not found, possibly a non-TCP packet");
                            }
                        }
                    } else {
                        eprintln!("Received non-IP packet");
                        continue;
                    }
                }
                let download_speed = received_bytes as f64 / 20.0;
                let upload_speed = sent_bytes as f64 / 20.0;
                println!("Total length of captured packets: {} bytes", total_length);
                println!("Download speed is: {} B/s, And upload speed is: {} B/s", download_speed, upload_speed);
            },
            Ok(_) => {
                eprintln!("Unsupported channel type");
                return;
            },
            Err(e) => {
                eprintln!("Error creating channel: {}", e);
                return;
            }
        };
    }
    print!("\n");
    println!("-------------------------------------------------------------");
}