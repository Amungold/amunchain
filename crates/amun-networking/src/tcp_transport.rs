use crate::envelope::Envelope;
use crate::transport_trait::Transport;
use std::collections::VecDeque;
use std::io::{ErrorKind, Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};

/// Real TCP network transport with length-prefixed message framing.
///
/// Wire format: [u32 length in big-endian][JSON-encoded Envelope payload]
///
/// This ensures complete message boundaries over TCP streams.
pub struct TcpTransport {
    listener: Option<TcpListener>,
    streams: Vec<TcpStream>,
    outbox: VecDeque<Envelope>,
    inbox: VecDeque<Envelope>,
    address: SocketAddr,
    read_buffers: Vec<Vec<u8>>,
    pending_connections: Vec<SocketAddr>,
}

impl TcpTransport {
    pub fn new(address: SocketAddr) -> Self {
        Self {
            listener: None,
            streams: Vec::new(),
            outbox: VecDeque::new(),
            inbox: VecDeque::new(),
            address,
            read_buffers: Vec::new(),
            pending_connections: Vec::new(),
        }
    }

    /// Bind to the local address and start listening for connections.
    pub fn bind(&mut self) -> std::io::Result<()> {
        let listener = TcpListener::bind(self.address)?;
        listener.set_nonblocking(true)?;
        self.listener = Some(listener);
        Ok(())
    }

    /// Queue a connection attempt to a remote peer.
    /// Actual connection happens during tick().
    pub fn connect_to(&mut self, address: SocketAddr) {
        self.pending_connections.push(address);
    }

    /// Accept an incoming connection. Non-blocking.
    fn accept_one(&mut self) -> std::io::Result<()> {
        if let Some(ref listener) = self.listener {
            match listener.accept() {
                Ok((stream, _addr)) => {
                    stream.set_nonblocking(true)?;
                    self.streams.push(stream);
                    self.read_buffers.push(Vec::new());
                }
                Err(e) if e.kind() == ErrorKind::WouldBlock => {}
                Err(e) => return Err(e),
            }
        }
        Ok(())
    }

    /// Attempt pending outbound connections. Non-blocking.
    fn connect_pending(&mut self) {
        let addresses: Vec<_> = self.pending_connections.drain(..).collect();
        for addr in addresses {
            match TcpStream::connect(addr) {
                Ok(stream) => {
                    let _ = stream.set_nonblocking(true);
                    self.streams.push(stream);
                    self.read_buffers.push(Vec::new());
                }
                Err(_) => {
                    // Re-queue for next tick
                    self.pending_connections.push(addr);
                }
            }
        }
    }

    /// Read available data from all connected streams.
    /// Parses length-prefixed messages: [u32 BE length][payload].
    fn read_all(&mut self) {
        let mut buf = [0u8; 65536];
        for (i, stream) in self.streams.iter_mut().enumerate() {
            match stream.read(&mut buf) {
                Ok(n) if n > 0 => {
                    self.read_buffers[i].extend_from_slice(&buf[..n]);
                    while self.read_buffers[i].len() >= 4 {
                        let len = u32::from_be_bytes([
                            self.read_buffers[i][0],
                            self.read_buffers[i][1],
                            self.read_buffers[i][2],
                            self.read_buffers[i][3],
                        ]) as usize;

                        // Sanity check: reject excessively large messages
                        if len > 10_000_000 {
                            self.read_buffers[i].clear();
                            break;
                        }

                        if self.read_buffers[i].len() >= 4 + len {
                            let payload = self.read_buffers[i][4..4 + len].to_vec();
                            self.read_buffers[i].drain(..4 + len);
                            if let Ok(envelope) = serde_json::from_slice::<Envelope>(&payload) {
                                self.inbox.push_back(envelope);
                            }
                        } else {
                            break;
                        }
                    }
                }
                Err(e) if e.kind() == ErrorKind::WouldBlock => {}
                _ => {}
            }
        }
    }

    /// Write all outbox messages to all connected streams.
    fn write_all(&mut self) {
        while let Some(envelope) = self.outbox.pop_front() {
            if let Ok(payload) = serde_json::to_vec(&envelope) {
                let len = payload.len() as u32;
                let mut framed = Vec::with_capacity(4 + payload.len());
                framed.extend_from_slice(&len.to_be_bytes());
                framed.extend_from_slice(&payload);
                // Write to all connected streams (broadcast)
                let mut dead_indices = Vec::new();
                for (i, stream) in self.streams.iter_mut().enumerate() {
                    if stream.write_all(&framed).is_err() {
                        dead_indices.push(i);
                    }
                }
                // Remove dead connections in reverse order
                for i in dead_indices.into_iter().rev() {
                    self.streams.remove(i);
                    self.read_buffers.remove(i);
                }
            }
        }
    }
}

impl Transport for TcpTransport {
    fn send(&mut self, envelope: Envelope) {
        self.outbox.push_back(envelope);
    }

    fn next_outgoing(&mut self) -> Option<Envelope> {
        self.outbox.pop_front()
    }

    fn deliver(&mut self, envelope: Envelope) {
        self.inbox.push_back(envelope);
    }

    fn next_incoming(&mut self) -> Option<Envelope> {
        self.inbox.pop_front()
    }

    fn tick(&mut self, _elapsed_ms: u64) {
        let _ = self.accept_one();
        self.connect_pending();
        self.read_all();
        self.write_all();
    }
}
