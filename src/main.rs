use std::io::ErrorKind;
use std::collections::VecDeque;
use std::collections::HashMap;
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::Read;
use std::io::Write;

enum Event {
    Accept(TcpStream),
    Read(TcpStream),
    Write(TcpStream),
    StreamError(ErrorKind),
}

struct Demux {
    listener: TcpListener,
    toread: HashMap<String,TcpStream>,
    towrite: HashMap<String,TcpStream>,
}

impl Demux {
    pub fn new(listener:TcpListener) -> Self  {
        listener.set_nonblocking(true)
            .expect("Cannot set TcpListener to non blocking");

        Self {
            listener,
            toread: HashMap::new(),
            towrite: HashMap::new(),
        }
    }

    pub fn accept(&mut self) -> Option<Event> {
        let res = self.listener.accept();

        match res  {
            Ok((peer,_)) => {
                return Some( Event::Accept(peer) );
            },
            Err(e) if e.kind() != ErrorKind::WouldBlock => {
                return Some( Event::StreamError(e.kind()) );
            },
            _ => {
                return None;
            }
        }

    }

    pub fn read(&mut self, stream: TcpStream) {
        let key = stream.peer_addr().unwrap().to_string();

        self.toread.insert( key, stream );
    }
    
    pub fn write(&mut self, stream: TcpStream) {
        let key = stream.peer_addr().unwrap().to_string();

        self.towrite.insert( key, stream );
    }

    pub fn dispatch(&mut self, queue:&mut VecDeque<Event>) {
        if let Some(ev) = self.accept() {
            queue.push_back( ev );
        }

        for (_addr,stream) in self.toread.drain() {
            queue.push_back( Event::Read(stream) );
        }

        for (_addr,stream) in self.towrite.drain() {
            queue.push_back( Event::Write(stream) );
        }
    }

}

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
