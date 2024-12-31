use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use rand::Rng;

/// Representasi titik dalam ruang 2D
#[derive(Debug, Clone, PartialEq)]
struct Point {
    x: f64,
    y: f64,
}

impl Hash for Point {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.to_bits().hash(state);
        self.y.to_bits().hash(state);
    }
}

/// Node di graf untuk representasi jalur
#[derive(Debug, Clone)]
struct Node {
    id: usize,
    position: Point,
    neighbors: Vec<(usize, f64)>, // (ID tetangga, jarak ke tetangga)
}

/// Status dalam algoritma probabilistik
#[derive(Debug, PartialEq)]
struct State {
    id: usize,
    cost: f64,
}

impl Eq for State {}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.partial_cmp(&self.cost).unwrap_or(Ordering::Equal)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Graf untuk representasi lingkungan robot
struct Graph {
    nodes: Vec<Node>,
}

impl Graph {
    fn new() -> Self {
        Graph { nodes: Vec::new() }
    }

    fn add_node(&mut self, id: usize, x: f64, y: f64) {
        self.nodes.push(Node {
            id,
            position: Point { x, y },
            neighbors: Vec::new(),
        });
    }

    fn add_edge(&mut self, from: usize, to: usize, distance: f64) {
        if let Some(node) = self.nodes.iter_mut().find(|n| n.id == from) {
            node.neighbors.push((to, distance));
        }
    }

    /// Algoritma pencarian jalur probabilistik
    fn probabilistic_pathfinding(&self, start_id: usize, goal_id: usize) -> Option<Vec<usize>> {
        let mut heap = BinaryHeap::new();
        let mut costs = HashMap::new();
        let mut came_from = HashMap::new();

        costs.insert(start_id, 0.0);
        heap.push(State { id: start_id, cost: 0.0 });

        while let Some(State { id, cost }) = heap.pop() {
            if id == goal_id {
                let mut path = Vec::new();
                let mut current = Some(&id);

                while let Some(&node_id) = current {
                    path.push(node_id);
                    current = came_from.get(&node_id);
                }

                path.reverse();
                return Some(path);
            }

            let current_node = self.nodes.iter().find(|n| n.id == id)?;
            for &(neighbor_id, distance) in &current_node.neighbors {
                let mut rng = rand::thread_rng();
                let uncertainty: f64 = rng.gen_range(0.9..1.1); // Faktor ketidakpastian

                let new_cost = cost + (distance * uncertainty);
                let neighbor_cost = costs.get(&neighbor_id).cloned().unwrap_or(f64::INFINITY);

                if new_cost < neighbor_cost {
                    costs.insert(neighbor_id, new_cost);
                    came_from.insert(neighbor_id, id);
                    heap.push(State {
                        id: neighbor_id,
                        cost: new_cost,
                    });
                }
            }
        }

        None
    }
}

fn main() {
    let mut graph = Graph::new();

    // Tambahkan node (ID, x, y)
    graph.add_node(1, 0.0, 0.0);
    graph.add_node(2, 1.0, 1.0);
    graph.add_node(3, 2.0, 2.0);
    graph.add_node(4, 3.0, 1.5);

    // Tambahkan edge (dari, ke, jarak)
    graph.add_edge(1, 2, 1.5);
    graph.add_edge(2, 3, 1.0);
    graph.add_edge(3, 4, 1.2);
    graph.add_edge(1, 3, 2.5);

    let start = 1;
    let goal = 4;

    match graph.probabilistic_pathfinding(start, goal) {
        Some(path) => println!("Jalur terbaik: {:?}", path),
        None => println!("Tidak ada jalur yang ditemukan."),
    }
}
