use serde::Deserialize;
use std::collections::{HashMap, HashSet, BTreeMap, VecDeque};
use std::process::ExitCode;

mod scc;
mod edge_semantics;

use scc::find_cycles;
use edge_semantics::{classify_edges, constitutional_edges_only, EdgeType};

// ─── Cargo Metadata ───────────────────────────────────────
#[derive(Debug, Deserialize)]
struct CargoMetadata { packages: Vec<Package>, resolve: Option<Resolve> }
#[derive(Debug, Deserialize)]
pub struct Package { pub name: String, pub dependencies: Vec<Dependency> }
#[derive(Debug, Deserialize)]
pub struct Dependency { pub name: String, #[serde(default)] pub optional: bool }
#[derive(Debug, Deserialize)]
pub struct Resolve { pub nodes: Vec<ResolveNode> }
#[derive(Debug, Deserialize)]
pub struct ResolveNode { pub id: String, pub dependencies: Vec<String> }

// ─── Constitution Model (from YAML) ───────────────────────
#[derive(Debug, Deserialize)]
struct Constitution {
    version: String,
    layers: Vec<ConstitutionalLayer>,
    prohibited_dependencies: Vec<ProhibitedDependency>,
    crate_freeze: CrateFreezePolicy,
}

#[derive(Debug, Deserialize)]
struct ConstitutionalLayer {
    id: String,
    name: String,
    crates: Vec<String>,
    laws: Vec<Law>,
}

#[derive(Debug, Deserialize)]
struct Law {
    id: String,
    text: String,
    #[serde(rename = "type")]
    law_type: String,
    #[serde(default)]
    from_layer: Option<String>,
    #[serde(default)]
    to_layer: Option<Vec<String>>,
    #[serde(default)]
    dependencies: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
struct ProhibitedDependency {
    from: String,
    to: String,
    reason: String,
}

#[derive(Debug, Deserialize)]
struct CrateFreezePolicy {
    enabled: bool,
    policy: String,
}

// ─── Layer Model ──────────────────────────────────────────
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Layer { Kernel, Constitutional, Interface, Consensus, Execution, Persistence, Network, Governance, Testing, Unknown }

impl Layer {
    fn name(&self) -> &str {
        match self {
            Layer::Kernel => "Layer 0 (Kernel)",
            Layer::Constitutional => "Layer 0.5 (Constitutional)",
            Layer::Interface => "Interface",
            Layer::Consensus => "Layer 1 (Consensus)",
            Layer::Execution => "Layer 2 (Execution)",
            Layer::Persistence => "Layer 3 (Persistence)",
            Layer::Network => "Layer 4 (Network)",
            Layer::Governance => "Layer 5 (Governance)",
            Layer::Testing => "Layer 6 (Testing)",
            Layer::Unknown => "Unknown",
        }
    }

    fn from_id(id: &str) -> Self {
        match id {
            "Kernel" => Layer::Kernel,
            "Constitutional" => Layer::Constitutional,
            "Interface" => Layer::Interface,
            "Consensus" => Layer::Consensus,
            "Execution" => Layer::Execution,
            "Persistence" => Layer::Persistence,
            "Network" => Layer::Network,
            "Governance" => Layer::Governance,
            "Testing" => Layer::Testing,
            _ => Layer::Unknown,
        }
    }

