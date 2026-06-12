use std::process::{Child, Command};
use std::thread;
use std::time::Duration;

fn main() {
    let cycles: u32 = 5;
    let interval: u32 = 90;

    println!("=== N100.4 REJOIN STRESS TEST ===");
    println!("Cycles: {} | Interval: {}s", cycles, interval);

    // Start 4 validators
    let mut children: Vec<Child> = Vec::new();
    for i in 0..4 {
        let child = Command::new("cargo")
            .args([
                "run",
                "--release",
                "--bin",
                "validator",
                "--",
                &i.to_string(),
                &(9900 + i).to_string(),
            ])
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .spawn()
            .expect("Failed to start validator");
        children.push(child);
        println!("Started validator {}", i);
    }

    // Wait for initial sync
    println!("Waiting 90s for initial chain sync...");
    thread::sleep(Duration::from_secs(90));

    // Stress cycles
    let mut successes = 0;
    let mut failures = 0;

    for cycle in 1..=cycles {
        let victim = (cycle % 4) as usize;
        println!(
            "
Cycle {}/{} | t={}s | Killing validator {}",
            cycle,
            cycles,
            cycle * interval,
            victim
        );

        // Kill one validator
        children[victim].kill().expect("Failed to kill validator");
        println!("Killed validator {}", victim);

        // Wait for network to continue
        thread::sleep(Duration::from_secs(60));

        // Rejoin the killed validator
        println!("Rejoining validator {}...", victim);
        let rejoin_result = Command::new("cargo")
            .args([
                "run",
                "--release",
                "--bin",
                "rejoin",
                "--",
                &format!("/tmp/amun-test-validator-{}", victim),
                "127.0.0.1:9900",
                "127.0.0.1:9901",
                "127.0.0.1:9902",
                "127.0.0.1:9903",
            ])
            .output()
            .expect("Failed to run rejoin");

        // Check rejoin success
        if rejoin_result.status.success() {
            // Run store audit to check equivalence
            let audit_result = Command::new("cargo")
                .args([
                    "run",
                    "--release",
                    "--example",
                    "store_stats",
                    "--",
                    &format!("/tmp/amun-test-validator-{}", victim),
                ])
                .output()
                .expect("Failed to run store audit");

            let audit_output = String::from_utf8_lossy(&audit_result.stdout);
            let parts: Vec<&str> = audit_output.split_whitespace().collect();
            if parts.len() >= 3 && parts[2] == "0" {
                println!("Result: PASS (store missing=0)");
                successes += 1;
            } else {
                println!("Result: FAIL (store has gaps: {})", audit_output.trim());
                failures += 1;
            }
        } else {
            println!("Result: FAIL (rejoin command failed)");
            failures += 1;
        }

        // Restart the validator
        let child = Command::new("cargo")
            .args([
                "run",
                "--release",
                "--bin",
                "validator",
                "--",
                &victim.to_string(),
                &(9900 + victim).to_string(),
            ])
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .spawn()
            .expect("Failed to restart validator");
        children[victim] = child;
        println!("Restarted validator {}", victim);

        // Wait for next cycle
        if cycle < cycles {
            thread::sleep(Duration::from_secs(30));
        }
    }

    // Cleanup
    for child in &mut children {
        let _ = child.kill();
    }

    println!("\n========================================");
    println!("  N100.4 REJOIN STRESS TEST RESULTS");
    println!("========================================");
    println!("  Cycles: {}", cycles);
    println!("  Passed: {}", successes);
    println!("  Failed: {}", failures);
    println!("  Verdict: {}", if failures == 0 { "PASS" } else { "FAIL" });
    println!("========================================");
}
