// Copyright (c) 2026 Amungold Global
// SPDX-License-Identifier: AGPL-3.0-or-later
// Licensed under the GNU AGPLv3 with Constitutional Sovereignty Addendum.

use blake3::Hasher;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct SpecificationHashes {
    pub semantic_hash: String,
    pub documentary_hash: String,
    pub combined_hash: String,
}

/// Computes the combined specification hash from all source files.
pub fn compute_specification_hash(root: &Path) -> Result<String, String> {
    let hashes = compute_all_hashes(root)?;
    Ok(hashes.combined_hash)
}

/// Computes semantic, documentary, and combined hashes.
pub fn compute_all_hashes(root: &Path) -> Result<SpecificationHashes, String> {
    let semantic_exts = ["rs", "toml", "lock"];
    let documentary_exts = ["md", "txt", "yaml"];

    let semantic_hash = compute_hash_for_extensions(root, &semantic_exts)?;
    let documentary_hash = compute_hash_for_extensions(root, &documentary_exts)?;

    let mut all_exts = semantic_exts.to_vec();
    all_exts.extend_from_slice(&documentary_exts);
    let combined_hash = compute_hash_for_extensions(root, &all_exts)?;

    Ok(SpecificationHashes {
        semantic_hash,
        documentary_hash,
        combined_hash,
    })
}

fn compute_hash_for_extensions(root: &Path, extensions: &[&str]) -> Result<String, String> {
    let mut files = Vec::new();
    collect_files_with_extensions(root, extensions, &mut files)?;
    files.sort();

    let mut hasher = Hasher::new();
    for path in &files {
        let content = fs::read_to_string(path)
            .map_err(|e| format!("Failed to read {}: {}", path.display(), e))?;
        // Normalize line endings before hashing to avoid OS-specific drift.
        let normalized = content.replace("\r\n", "\n");
        hasher.update(normalized.as_bytes());
    }

    Ok(hasher.finalize().to_hex().to_string())
}

fn collect_files_with_extensions(
    dir: &Path,
    extensions: &[&str],
    files: &mut Vec<std::path::PathBuf>,
) -> Result<(), String> {
    if !dir.exists() {
        return Ok(());
    }
    let entries = fs::read_dir(dir)
        .map_err(|e| format!("Failed to read dir {}: {}", dir.display(), e))?;

    for entry in entries {
        let entry = entry.map_err(|e| format!("Entry error: {}", e))?;
        let path = entry.path();
        if path.is_dir() {
            collect_files_with_extensions(&path, extensions, files)?;
        } else if let Some(ext) = path.extension() {
            if extensions.contains(&ext.to_str().unwrap_or("")) {
                files.push(path);
            }
        }
    }
    Ok(())
}