    fn classify(name: &str, constitution: &Constitution) -> Self {
        for layer in &constitution.layers {
            if layer.crates.contains(&name.to_string()) {
                return Layer::from_id(&layer.id);
            }
        }
        if name.starts_with("amun-") { Layer::Unknown } else { Layer::Unknown }
    }
}

// ─── Main ─────────────────────────────────────────────────
fn main() -> ExitCode {
    println!("=== Constitutional Linter v0.4.0 (Phase 48) ===\n");

    // Load constitution YAML
    let constitution_yaml = std::fs::read_to_string("docs/architecture/constitution.yaml")
        .unwrap_or_else(|_| "version: \"2.1\"\nlayers: []\nprohibited_dependencies: []\ncrate_freeze:\n  enabled: true\n  policy: \"RFC required\"".to_string());
    let constitution: Constitution = serde_yaml::from_str(&constitution_yaml)
        .unwrap_or_else(|e| {
            eprintln!("⚠️  Failed to parse constitution.yaml: {}", e);
            Constitution {
                version: "2.1".into(),
                layers: vec![],
                prohibited_dependencies: vec![],
                crate_freeze: CrateFreezePolicy { enabled: true, policy: "RFC required".into() },
            }
        });

    println!("Constitution v{} loaded ({} layers)\n", constitution.version, constitution.layers.len());

    // Run cargo metadata
    let output = std::process::Command::new("cargo")
        .args(["metadata", "--format-version", "1"]).output();
    let output = match output {
        Ok(o) => o,
        Err(e) => { eprintln!("cargo metadata: {}", e); return ExitCode::FAILURE; }
    };
    let metadata: CargoMetadata = match serde_json::from_slice(&output.stdout) {
        Ok(m) => m,
        Err(e) => { eprintln!("parse: {}", e); return ExitCode::FAILURE; }
    };

    let layers: HashMap<String, Layer> = metadata.packages.iter()
        .map(|p| (p.name.clone(), Layer::classify(&p.name, &constitution))).collect();

    let mut violations = 0u32;
    let mut check = |desc: &str, cond: bool, detail: &str| {
        if cond { println!("✅ {}", desc); }
        else { println!("❌ {}: {}", desc, detail); violations += 1; }
    };

    // ─── 1. SCC Cycle Detection ───────────────────────────
    println!("--- Architectural Cycle Detection ---");
    let typed_edges = classify_edges(&metadata.packages, &metadata.resolve);
    let const_edges = constitutional_edges_only(&typed_edges);
    let all_nodes: HashSet<String> = const_edges.keys().cloned()
        .chain(const_edges.values().flat_map(|v| v.iter().cloned()))
        .collect();

    let scc_result = find_cycles(&all_nodes, &const_edges);
    if scc_result.has_cycles {
        for cycle in &scc_result.cycles {
            check("Architectural cycles", false,
                &format!("Cycle detected: {}", cycle.join(" → ")));
        }
        // Check self-loops
        for (node, deps) in &const_edges {
            if deps.contains(node) {
                check("Self-loop", false, &format!("{} depends on itself", node));
            }
        }
    } else {
        println!("✅ No architectural cycles detected");
    }
    println!("   Analyzed {} nodes, {} edges, {} SCCs",
        all_nodes.len(),
        const_edges.values().map(|v| v.len()).sum::<usize>(),
        scc_result.components.len());

    // ─── 2. Edge Semantics Summary ────────────────────────
    println!("\n--- Edge Semantics ---");
    let total_edges: usize = typed_edges.values().map(|v| v.len()).sum();
    let runtime_edges: usize = typed_edges.values()
        .flat_map(|v| v.iter())
        .filter(|e| e.edge_type.is_constitutional())
        .count();
    let optional_edges = total_edges - runtime_edges;
    println!("   Total edges: {}", total_edges);
    println!("   Constitutional (runtime): {}", runtime_edges);
    println!("   Non-constitutional (optional/dev/build): {}", optional_edges);

    // ─── 3. Layer Purity (from YAML constitution) ──────────
    println!("\n--- Layer Purity Checks ---");
    for layer_def in &constitution.layers {
        for law in &layer_def.laws {
            if law.law_type == "forbid_dependency" {
                if let (Some(from), Some(to_layers)) = (&law.from_layer, &law.to_layer) {
                    let from_layer = Layer::from_id(from);
                    let forbidden: HashSet<Layer> = to_layers.iter().map(|l| Layer::from_id(l)).collect();

                    for pkg in &metadata.packages {
                        if layers.get(&pkg.name) == Some(&from_layer) {
                            for dep in &pkg.dependencies {
                                let dep_layer = layers.get(&dep.name).unwrap_or(&Layer::Unknown);
                                if forbidden.contains(dep_layer) && dep.name.starts_with("amun-") {
                                    check(&format!("Law {}", law.id), false,
                                        &format!("{} ({}) → {} ({})", pkg.name, from_layer.name(), dep.name, dep_layer.name()));
                                }
                            }
                        }
                    }
                }
            }
            if law.law_type == "forbid_dependency_exact" {
                if let Some(forbidden_deps) = &law.dependencies {
                    for pkg in &metadata.packages {
                        let pkg_layer = layers.get(&pkg.name).unwrap_or(&Layer::Unknown);
                        let layer_id = Layer::from_id(&layer_def.id);
                        if *pkg_layer == layer_id || layer_def.crates.contains(&pkg.name) {
                            for dep in &pkg.dependencies {
                                if forbidden_deps.contains(&dep.name) {
                                    check(&format!("Law {}", law.id), false,
                                        &format!("{} → {}", pkg.name, dep.name));
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // ─── 4. Crate Freeze Status ────────────────────────────
    println!("\n--- Crate Freeze Policy ---");
    if constitution.crate_freeze.enabled {
        println!("🔒 Crate freeze is ACTIVE: {}", constitution.crate_freeze.policy);
        // Count unclassified amun-* crates
        let unclassified: Vec<_> = metadata.packages.iter()
            .filter(|p| p.name.starts_with("amun-") && layers.get(&p.name) == Some(&Layer::Unknown))
            .map(|p| p.name.clone())
            .collect();
        if !unclassified.is_empty() {
            check("Crate classification", false,
                &format!("{} unclassified crates: {}", unclassified.len(), unclassified.join(", ")));
        }
    } else {
        println!("⚠️  Crate freeze is DISABLED");
    }

    // ─── 5. Prohibited Dependencies ────────────────────────
    println!("\n--- Prohibited Dependencies ---");
    for prohibited in &constitution.prohibited_dependencies {
        for pkg in &metadata.packages {
            if pkg.name == prohibited.from {
                for dep in &pkg.dependencies {
                    if dep.name == prohibited.to || (prohibited.to == "amun-*" && dep.name.starts_with("amun-")) {
                        check(&format!("Prohibited: {} → {}", prohibited.from, prohibited.to), false,
                            &format!("{} → {} ({})", pkg.name, dep.name, prohibited.reason));
                    }
                }
            }
        }
    }

    // ─── Result ────────────────────────────────────────────
    println!();
    if violations == 0 {
        println!("===== ALL CONSTITUTIONAL LAWS PASS ✅ =====");
        ExitCode::SUCCESS
    } else {
        println!("===== {} CONSTITUTIONAL VIOLATIONS ❌ =====", violations);
        ExitCode::FAILURE
    }
}
