use std::io::Read;

const MAX_FRAME_SIZE: usize = 4 * 1024 * 1024;
const MAX_READ_BUFFER: usize = 8 * 1024 * 1024;
const COMPACT_THRESHOLD: usize = 64 * 1024;
const MAX_CONSECUTIVE_OVERSIZED: usize = 10;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecodeError {
    FrameTooLarge(usize),
    BufferOverflow,
    ReadError,
    TooManyOversizedFrames,
    ZeroLengthFrame,
}

#[derive(Default, Clone)]
pub struct CodecMetrics {
    pub frames_decoded: u64,
    pub bytes_decoded: u64,
    pub invalid_frames: u64,
    pub oversized_frames: u64,
    pub buffer_overflows: u64,
    pub buffer_compactions: u64,
    pub connections_rejected: u64,
}

pub struct FrameCodec {
    buffer: Vec<u8>,
    read_pos: usize,
    consecutive_oversized: usize,
    metrics: CodecMetrics,
}

impl FrameCodec {
    pub fn new() -> Self {
        Self {
            buffer: Vec::new(),
            read_pos: 0,
            consecutive_oversized: 0,
            metrics: CodecMetrics::default(),
        }
    }

    pub fn encode(data: &[u8]) -> crate::payload::Payload {
        let len = data.len() as u32;
        let mut framed = Vec::with_capacity(4 + data.len());
        framed.extend_from_slice(&len.to_be_bytes());
        framed.extend_from_slice(data);
        framed.into()
    }

    fn compact(&mut self) {
        if self.read_pos > 0 {
            self.buffer.copy_within(self.read_pos.., 0);
            self.buffer.truncate(self.buffer.len() - self.read_pos);
            self.read_pos = 0;
            self.metrics.buffer_compactions += 1;
        }
    }

    pub fn decode(
        &mut self,
        stream: &mut impl Read,
    ) -> Result<Vec<crate::payload::Payload>, DecodeError> {
        let mut frames = Vec::new();
        let mut buf = [0u8; 4096];

        loop {
            match stream.read(&mut buf) {
                Ok(0) => break,
                Ok(n) => {
                    self.buffer.extend_from_slice(&buf[..n]);
                    self.metrics.bytes_decoded += n as u64;

                    if self.buffer.len() - self.read_pos > MAX_READ_BUFFER {
                        self.metrics.buffer_overflows += 1;
                        self.metrics.connections_rejected += 1;
                        return Err(DecodeError::BufferOverflow);
                    }
                }
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => break,
                Err(_) => return Err(DecodeError::ReadError),
            }
        }

        loop {
            let available = self.buffer.len() - self.read_pos;
            if available < 4 {
                break;
            }

            let len = u32::from_be_bytes([
                self.buffer[self.read_pos],
                self.buffer[self.read_pos + 1],
                self.buffer[self.read_pos + 2],
                self.buffer[self.read_pos + 3],
            ]) as usize;

            // Zero-length frames are protocol violations
            if len == 0 {
                self.metrics.invalid_frames += 1;
                self.read_pos += 4;
                self.consecutive_oversized += 1;
                if self.consecutive_oversized > MAX_CONSECUTIVE_OVERSIZED {
                    self.metrics.connections_rejected += 1;
                    return Err(DecodeError::ZeroLengthFrame);
                }
                continue;
            }

            if len > MAX_FRAME_SIZE {
                self.metrics.oversized_frames += 1;
                self.consecutive_oversized += 1;
                self.read_pos += 4;

                if available >= 4 + len {
                    self.read_pos += len;
                } else {
                    self.metrics.connections_rejected += 1;
                    return Err(DecodeError::FrameTooLarge(len));
                }

                if self.consecutive_oversized > MAX_CONSECUTIVE_OVERSIZED {
                    self.metrics.connections_rejected += 1;
                    return Err(DecodeError::TooManyOversizedFrames);
                }
                continue;
            }

            self.consecutive_oversized = 0;

            if available < 4 + len {
                break;
            }

            let frame_start = self.read_pos + 4;
            let frame_end = frame_start + len;
            frames.push(crate::payload::Payload::from(
                self.buffer[frame_start..frame_end].to_vec(),
            ));
            self.metrics.frames_decoded += 1;
            self.read_pos = frame_end;

            if self.read_pos > COMPACT_THRESHOLD {
                self.compact();
            }
        }

        if self.read_pos > COMPACT_THRESHOLD {
            self.compact();
        }

        Ok(frames)
    }

    pub fn metrics(&self) -> &CodecMetrics {
        &self.metrics
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_partial_frame_reassembly() {
        let mut codec = FrameCodec::new();
        let data = b"hello world";
        let framed = FrameCodec::encode(data);

        let mut cursor = Cursor::new(&framed[..3]);
        let result = codec.decode(&mut cursor).unwrap();
        assert!(result.is_empty());

        let mut cursor = Cursor::new(&framed[3..]);
        let result = codec.decode(&mut cursor).unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(&result[0], &bytes::Bytes::from_static(data));
    }

    #[test]
    fn test_oversized_frame_rejected() {
        let mut codec = FrameCodec::new();
        let oversized_len = (5 * 1024 * 1024u32).to_be_bytes();
        let mut data = Vec::new();
        data.extend_from_slice(&oversized_len);
        data.extend_from_slice(&[0u8; 100]);

        let mut cursor = Cursor::new(&data);
        let result = codec.decode(&mut cursor);
        assert!(matches!(result, Err(DecodeError::FrameTooLarge(_))));
    }

    #[test]
    fn test_multiple_frames() {
        let mut codec = FrameCodec::new();
        let f1 = FrameCodec::encode(b"first");
        let f2 = FrameCodec::encode(b"second");
        let f3 = FrameCodec::encode(b"third");

        let mut combined = Vec::new();
        combined.extend_from_slice(&f1);
        combined.extend_from_slice(&f2);
        combined.extend_from_slice(&f3);

        let mut cursor = Cursor::new(&combined);
        let result = codec.decode(&mut cursor).unwrap();
        assert_eq!(result.len(), 3);
    }

    #[test]
    fn test_slow_header_attack() {
        let mut codec = FrameCodec::new();
        // Send only 1 byte at a time - can never form a complete 4-byte header
        // This tests protection against slow header attacks where an attacker
        // dribbles bytes to consume memory without completing a frame
        for _ in 0..10 {
            let mut cursor = Cursor::new(&[0x00]);
            let result = codec.decode(&mut cursor).unwrap();
            assert!(result.is_empty(), "1 byte cannot form a header");
        }
        // Buffer has 10 bytes (< 4 needed for header) so no overflow
        assert!(codec.metrics().buffer_overflows == 0);
    }

    #[test]
    fn test_zero_length_frame_rejected() {
        let mut codec = FrameCodec::new();
        // Send 4 zero bytes = frame length 0
        let mut cursor = Cursor::new(&[0x00, 0x00, 0x00, 0x00]);
        let result = codec.decode(&mut cursor).unwrap();
        assert!(result.is_empty(), "Zero-length frames should be skipped");
        assert!(codec.metrics().invalid_frames > 0);
    }
}
