use std::process::Command;

fn main() {
    // Capture full rustc version diagnostics
    let rustc_verbose = Command::new("rustc")
        .arg("-Vv")
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_else(|_| "unknown".to_string());

    // Extract key fields
    let host = rustc_verbose
        .lines()
        .find(|l| l.starts_with("host:"))
        .map(|l| l.replace("host: ", ""))
        .unwrap_or_else(|| "unknown".to_string());

    let commit_hash = rustc_verbose
        .lines()
        .find(|l| l.starts_with("commit-hash:"))
        .map(|l| l.replace("commit-hash: ", ""))
        .unwrap_or_else(|| "unknown".to_string());

    let llvm_version = rustc_verbose
        .lines()
        .find(|l| l.starts_with("LLVM version:"))
        .map(|l| l.replace("LLVM version: ", ""))
        .unwrap_or_else(|| "unknown".to_string());

    println!("cargo:rustc-env=RUSTC_HOST={}", host);
    println!("cargo:rustc-env=RUSTC_COMMIT={}", commit_hash);
    println!("cargo:rustc-env=RUSTC_LLVM={}", llvm_version);
    println!("cargo:rustc-env=RUSTC_FULL={}", rustc_verbose.replace('\n', " | "));
}
