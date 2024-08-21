use std::io::ErrorKind;
use std::collections::VecDeque;
use std::net::TcpListener;
use std::io::Read;
use std::io::Write;

mod message;
mod demux;

use demux::*;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8000")
        .expect("Cannot bind listener");

    println!("Listening to 127.0.0.1:8000");

    let mut demux = Demux::new(listener);

    let mut queue: VecDeque<Event> = VecDeque::new();

    loop {
        
        demux.dispatch( &mut queue );

        if let Some(ev) = queue.pop_front() {
            match ev {
                Event::Accept(peer) => {
                    let key = peer.peer_addr().unwrap();
                    println!("Accepting peer: {}", key.to_string() );
                    peer.set_nonblocking( true ).expect("Cannot set TcpStream to non blocking");
                    peer.set_read_timeout( Some(std::time::Duration::from_secs(1)) ).unwrap();
                    peer.set_write_timeout( Some(std::time::Duration::from_secs(1)) ).unwrap();
                    demux.read( peer );
                },
                Event::Read(mut peer) => {
                    print!("Reading from {:?}", peer.peer_addr() );

                    let mut buffer:[u8; 256] = [0; 256];
                    match peer.read(&mut buffer) {
                        Ok(bytes) => {
                            print!( " bytes: {}: ", bytes );
                            let payload = std::str::from_utf8( &buffer[..bytes] ).unwrap();
                            println!("{}", payload);
                            demux.write( peer );
                        },
                        Err(e) if e.kind() != ErrorKind::WouldBlock => {
                            println!(" Error while read: ");
                        },
                        _ => {}
                    }
                },
                Event::Write(mut peer) => {
                    print!("Write to {:?}", peer.peer_addr() );

                    let buffer = "Thak You, Bye\n".as_bytes();
                    match peer.write(&buffer) {
                        Ok(bytes) => {
                            println!( " bytes: {}: ", bytes );
                        },
                        Err(e) if e.kind() != ErrorKind::WouldBlock => {
                            println!(" Error while write: ");
                        },
                        _ => {}
                    }
                },
                Event::StreamError(err) => {
                    println!("Error: {:?}", err);
                }
            }
        }
    }
}
