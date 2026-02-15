use std::collections::HashMap;
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::sync::{Arc, Mutex};
use serde::{Serialize, Deserialize};

pub type PeerId = String;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Message {
    Handshake { version: String, peer_id: PeerId },
    Ping,
    Pong,
    GetPeers,
    PeersList(Vec<PeerId>),
    NewTransaction(Vec<u8>),
    NewBlock(Vec<u8>),
    GetBlock(String),
    BlockResponse(Vec<u8>),
}

pub struct P2PNetwork {
    peer_id: PeerId,
    listen_addr: String,
    peers: Arc<Mutex<HashMap<PeerId, TcpStream>>>,
    _message_handler: Arc<Mutex<dyn FnMut(PeerId, Message) + Send>>,
}

impl P2PNetwork {
    pub fn new(peer_id: PeerId, listen_addr: String) -> Self {
        Self {
            peer_id,
            listen_addr,
            peers: Arc::new(Mutex::new(HashMap::new())),
            _message_handler: Arc::new(Mutex::new(|_, _| {})),
        }
    }

    pub fn start(&self) -> Result<(), String> {
        let listener = TcpListener::bind(&self.listen_addr)
            .map_err(|e| format!("Failed to bind: {}", e))?;
        
        println!("P2P network listening on {}", self.listen_addr);
        
        let peers = Arc::clone(&self.peers);
        let peer_id = self.peer_id.clone();
        
        std::thread::spawn(move || {
            for stream in listener.incoming() {
                match stream {
                    Ok(stream) => {
                        let peers = Arc::clone(&peers);
                        let peer_id = peer_id.clone();
                        std::thread::spawn(move || {
                            Self::handle_connection(stream, peers, peer_id);
                        });
                    }
                    Err(e) => eprintln!("Connection failed: {}", e),
                }
            }
        });
        
        Ok(())
    }

    pub fn connect_to_peer(&self, addr: &str) -> Result<(), String> {
        let stream = TcpStream::connect(addr)
            .map_err(|e| format!("Failed to connect to {}: {}", addr, e))?;
        
        let peer_id = format!("peer_{}", addr);
        
        // Send handshake
        let handshake = Message::Handshake {
            version: "0.7.0".to_string(),
            peer_id: self.peer_id.clone(),
        };
        Self::send_message(&mut stream.try_clone().unwrap(), &handshake)?;
        
        self.peers.lock().unwrap().insert(peer_id, stream);
        println!("Connected to peer: {}", addr);
        
        Ok(())
    }

    fn handle_connection(mut stream: TcpStream, peers: Arc<Mutex<HashMap<PeerId, TcpStream>>>, _local_peer_id: PeerId) {
        let mut buffer = [0u8; 4096];
        
        loop {
            match stream.read(&mut buffer) {
                Ok(n) if n > 0 => {
                    if let Ok(msg) = bincode::deserialize::<Message>(&buffer[..n]) {
                        match msg {
                            Message::Handshake { version, peer_id } => {
                                println!("Handshake from {} (version: {})", peer_id, version);
                                peers.lock().unwrap().insert(peer_id, stream.try_clone().unwrap());
                            }
                            Message::Ping => {
                                Self::send_message(&mut stream, &Message::Pong).ok();
                            }
                            Message::NewTransaction(tx) => {
                                println!("Received transaction: {} bytes", tx.len());
                            }
                            Message::NewBlock(block) => {
                                println!("Received block: {} bytes", block.len());
                            }
                            _ => {}
                        }
                    }
                }
                Ok(_) => break,
                Err(e) => {
                    eprintln!("Read error: {}", e);
                    break;
                }
            }
        }
    }

    fn send_message(stream: &mut TcpStream, msg: &Message) -> Result<(), String> {
        let encoded = bincode::serialize(msg)
            .map_err(|e| format!("Serialize error: {}", e))?;
        stream.write_all(&encoded)
            .map_err(|e| format!("Write error: {}", e))?;
        stream.flush().map_err(|e| format!("Flush error: {}", e))?;
        Ok(())
    }

    pub fn broadcast(&self, msg: Message) -> Result<(), String> {
        let peers = self.peers.lock().unwrap();
        for (peer_id, stream) in peers.iter() {
            let mut stream = stream.try_clone()
                .map_err(|e| format!("Clone stream error: {}", e))?;
            if let Err(e) = Self::send_message(&mut stream, &msg) {
                eprintln!("Failed to send to {}: {}", peer_id, e);
            }
        }
        Ok(())
    }

    pub fn get_peer_count(&self) -> usize {
        self.peers.lock().unwrap().len()
    }
}
