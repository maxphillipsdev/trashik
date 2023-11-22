use std::io::{Cursor, Read, Result};
use std::net::{TcpListener, TcpStream};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:25565").expect("Failed to bind to address");
    println!("Listening for Minecraft TCP connections on port 25565...");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {:?}", stream.peer_addr().unwrap());
                handle_client(stream);
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}

fn handle_client(mut stream: TcpStream) {
    let handshake = HandshakeRequest::read(&mut stream);

    match handshake {
        Ok(handshake) => {
            println!("Handshake result: {:#?}", handshake);
        }
        Err(err) => {
            println!("Ahhh shit {}", err);
        }
    }
}

#[derive(Debug)]
struct HandshakeRequest {
    length: i32,
    packet_id: i32,
    protocol_version: i32,
    server_address: String,
}

impl HandshakeRequest {
    fn read(stream: &mut TcpStream) -> Result<Self> {
        // get the length
        let length = read_var_int(stream)?;

        // get the raw data as a cursor
        let mut raw_data = vec![0u8; length as usize];
        stream.read_exact(&mut raw_data)?;
        let mut raw_data_cursor = Cursor::new(raw_data);

        // Get all the other parts
        let packet_id = read_var_int(&mut raw_data_cursor)?;

        let protocol_version = read_var_int(&mut raw_data_cursor)?;
        let server_address = read_string(&mut raw_data_cursor, 255)?;

        Ok(Self {
            length,
            packet_id,
            protocol_version,
            server_address,
        })
    }
}

fn read_var_int<T>(stream: &mut T) -> Result<i32>
where
    T: Read,
{
    let mut result = 0;
    let mut shift = 0;
    let mut index = 0;

    loop {
        let mut buf = [0u8; 1];
        stream.read_exact(&mut buf)?;
        let byte = buf[0];
        result |= ((byte as i32) & 0x7F) << shift;
        shift += 7;
        index += 1;

        if (byte & 0x80) == 0 {
            break;
        }
    }

    Ok(result)
}

fn read_string<T>(stream: &mut T, max_size: i32) -> Result<String>
where
    T: Read,
{
    let length = read_var_int(stream)?;

    // if length > max_size {
    // TODO: error handle lol
    // }

    let mut bytes = vec![0u8; length as usize];
    stream.read_exact(&mut bytes);

    Ok(String::from_utf8_lossy(&bytes).to_string())
}
