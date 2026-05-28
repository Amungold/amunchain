#!/usr/bin/env python3
"""
Generate canonical transcript using raw integers.
NO FLOATING POINT in transcript.
"""

import json
import hashlib
import sys
sys.path.insert(0, '/root/projects/amunchain/amunchain')

from phase_80a.math.ops import to_fixed, F_sqrt, F_exp

def generate_transcript():
    """Generate canonical transcript with raw integer inputs/outputs."""
    
    # Test vectors in raw fixed-point (SCALE = 1_000_000)
    sqrt_inputs_raw = [
        10000,      # 0.01
        250000,     # 0.25
        500000,     # 0.5
        1000000,    # 1.0
        2000000,    # 2.0
        4000000,    # 4.0
        9000000,    # 9.0
        16000000    # 16.0
    ]
    
    exp_inputs_raw = [
        -2000000,   # -2.0
        -1000000,   # -1.0
        -500000,    # -0.5
        0,          # 0.0
        500000,     # 0.5
        1000000,    # 1.0
        2000000     # 2.0
    ]
    
    operations = []
    hasher = hashlib.sha256()
    
    for raw_input in sqrt_inputs_raw:
        result_raw = F_sqrt(raw_input)
        operations.append({
            "op": "sqrt",
            "input_raw": raw_input,
            "output_raw": result_raw
        })
        hasher.update(result_raw.to_bytes(8, 'big'))
    
    for raw_input in exp_inputs_raw:
        result_raw = F_exp(raw_input)
        operations.append({
            "op": "exp",
            "input_raw": raw_input,
            "output_raw": result_raw
        })
        hasher.update(result_raw.to_bytes(8, 'big'))
    
    transcript = {
        "version": "2.0.0",
        "description": "Canonical consensus math transcript",
        "scale": 1000000,
        "operations": operations,
        "final_hash": hasher.hexdigest()
    }
    
    output_path = "/root/projects/amunchain/amunchain/phase_80a/transcripts/canonical.json"
    with open(output_path, "w") as f:
        json.dump(transcript, f, indent=2)
    
    print(f"Canonical transcript generated:")
    print(f"  Path: {output_path}")
    print(f"  Operations: {len(operations)}")
    print(f"  Final hash: {transcript['final_hash']}")
    
    return transcript

if __name__ == "__main__":
    generate_transcript()
