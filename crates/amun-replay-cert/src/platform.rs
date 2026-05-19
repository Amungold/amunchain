#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PlatformFingerprint {
    pub arch: String,
    pub os: String,
    pub endian: String,
    pub rustc_host: String,
    pub rustc_commit: String,
    pub rustc_llvm: String,
    pub opt_level: String,
}

impl PlatformFingerprint {
    pub fn current() -> Self {
        Self {
            arch: std::env::consts::ARCH.to_string(),
            os: std::env::consts::OS.to_string(),
            endian: if cfg!(target_endian = "little") { "little" } else { "big" }.to_string(),
            rustc_host: env!("RUSTC_HOST").to_string(),
            rustc_commit: env!("RUSTC_COMMIT").to_string(),
            rustc_llvm: env!("RUSTC_LLVM").to_string(),
            opt_level: if cfg!(debug_assertions) { "debug" } else { "release" }.to_string(),
        }
    }

    pub fn name(&self) -> String {
        format!("{}-{}-{}-{}", self.arch, self.os, self.opt_level, &self.rustc_commit[..8.min(self.rustc_commit.len())])
    }

    pub fn tag(&self) -> Vec<u8> {
        self.name().into_bytes()
    }
}
