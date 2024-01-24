
use std::net::{Ipv4Addr, SocketAddrV4, TcpListener, TcpStream, Shutdown};
use std::process::{Command, exit};

fn main() {
   
    let bind_ip = "127.0.0.1";
    let bind_post = 1234; 
    let ip = bind_ip.parse::<Ipv4Addr>();
    //catching error
    let address = match ip{
        Ok(i) => i,
        Err(e) => {println!("{}", e); exit(0)}
    };


}
