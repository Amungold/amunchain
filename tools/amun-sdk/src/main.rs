use std::fs;
use std::process::{exit, Command};
use std::time::Instant;

const PHASES: &[(&str, &[&str])] = &[
    (
        "Phase 0 - Kernel & Safety",
        &["amun-unsafe", "amun-failure", "amun-kernel-types"],
    ),
    (
        "Phase 1 - Types & Constitution",
        &[
            "amun-codec",
            "amun-state-types",
            "amun-constitution",
            "amun-consensus-types",
        ],
    ),
    (
        "Phase 2 - Block & Execution",
        &[
            "amun-block",
            "amun-merkle",
            "amun-evidence",
            "amun-execution",
        ],
    ),
    (
        "Phase 3 - State & Storage",
        &[
            "amun-stf",
            "amun-transaction",
            "amun-runtime",
            "amun-storage",
        ],
    ),
    ("Phase 4 - Consensus Engine", &["amun-consensus"]),
    ("Phase 5 - Integration Tests", &["amun-determinism-tests"]),
    (
        "Phase 6 - Network & Cryptography",
        &["amun-bls", "amun-network", "amun-gossip"],
    ),
];

const TOOLS: &[&str] = &["constitutional-linter"];

struct CrateResult {
    name: String,
    passed: usize,
    failed: usize,
    success: bool,
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let command = args.get(1).map(|s| s.as_str()).unwrap_or("smoke");
    match command {
        "smoke" => run_smoke_test(),
        "build" => run_full_build(),
        "test" => run_full_test(),
        "lint" => run_linter(),
        "clean" => run_clean(),
        "doctor" => run_doctor(),
        "export-csv" => run_export_csv(),
        "export-json" => run_export_json(),
        _ => {
            eprintln!(
                "Usage: amun-sdk [smoke|build|test|lint|clean|doctor|export-csv|export-json]"
            );
            exit(1);
        }
    }
}

fn run_doctor() {
    print_banner("AmunChain Environment Doctor");
    check_tool("rustc", &["--version"]);
    check_tool("cargo", &["--version"]);
    let output = Command::new("rustc").arg("--version").output().unwrap();
    println!("  Rust: {}", String::from_utf8_lossy(&output.stdout).trim());
    let output = Command::new("cargo").arg("--version").output().unwrap();
    println!(
        "  Cargo: {}",
        String::from_utf8_lossy(&output.stdout).trim()
    );
    let status = Command::new("cargo")
        .arg("check")
        .arg("--workspace")
        .status()
        .unwrap();
    println!("  {} Workspace check", icon(status.success()));
    println!();
}

fn run_smoke_test() {
    let results = execute_all_tests();
    print_results(&results);
}

fn run_export_csv() {
    let results = execute_all_tests();
    let mut csv = String::from("Crate,Passed,Failed,Status\n");
    for r in &results {
        csv.push_str(&format!(
            "{},{},{},{}\n",
            r.name,
            r.passed,
            r.failed,
            if r.success { "OK" } else { "FAIL" }
        ));
    }
    fs::write("amun_smoke_report.csv", &csv).unwrap();
    println!("[OK] Report exported to amun_smoke_report.csv");
}

fn run_export_json() {
    let results = execute_all_tests();
    let mut json = String::from("[\n");
    for (i, r) in results.iter().enumerate() {
        json.push_str(&format!(
            "  {{\"crate\":\"{}\",\"passed\":{},\"failed\":{},\"status\":\"{}\"}}{}\n",
            r.name,
            r.passed,
            r.failed,
            if r.success { "OK" } else { "FAIL" },
            if i < results.len() - 1 { "," } else { "" }
        ));
    }
    json.push_str("]\n");
    fs::write("amun_smoke_report.json", &json).unwrap();
    println!("[OK] Report exported to amun_smoke_report.json");
}

