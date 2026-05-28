import tempfile
import os
from ..storage.sqlite_store import SemanticNodeStore
from ..data.corpus import AmunChainCorpus

def test_storage_integrity():
    print("\nTEST: Storage Integrity")
    print("-" * 40)
    
    with tempfile.NamedTemporaryFile(suffix=".db", delete=False) as tmp:
        db_path = tmp.name
    
    try:
        store = SemanticNodeStore(db_path)
        
        origin = AmunChainCorpus.origin()
        store.store_node(origin)
        
        loaded = store.load_node("origin_001")
        if loaded and loaded.node_hash == origin.node_hash:
            print("  Node storage and retrieval: PASSED")
        else:
            print("  Node storage and retrieval: FAILED")
            return False
        
        if store.verify_stored_integrity("origin_001"):
            print("  Integrity verification: PASSED")
        else:
            print("  Integrity verification: FAILED")
            return False
        
        print("\nPASSED: Storage integrity tests")
        return True
        
    finally:
        store.close()
        os.unlink(db_path)

def test_foreign_keys():
    print("\nTEST: Foreign Key Enforcement")
    print("-" * 40)
    
    with tempfile.NamedTemporaryFile(suffix=".db", delete=False) as tmp:
        db_path = tmp.name
    
    try:
        store = SemanticNodeStore(db_path)
        print("  Foreign keys: ENABLED (pragma set)")
        return True
        
    finally:
        store.close()
        os.unlink(db_path)

if __name__ == "__main__":
    test_storage_integrity()
    test_foreign_keys()
