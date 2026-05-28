use std::collections::{HashMap, HashSet};

/// Tarjan's Strongly Connected Components algorithm for cycle detection.
/// Identifies architectural cycles that create irreversible coupling.

#[derive(Debug, Clone)]
pub struct SCCResult {
    /// Each SCC is a set of mutually reachable nodes
    pub components: Vec<HashSet<String>>,
    /// Cycles (SCCs with size > 1 or self-loops)
    pub cycles: Vec<Vec<String>>,
    /// Whether any cycles were found
    pub has_cycles: bool,
}

/// Run Tarjan's SCC algorithm on a dependency graph
pub fn find_cycles(
    nodes: &HashSet<String>,
    edges: &HashMap<String, HashSet<String>>,
) -> SCCResult {
    let mut index_counter = 0u32;
    let mut indices: HashMap<String, u32> = HashMap::new();
    let mut lowlink: HashMap<String, u32> = HashMap::new();
    let mut on_stack: HashSet<String> = HashSet::new();
    let mut stack: Vec<String> = Vec::new();
    let mut components: Vec<HashSet<String>> = Vec::new();

    fn strongconnect(
        node: &str,
        index_counter: &mut u32,
        indices: &mut HashMap<String, u32>,
        lowlink: &mut HashMap<String, u32>,
        on_stack: &mut HashSet<String>,
        stack: &mut Vec<String>,
        components: &mut Vec<HashSet<String>>,
        edges: &HashMap<String, HashSet<String>>,
    ) {
        indices.insert(node.to_string(), *index_counter);
        lowlink.insert(node.to_string(), *index_counter);
        *index_counter += 1;
        stack.push(node.to_string());
        on_stack.insert(node.to_string());

        if let Some(neighbors) = edges.get(node) {
            for neighbor in neighbors {
                if !indices.contains_key(neighbor) {
                    strongconnect(neighbor, index_counter, indices, lowlink, on_stack, stack, components, edges);
                    let nl = lowlink.get(neighbor).copied().unwrap_or(0);
                    let ll = lowlink.get(node).copied().unwrap_or(0);
                    lowlink.insert(node.to_string(), nl.min(ll));
                } else if on_stack.contains(neighbor) {
                    let ni = indices.get(neighbor).copied().unwrap_or(0);
                    let ll = lowlink.get(node).copied().unwrap_or(0);
                    lowlink.insert(node.to_string(), ni.min(ll));
                }
            }
        }

        if lowlink.get(node).copied() == indices.get(node).copied() {
            let mut component = HashSet::new();
            loop {
                let w = stack.pop().unwrap();
                on_stack.remove(&w);
                component.insert(w.clone());
                if w == node {
                    break;
                }
            }
            components.push(component);
        }
    }

    // Run Tarjan from each unvisited node
    for node in nodes {
        if !indices.contains_key(node) {
            strongconnect(node, &mut index_counter, &mut indices, &mut lowlink, &mut on_stack, &mut stack, &mut components, edges);
        }
    }

    // Extract cycles (SCCs with size > 1, or self-loops)
    let cycles: Vec<Vec<String>> = components.iter()
        .filter(|c| c.len() > 1)
        .map(|c| {
            let mut v: Vec<String> = c.iter().cloned().collect();
            v.sort();
            v
        })
        .collect();

    // Check for self-loops
    let mut has_self_loops = false;
    for (node, deps) in edges {
        if deps.contains(node) {
            has_self_loops = true;
            break;
        }
    }

    SCCResult {
        has_cycles: !cycles.is_empty() || has_self_loops,
        components,
        cycles,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_cycles() {
        let nodes: HashSet<String> = ["A", "B", "C"].iter().map(|s| s.to_string()).collect();
        let edges: HashMap<String, HashSet<String>> = HashMap::from([
            ("A".into(), HashSet::from(["B".into()])),
            ("B".into(), HashSet::from(["C".into()])),
        ]);
        let result = find_cycles(&nodes, &edges);
        assert!(!result.has_cycles);
    }

    #[test]
    fn test_cycle_detection() {
        let nodes: HashSet<String> = ["A", "B", "C"].iter().map(|s| s.to_string()).collect();
        let edges: HashMap<String, HashSet<String>> = HashMap::from([
            ("A".into(), HashSet::from(["B".into()])),
            ("B".into(), HashSet::from(["C".into()])),
            ("C".into(), HashSet::from(["A".into()])), // cycle!
        ]);
        let result = find_cycles(&nodes, &edges);
        assert!(result.has_cycles);
        assert_eq!(result.cycles.len(), 1);
    }

    #[test]
    fn test_self_loop() {
        let nodes: HashSet<String> = ["A"].iter().map(|s| s.to_string()).collect();
        let edges: HashMap<String, HashSet<String>> = HashMap::from([
            ("A".into(), HashSet::from(["A".into()])),
        ]);
        let result = find_cycles(&nodes, &edges);
        assert!(result.has_cycles);
    }
}