fn execute_all_tests() -> Vec<CrateResult> {
    print_banner("AmunChain Smoke Test Suite");
    let _start = Instant::now();
    let mut all_results: Vec<CrateResult> = Vec::new();

    for (phase_name, crates) in PHASES {
        println!("{}", phase_name);
        for crate_name in *crates {
            let output = Command::new("cargo")
                .args(["test", "-p", crate_name, "-q"])
                .output()
                .unwrap();
            let combined = format!(
                "{}{}",
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            );
            let passed = extract_passed(&combined);
            let failed = extract_failed(&combined);
            let success = output.status.success() && failed == 0;
            all_results.push(CrateResult {
                name: crate_name.to_string(),
                passed,
                failed,
                success,
            });
            println!(
                "  {} {:30} {:>4} passed, {:>4} failed",
                icon(success),
                crate_name,
                passed,
                failed
            );
        }
        println!();
    }

    println!("Tools");
    for tool in TOOLS {
        let output = Command::new("cargo")
            .args(["test", "-p", tool, "-q"])
            .output()
            .unwrap();
        let combined = format!(
            "{}{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
        let passed = extract_passed(&combined);
        let failed = extract_failed(&combined);
        let success = output.status.success() && failed == 0;
        all_results.push(CrateResult {
            name: tool.to_string(),
            passed,
            failed,
            success,
        });
        println!(
            "  {} {:30} {:>4} passed, {:>4} failed",
            icon(success),
            tool,
            passed,
            failed
        );
    }
    println!();
    all_results
}

fn print_results(results: &[CrateResult]) {
    let total_passed: usize = results.iter().map(|r| r.passed).sum();
    let total_failed: usize = results.iter().map(|r| r.failed).sum();
    print_summary(total_passed, total_failed);
    print_table(results);
}

fn run_full_build() {
    print_banner("AmunChain Full Build");
    let start = Instant::now();
    let status = Command::new("cargo")
        .arg("build")
        .arg("--workspace")
        .status()
        .unwrap();
    println!(
        "  {} Build {} in {:.1}s",
        icon(status.success()),
        if status.success() {
            "succeeded"
        } else {
            "FAILED"
        },
        start.elapsed().as_secs_f32()
    );
}

fn run_full_test() {
    print_banner("AmunChain Full Test");
    let status = Command::new("cargo")
        .arg("test")
        .arg("--workspace")
        .status()
        .unwrap();
    println!(
        "  {} Tests {}",
        icon(status.success()),
        if status.success() { "passed" } else { "FAILED" }
    );
}

fn run_linter() {
    print_banner("Constitutional Linter");
    let status = Command::new("cargo")
        .arg("test")
        .arg("-p")
        .arg("constitutional-linter")
        .status()
        .unwrap();
    println!(
        "  {} Linter {}",
        icon(status.success()),
        if status.success() {
            "passed"
        } else {
            "FOUND VIOLATIONS"
        }
    );
}

fn run_clean() {
    Command::new("cargo").arg("clean").status().unwrap();
    println!("[OK] Clean complete");
}

fn check_tool(name: &str, args: &[&str]) {
    print!("  Checking {} ... ", name);
    let output = Command::new(name).args(args).output();
    match output {
        Ok(o) if o.status.success() => println!("[OK]"),
        _ => println!("[FAIL]"),
    }
}

fn extract_passed(output: &str) -> usize {
    let mut total = 0usize;
    for line in output.lines() {
        if line.contains("test result:") {
            if let Some(pos) = line.find("passed") {
                let before = &line[..pos].trim();
                if let Some(last_space) = before.rfind(|c: char| !c.is_alphanumeric()) {
                    total += before[last_space + 1..].trim().parse().unwrap_or(0);
                }
            }
        }
    }
    total
}

fn extract_failed(output: &str) -> usize {
    let mut total = 0usize;
    for line in output.lines() {
        if line.contains("test result:") {
            if let Some(pos) = line.find("failed") {
                let before = &line[..pos].trim();
                if let Some(last_space) = before.rfind(|c: char| !c.is_alphanumeric()) {
                    total += before[last_space + 1..].trim().parse().unwrap_or(0);
                }
            }
        }
    }
    total
}

fn icon(success: bool) -> &'static str {
    if success {
        "[OK]"
    } else {
        "[FAIL]"
    }
}

fn print_banner(title: &str) {
    println!();
    println!("╔══════════════════════════════════════════════════════╗");
    println!("║  {:50} ║", title);
    println!("╚══════════════════════════════════════════════════════╝");
    println!();
}

fn print_summary(passed: usize, failed: usize) {
    println!("╔══════════════════════════════════════════════════════╗");
    println!("║  SMOKE TEST SUMMARY                                ║");
    println!("╠══════════════════════════════════════════════════════╣");
    println!(
        "║  Total Passed : {:<5}                               ║",
        passed
    );
    println!(
        "║  Total Failed : {:<5}                               ║",
        failed
    );
    println!(
        "║  Status       : {:<35} ║",
        if failed == 0 {
            "ALL SYSTEMS OPERATIONAL"
        } else {
            "ISSUES DETECTED"
        }
    );
    println!("╚══════════════════════════════════════════════════════╝");
}

fn print_table(results: &[CrateResult]) {
    println!();
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║  CRATE                         PASSED   FAILED   STATUS    ║");
    println!("╠══════════════════════════════════════════════════════════════╣");
    for r in results {
        println!(
            "║  {:28}   {:>4}     {:>4}     {:<8} ║",
            r.name,
            r.passed,
            r.failed,
            if r.success { "OK" } else { "FAIL" }
        );
    }
    println!("╚══════════════════════════════════════════════════════════════╝");
}
