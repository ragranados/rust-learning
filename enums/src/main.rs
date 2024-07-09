// #![allow(dead_code)]

enum IpAddr {
    V4(String),
    V6(String),
}

impl IpAddr {
    fn print(&self) {
        match self {
            IpAddr::V4(ip) => println!("Ip 4: {ip}"),
            IpAddr::V6(ip) => println!("Ip 6: {ip}"),
            // _ => println!("Ninguno")
        }
    }
}

fn main() {
    let home = IpAddr::V4(String::from("127.0.0.1"));
    home.print();

    let home = IpAddr::V6(String::from("::1"));
    home.print();
}
