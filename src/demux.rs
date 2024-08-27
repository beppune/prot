use std::net::{ TcpStream, TcpListener };
use std::collections::HashMap;
use std::io::ErrorKind;
use std::collections::VecDeque;

pub enum Event {
    Accept(TcpStream),
    Read(TcpStream),
    Write(TcpStream),
    StreamError(ErrorKind),
}

pub struct Demux {
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

#[cfg(test)]
mod demuxtest {
    use super::*;

    #[test]
    pub fn mytest() {
        assert!(true);
    }
}







