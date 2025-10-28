use std::collections::{HashMap, HashSet};
use uuid::Uuid;

fn has_cycle_dfs(
    graph: &HashMap<Uuid, Vec<Uuid>>,
    node: Uuid,
    visited: &mut HashSet<Uuid>,
    rec_stack: &mut HashSet<Uuid>,
    depth: usize,
) -> bool {
    let indent = "  ".repeat(depth);
    println!("{}🔍 Visiting: {:?}", indent, node);

    visited.insert(node);
    rec_stack.insert(node);
    println!("{}   Stack: {:?}", indent, rec_stack);

    if let Some(neighbors) = graph.get(&node) {
        for &neighbor in neighbors {
            println!("{}   → Checking neighbor: {:?}", indent, neighbor);

            if !visited.contains(&neighbor) {
                if has_cycle_dfs(graph, neighbor, visited, rec_stack, depth + 1) {
                    println!("{}   ❌ Cycle found!", indent);
                    return true;
                }
            } else if rec_stack.contains(&neighbor) {
                println!("{}   ❌ CYCLE! {:?} is in recursion stack", indent, neighbor);
                return true;
            } else {
                println!("{}   ✅ Already visited, not in stack", indent);
            }
        }
    }

    rec_stack.remove(&node);
    println!("{}🔙 Done with: {:?}", indent, node);
    false
}

fn main() {
    println!("##### Test Cycle Detection Algorithm! #####");
    let a = Uuid::new_v4();
    let b = Uuid::new_v4();
    let c = Uuid::new_v4();

    println!("Testing cycle detection algorithm\n");
    println!("Node IDs:");
    println!("A: {:?}", a);
    println!("B: {:?}", b);
    println!("C: {:?}", c);
    println!();

    // Test 1: No cycle (A → B → C)
    println!("═══ Test 1: Linear chain (A → B → C) ═══");
    let mut graph1: HashMap<Uuid, Vec<Uuid>> = HashMap::new();
    graph1.insert(a, vec![b]);
    graph1.insert(b, vec![c]);

    let mut visited = HashSet::new();
    let mut rec_stack = HashSet::new();

    let has_cycle = has_cycle_dfs(&graph1, a, &mut visited, &mut rec_stack, 0);
    println!("Result: {}\n", if has_cycle { "CYCLE ❌" } else { "NO CYCLE ✅" });

    // Test 2: Cycle (A → B → C → A)
    println!("═══ Test 2: Cycle (A → B → C → A) ═══");
    let mut graph2: HashMap<Uuid, Vec<Uuid>> = HashMap::new();
    graph2.insert(a, vec![b]);
    graph2.insert(b, vec![c]);
    graph2.insert(c, vec![a]); // Creates cycle!

    let mut visited = HashSet::new();
    let mut rec_stack = HashSet::new();

    let has_cycle = has_cycle_dfs(&graph2, a, &mut visited, &mut rec_stack, 0);
    println!("Result: {}\n", if has_cycle { "CYCLE ❌" } else { "NO CYCLE ✅" });

    // Test 3: Diamond (no cycle)
    println!("═══ Test 3: Diamond (A → B, A → C, B → C) ═══");
    let mut graph3: HashMap<Uuid, Vec<Uuid>> = HashMap::new();
    graph3.insert(a, vec![b, c]);
    graph3.insert(b, vec![c]);

    let mut visited = HashSet::new();
    let mut rec_stack = HashSet::new();

    let has_cycle = has_cycle_dfs(&graph3, a, &mut visited, &mut rec_stack, 0);
    println!("Result: {}\n", if has_cycle { "CYCLE ❌" } else { "NO CYCLE ✅" });
}
