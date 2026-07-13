use amun_canonical_codec::{CanonicalReader, CanonicalWriter};

pub const SNAPSHOT_VERSION_V1: u32 = 1;
pub const SNAPSHOT_MAGIC: &[u8; 4] = b"AMSN";
pub const MAX_CHUNK_SIZE: u64 = 16 * 1024 * 1024;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SnapshotHeader {
    pub magic: [u8; 4],
    pub snapshot_version: u32,
    pub protocol_version: u32,
    pub state_root: [u8; 32],
    pub canonical_empty_root: [u8; 32],
    pub chunk_count: u64,
    pub total_nodes: u64,
    pub total_size: u64,
    pub created_at_epoch: u64,
    pub created_at_generation: u64,
    pub constitutional_hash: [u8; 32],
}

impl SnapshotHeader {
    pub fn encode(&self) -> Vec<u8> {
        let mut w = CanonicalWriter::new();
        w.write_u8(self.magic[0]);
        w.write_u8(self.magic[1]);
        w.write_u8(self.magic[2]);
        w.write_u8(self.magic[3]);
        w.write_u32(self.snapshot_version);
        w.write_u32(self.protocol_version);
        w.write_hash(&self.state_root);
        w.write_hash(&self.canonical_empty_root);
        w.write_u64(self.chunk_count);
        w.write_u64(self.total_nodes);
        w.write_u64(self.total_size);
        w.write_u64(self.created_at_epoch);
        w.write_u64(self.created_at_generation);
        w.write_hash(&self.constitutional_hash);
        w.into_bytes()
    }

    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut r = CanonicalReader::new(data);
        let m0 = r.read_u8()?;
        let m1 = r.read_u8()?;
        let m2 = r.read_u8()?;
        let m3 = r.read_u8()?;
        if [m0, m1, m2, m3] != *SNAPSHOT_MAGIC {
            return None;
        }
        let snapshot_version = r.read_u32()?;
        if snapshot_version != SNAPSHOT_VERSION_V1 {
            return None;
        }
        let protocol_version = r.read_u32()?;
        let state_root = r.read_hash()?;
        let canonical_empty_root = r.read_hash()?;
        let chunk_count = r.read_u64()?;
        let total_nodes = r.read_u64()?;
        let total_size = r.read_u64()?;
        let created_at_epoch = r.read_u64()?;
        let created_at_generation = r.read_u64()?;
        let constitutional_hash = r.read_hash()?;
        if !r.is_finished() {
            return None;
        }
        Some(Self {
            magic: *SNAPSHOT_MAGIC,
            snapshot_version,
            protocol_version,
            state_root,
            canonical_empty_root,
            chunk_count,
            total_nodes,
            total_size,
            created_at_epoch,
            created_at_generation,
            constitutional_hash,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SerializedNode {
    pub depth: usize,
    pub node_type: u8,
    pub node_hash: [u8; 32],
    pub data: Vec<u8>,
}

impl SerializedNode {
    pub fn encode(&self) -> Vec<u8> {
        let mut w = CanonicalWriter::new();
        w.write_u64(self.depth as u64);
        w.write_u8(self.node_type);
        w.write_hash(&self.node_hash);
        w.write_bytes(&self.data);
        w.into_bytes()
    }

    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut r = CanonicalReader::new(data);
        let depth = r.read_u64()? as usize;
        let node_type = r.read_u8()?;
        let node_hash = r.read_hash()?;
        let node_data = r.read_bytes()?;
        if !r.is_finished() {
            return None;
        }
        Some(Self {
            depth,
            node_type,
            node_hash,
            data: node_data,
        })
    }
}

pub struct SnapshotBuilder {
    nodes: Vec<SerializedNode>,
}

impl SnapshotBuilder {
    pub fn new() -> Self {
        Self { nodes: Vec::new() }
    }
    pub fn add_node(&mut self, node: SerializedNode) {
        self.nodes.push(node);
    }
    pub fn build(self) -> SnapshotData {
        SnapshotData { nodes: self.nodes }
    }
}

#[derive(Debug, Clone)]
pub struct SnapshotData {
    pub nodes: Vec<SerializedNode>,
}

impl SnapshotData {
    pub fn total_nodes(&self) -> u64 {
        self.nodes.len() as u64
    }
    pub fn total_size(&self) -> u64 {
        self.nodes
            .iter()
            .map(|n| n.data.len() as u64 + 8 + 1 + 32 + 8)
            .sum()
    }
}

pub struct SnapshotReader {
    pub header: SnapshotHeader,
    pub nodes: Vec<SerializedNode>,
}

impl SnapshotReader {
    pub fn read(data: &[u8]) -> Option<Self> {
        let header = SnapshotHeader::decode(data)?;
        let header_size = header.encode().len();
        let mut nodes = Vec::with_capacity(header.total_nodes as usize);
        let remaining = &data[header_size..];
        let mut r = CanonicalReader::new(remaining);
        while !r.is_finished() {
            let node_bytes = r.read_bytes()?;
            if let Some(node) = SerializedNode::decode(&node_bytes) {
                nodes.push(node);
            }
        }
        Some(Self { header, nodes })
    }
}
