use amun_constitutional_integration::ConstitutionalBridge;
use amun_constitutional_proof::{CertificationEvaluator, ReportGenerator};
use amun_verification_kernel::VerificationCertificate;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("═══════════════════════════════════════════");
    println!("  N47.8 — PRODUCTION EVIDENCE REPLAY");
    println!("═══════════════════════════════════════════");
    println!();

    let data_dir = std::env::var("AMUN_DATA_DIR")
        .unwrap_or_else(|_| "./data".to_string());
    let data_dir = PathBuf::from(&data_dir);

    // Phase 1: Load real system data
    println!("[1/6] Loading AmunChain operational data...");
    let phase_certificates = load_phase_certificates(&data_dir)?;
    
    if phase_certificates.is_empty() {
        println!();
        println!("WARNING: No verification certificates found.");
        println!("  This replay requires real AmunChain execution data.");
        println!("  Run an AmunChain node first to produce certificates,");
        println!("  then re-run this binary.");
        println!();
        println!("  Expected data layout:");
        println!("  {}/certificates/N41-CERT.json", data_dir.display());
        println!("  {}/certificates/N42-CERT.json", data_dir.display());
        println!("  ...");
        println!();
        println!("  Skipping production replay — N47.8 deferred.");
        return Ok(());
    }

    println!("   Loaded {} phase certificates", phase_certificates.len());
    for (phase, cert) in &phase_certificates {
        println!("   - {} : {} ({} claims)", phase, cert.certificate_id, cert.claims.len());
    }

    // Phase 2: Run constitutional pipeline
    println!();
    println!("[2/6] Running constitutional validation pipeline...");
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    let (registry, archive, verdicts, report, pkg) =
        ConstitutionalBridge::run_full_pipeline(phase_certificates, timestamp)
            .map_err(|e| format!("Pipeline failed: {}", e))?;

    println!("   Registry: {} obligations", registry.total());
    println!("   Archive: {} evidence records", archive.total_count());
    println!("   Verdicts: {} phase verdicts", verdicts.len());

    // Phase 3: Generate report
    println!();
    println!("[3/6] Generating constitutional report...");
    let markdown = ReportGenerator::to_markdown(&report);
    let report_path = PathBuf::from("N47_CONSTITUTIONAL_REPORT.md");
    fs::write(&report_path, &markdown)?;
    println!("   Written: {}", report_path.display());

    let json_report = ReportGenerator::to_json(&report)?;
    let json_report_path = PathBuf::from("N47_CONSTITUTIONAL_REPORT.json");
    fs::write(&json_report_path, &json_report)?;
    println!("   Written: {}", json_report_path.display());

    // Phase 4: Generate publication package
    println!();
    println!("[4/6] Generating publication package...");
    let pkg_json = serde_json::to_string_pretty(&pkg)?;
    let pkg_path = PathBuf::from("N47_PKG_001.json");
    fs::write(&pkg_path, &pkg_json)?;
    println!("   Written: {}", pkg_path.display());
    println!("   Package hash: {}", pkg.package_hash);
    println!("   Frozen: {}", pkg.frozen);

    // Phase 5: Issue constitutional certificate
    println!();
    println!("[5/6] Issuing constitutional certificate...");
    let cert = CertificationEvaluator::evaluate(
        &registry,
        &archive,
        &verdicts,
        pkg.package_id.clone(),
        timestamp,
        "N47-Constitutional-Authority".into(),
    );

    let cert_json = serde_json::to_string_pretty(&cert)?;
    let cert_path = PathBuf::from("N47_CERT_001.json");
    fs::write(&cert_path, &cert_json)?;
    println!("   Written: {}", cert_path.display());
    println!("   Verdict: {:?}", cert.verdict);

    // Phase 6: Summary
    println!();
    println!("═══════════════════════════════════════════");
    println!("  N47.8 — PRODUCTION REPLAY COMPLETE");
    println!("═══════════════════════════════════════════");
    println!();
    println!("  Outputs:");
    println!("    N47_CONSTITUTIONAL_REPORT.md");
    println!("    N47_CONSTITUTIONAL_REPORT.json");
    println!("    N47_PKG_001.json");
    println!("    N47_CERT_001.json");
    println!();
    println!("  Gates:");
    for gate in &cert.gates {
        let status = if gate.passed { "PASS" } else { "FAIL" };
        let kind = if gate.is_hard_gate { "HARD" } else { "COND" };
        println!("    [{}] [{}] {} — {}", status, kind, gate.gate_id, gate.details);
    }
    println!();
    println!("  Overall: {:?}", cert.verdict);
    println!();

    Ok(())
}

fn load_phase_certificates(
    data_dir: &Path,
) -> Result<HashMap<String, VerificationCertificate>, Box<dyn std::error::Error>> {
    let mut certs = HashMap::new();
    let phases = ["N41", "N42", "N43", "N44", "N45", "N46", "N46.5"];

    let cert_dir = data_dir.join("certificates");
    let search_dir = if cert_dir.exists() {
        cert_dir
    } else {
        PathBuf::from(".")
    };

    for phase in &phases {
        let filename = format!("{}-CERT.json", phase);
        let path = search_dir.join(&filename);

        if path.exists() {
            let content = fs::read_to_string(&path)?;
            let cert: VerificationCertificate = serde_json::from_str(&content)?;
            if cert.verify() {
                certs.insert(phase.to_string(), cert);
            } else {
                eprintln!("   Warning: {} failed verification, skipping", filename);
            }
        }
    }

    Ok(certs)
}
