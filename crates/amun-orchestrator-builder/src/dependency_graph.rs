use cargo_metadata::MetadataCommand;
use std::collections::{HashMap, HashSet};
use std::path::Path;

/// Maps crate name → set of crates that (transitively) depend on it.
pub struct DependencyGraph {
    dependents: HashMap<String, HashSet<String>>,
    /// All workspace member package names.
    pub workspace_members: HashSet<String>,
}

impl DependencyGraph {
    pub async fn build(
        workspace_root: &Path,
    ) -> Result<Self, amun_orchestrator_core::OrchestratorError> {
        let meta = MetadataCommand::new()
            .current_dir(workspace_root)
            .exec()
            .map_err(|e| {
                amun_orchestrator_core::OrchestratorError::Build(format!("cargo metadata: {e}"))
            })?;

        // Collect workspace member IDs
        let member_ids: HashSet<_> = meta.workspace_members.iter().cloned().collect();
        let packages: HashMap<_, _> = meta.packages.iter().map(|p| (p.id.clone(), p)).collect();

        let mut workspace_members = HashSet::new();

        // Build dependents map using the resolved dependency graph
        let mut dependents: HashMap<String, HashSet<String>> = HashMap::new();

        if let Some(resolve) = &meta.resolve {
            for node in &resolve.nodes {
                let pkg = match packages.get(&node.id) {
                    Some(p) => p,
                    None => continue,
                };

                if member_ids.contains(&node.id) {
                    workspace_members.insert(pkg.name.clone());
                }

                for dep_id in &node.dependencies {
                    if let Some(dep_pkg) = packages.get(dep_id) {
                        dependents
                            .entry(dep_pkg.name.clone())
                            .or_default()
                            .insert(pkg.name.clone());
                    }
                }
            }
        }

        // Also include packages without resolve (fallback)
        for pkg in &meta.packages {
            if member_ids.contains(&pkg.id) {
                workspace_members.insert(pkg.name.clone());
            }
            for dep in &pkg.dependencies {
                dependents
                    .entry(dep.name.clone())
                    .or_default()
                    .insert(pkg.name.clone());
            }
        }

        Ok(Self {
            dependents,
            workspace_members,
        })
    }

    /// Transitively find all crates that depend on the given set.
    pub fn transitive_dependents(&self, changed: &HashSet<String>) -> HashSet<String> {
        let mut result = HashSet::new();
        let mut stack: Vec<String> = changed.iter().cloned().collect();

        while let Some(name) = stack.pop() {
            if let Some(deps) = self.dependents.get(&name) {
                for d in deps {
                    if result.insert(d.clone()) {
                        stack.push(d.clone());
                    }
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transitive_dependents() {
        let mut deps = HashMap::new();
        deps.insert("A".into(), HashSet::from(["B".into()]));
        deps.insert("B".into(), HashSet::from(["C".into()]));
        let g = DependencyGraph {
            dependents: deps,
            workspace_members: HashSet::new(),
        };
        let changed = HashSet::from(["A".into()]);
        let affected = g.transitive_dependents(&changed);
        assert!(affected.contains("B"));
        assert!(affected.contains("C"));
    }

    #[test]
    fn test_no_dependents() {
        let g = DependencyGraph {
            dependents: HashMap::new(),
            workspace_members: HashSet::new(),
        };
        assert!(g
            .transitive_dependents(&HashSet::from(["A".into()]))
            .is_empty());
    }
}
