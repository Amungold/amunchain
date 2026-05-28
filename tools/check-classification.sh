#!/bin/bash
# Check that crates follow the classification

echo "Crate Classification Check"
echo "=========================="
echo ""

# List all crates
for crate in crates/amun-*/; do
    name=$(basename "$crate")
    
    # Check naming convention
    if [[ ! "$name" =~ ^amun-[a-z]+(-[a-z]+)*$ ]]; then
        echo "❌ $name: Invalid naming (use hyphens, no underscores)"
    else
        echo "✅ $name: Naming OK"
    fi
done
