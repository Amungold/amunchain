use blake3::Hasher;
use serde::{Deserialize, Serialize};

use crate::{ConstitutionalReport, ConstitutionalVerdict};

/// A self-contained, signed package containing all constitutional artifacts
/// required to independently verify the N47 results.
///
/// The package is frozen once created and carries a package hash that covers
/// all its contents. The signature binds the package to a constitutional authority.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicationPackage {
    pub package_id: String,
    pub version: String,
    pub generated_at: u64,

    pub report: ConstitutionalReport,
    pub verdicts: Vec<ConstitutionalVerdict>,

    /// A manifest of every contained artifact with its hash.
    pub manifest: PackageManifest,

    /// Cryptographic signature over the package hash.
    pub signature: Option<PackageSignature>,

    /// Hash of the entire package contents (excluding the signature itself).
    pub package_hash: String,

    /// Whether the package is frozen and cannot be modified.
    pub frozen: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageManifest {
    pub report_hash: String,
    pub verdict_hashes: Vec<String>,
    pub artifact_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageSignature {
    pub signer: String,
    pub key_id: String,
    pub algorithm: String,
    pub signature: String,
    pub signed_at: u64,
}

impl PublicationPackage {
    pub fn new(
        package_id: String,
        report: ConstitutionalReport,
        verdicts: Vec<ConstitutionalVerdict>,
        generated_at: u64,
    ) -> Self {
        let report_hash = Self::hash_report(&report);
        let verdict_hashes: Vec<String> = verdicts.iter().map(|v| v.verdict_hash.clone()).collect();
        let artifact_count = 1 + verdicts.len();

        let manifest = PackageManifest {
            report_hash,
            verdict_hashes,
            artifact_count,
        };

        let mut pkg = Self {
            package_id,
            version: "1.0".into(),
            generated_at,
            report,
            verdicts,
            manifest,
            signature: None,
            package_hash: String::new(),
            frozen: false,
        };

        pkg.package_hash = pkg.compute_package_hash();
        pkg
    }

    /// Freeze the package. Once frozen, no modifications are allowed.
    pub fn freeze(&mut self) {
        self.frozen = true;
    }

    /// Verify the integrity of the package by recomputing the package hash.
    pub fn verify(&self) -> bool {
        self.package_hash == self.compute_package_hash()
    }

    /// Sign the package. This sets the signature and freezes the package.
    pub fn sign(&mut self, signer: String, key_id: String, signature: String, signed_at: u64) {
        self.signature = Some(PackageSignature {
            signer,
            key_id,
            algorithm: "ed25519".into(),
            signature,
            signed_at,
        });
        self.frozen = true;
    }

    fn compute_package_hash(&self) -> String {
        let mut hasher = Hasher::new();
        hasher.update(b"AMUN_PUBLICATION_PACKAGE_V1");
        hasher.update(self.package_id.as_bytes());
        hasher.update(self.version.as_bytes());
        hasher.update(&self.generated_at.to_le_bytes());
        hasher.update(self.manifest.report_hash.as_bytes());
        for vh in &self.manifest.verdict_hashes {
            hasher.update(vh.as_bytes());
        }
        hex::encode(hasher.finalize().as_bytes())
    }

    fn hash_report(report: &ConstitutionalReport) -> String {
        let json = serde_json::to_string(report).unwrap_or_default();
        let mut hasher = Hasher::new();
        hasher.update(b"AMUN_CONSTITUTIONAL_REPORT_V1");
        hasher.update(json.as_bytes());
        hex::encode(hasher.finalize().as_bytes())
    }
}
