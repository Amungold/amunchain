import sqlite3
import json
from typing import Optional, List, Dict
from datetime import datetime
from ..models.interpretive_node import InterpretiveNode
from ..models.merkle_snapshot import MerkleSnapshot
from ..core.hash_utils import canonicalize, sha256_hash, compute_node_hash

class SemanticNodeStore:
    def __init__(self, db_path: str = "phase_80a.db"):
        self.db_path = db_path
        self.conn = None
        self._init_db()
    
    def _init_db(self):
        self.conn = sqlite3.connect(self.db_path)
        self.conn.execute("PRAGMA foreign_keys = ON")
        
        self.conn.execute("""
            CREATE TABLE IF NOT EXISTS nodes (
                node_id TEXT PRIMARY KEY,
                node_hash TEXT NOT NULL,
                parent_id TEXT,
                parent_hash TEXT,
                semantic_origin_hash TEXT NOT NULL,
                frozen_data TEXT NOT NULL,
                created_at TIMESTAMP,
                FOREIGN KEY (parent_id) REFERENCES nodes(node_id)
            )
        """)
        
        self.conn.execute("""
            CREATE TABLE IF NOT EXISTS runtime_metrics (
                node_id TEXT PRIMARY KEY,
                aliveness TEXT,
                constraint_influence REAL,
                semantic_weight REAL,
                participation_activity REAL,
                metabolic_rate REAL,
                updated_at TIMESTAMP,
                FOREIGN KEY (node_id) REFERENCES nodes(node_id)
            )
        """)
        
        self.conn.execute("""
            CREATE TABLE IF NOT EXISTS snapshots (
                snapshot_id TEXT PRIMARY KEY,
                block_height INTEGER NOT NULL,
                merkle_root TEXT NOT NULL,
                node_count INTEGER NOT NULL,
                snapshot_data TEXT NOT NULL,
                created_at TIMESTAMP
            )
        """)
        
        self.conn.execute("""
            CREATE TABLE IF NOT EXISTS audit_log (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                node_id TEXT,
                operation TEXT,
                timestamp TIMESTAMP,
                details TEXT
            )
        """)
        
        self.conn.commit()
    
    def store_node(self, node: InterpretiveNode) -> bool:
        frozen = node.freeze()
        canonical_json = canonicalize(frozen)
        
        try:
            self.conn.execute("""
                INSERT OR REPLACE INTO nodes 
                (node_id, node_hash, parent_id, parent_hash, semantic_origin_hash, frozen_data, created_at)
                VALUES (?, ?, ?, ?, ?, ?, ?)
            """, (node.node_id, node.node_hash, node.parent_id, node.parent_hash,
                  node.semantic_origin_hash, canonical_json, node.created_at))
            
            self.conn.execute("""
                INSERT INTO audit_log (node_id, operation, timestamp, details)
                VALUES (?, ?, ?, ?)
            """, (node.node_id, "STORE", datetime.utcnow().isoformat() + "Z", "Node stored"))
            
            self.conn.commit()
            return True
        except Exception as e:
            print(f"Error storing node {node.node_id}: {e}")
            return False
    
    def store_runtime_metrics(self, node_id: str, aliveness: str, 
                               constraint_influence: float, semantic_weight: float,
                               participation_activity: float, metabolic_rate: float) -> bool:
        try:
            self.conn.execute("""
                INSERT OR REPLACE INTO runtime_metrics 
                (node_id, aliveness, constraint_influence, semantic_weight, 
                 participation_activity, metabolic_rate, updated_at)
                VALUES (?, ?, ?, ?, ?, ?, ?)
            """, (node_id, aliveness, constraint_influence, semantic_weight,
                  participation_activity, metabolic_rate, datetime.utcnow().isoformat() + "Z"))
            self.conn.commit()
            return True
        except Exception as e:
            print(f"Error storing metrics for {node_id}: {e}")
            return False
    
    def store_snapshot(self, snapshot: MerkleSnapshot) -> bool:
        try:
            snapshot_data = canonicalize(snapshot.to_dict())
            self.conn.execute("""
                INSERT OR REPLACE INTO snapshots 
                (snapshot_id, block_height, merkle_root, node_count, snapshot_data, created_at)
                VALUES (?, ?, ?, ?, ?, ?)
            """, (snapshot.snapshot_id, snapshot.block_height, snapshot.merkle_root,
                  snapshot.node_count, snapshot_data, snapshot.created_at))
            self.conn.commit()
            return True
        except Exception as e:
            print(f"Error storing snapshot: {e}")
            return False
    
    def load_node(self, node_id: str) -> Optional[InterpretiveNode]:
        cursor = self.conn.execute("""
            SELECT frozen_data, node_hash FROM nodes WHERE node_id = ?
        """, (node_id,))
        row = cursor.fetchone()
        
        if not row:
            return None
        
        frozen_json, stored_hash = row
        frozen = json.loads(frozen_json)
        
        reconstructed = InterpretiveNode.from_frozen(frozen)
        computed_hash = compute_node_hash(reconstructed)
        
        if computed_hash != stored_hash:
            print(f"WARNING: Integrity check failed for node {node_id}")
            print(f"  stored: {stored_hash}")
            print(f"  computed: {computed_hash}")
            return None
        
        return reconstructed
    
    def load_snapshot(self, snapshot_id: str) -> Optional[MerkleSnapshot]:
        cursor = self.conn.execute("""
            SELECT snapshot_data FROM snapshots WHERE snapshot_id = ?
        """, (snapshot_id,))
        row = cursor.fetchone()
        
        if not row:
            return None
        
        snapshot_data = json.loads(row[0])
        return MerkleSnapshot(
            snapshot_id=snapshot_data["snapshot_id"],
            block_height=snapshot_data["block_height"],
            merkle_root=snapshot_data["merkle_root"],
            node_count=snapshot_data["node_count"],
            node_hashes=snapshot_data["node_hashes"],
            created_at=snapshot_data["created_at"],
            parent_snapshot_hash=snapshot_data.get("parent_snapshot_hash"),
            signature=snapshot_data.get("signature")
        )
    
    def verify_stored_integrity(self, node_id: str) -> bool:
        cursor = self.conn.execute("""
            SELECT frozen_data, node_hash FROM nodes WHERE node_id = ?
        """, (node_id,))
        row = cursor.fetchone()
        
        if not row:
            return False
        
        frozen_json, stored_hash = row
        frozen = json.loads(frozen_json)
        
        reconstructed = InterpretiveNode.from_frozen(frozen)
        computed_hash = compute_node_hash(reconstructed)
        
        return computed_hash == stored_hash
    
    def get_lineage(self, node_id: str) -> List[str]:
        lineage = []
        current = node_id
        
        while current:
            lineage.append(current)
            cursor = self.conn.execute("SELECT parent_id FROM nodes WHERE node_id = ?", (current,))
            row = cursor.fetchone()
            if not row or not row[0]:
                break
            current = row[0]
        
        return list(reversed(lineage))
    
    def get_all_node_ids(self) -> List[str]:
        cursor = self.conn.execute("SELECT node_id FROM nodes")
        return [row[0] for row in cursor.fetchall()]
    
    def close(self):
        if self.conn:
            self.conn.close()
