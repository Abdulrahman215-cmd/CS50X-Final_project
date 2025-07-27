// cs50 ai told me to use this
use std::net::TcpStream;


pub fn scan_ports() {
    println!("----------------------------------------------------------------");
    println!("Scanning common ports to check if they are open or closed\n");


    // similar code I used to see how it would work and used is in this website: https://doc.rust-lang.org/std/net/struct.TcpStream.html
    if let Ok(_) = TcpStream::connect("127.0.0.1:8000") {
        println!("Port 8000 is open");
    } else {
        println!("Port 8000 is closed");
    }
    if let Ok(_) = TcpStream::connect("127.0.0.1:5500") {
        println!("Port 5500 is open");
    } else {
        println!("Port 5500 is closed");
    }
    print!("\n");
    println!("----------------------------------------------------------------")
}