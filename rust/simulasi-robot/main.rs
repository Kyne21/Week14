use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;

#[derive(Copy, Clone, Eq, PartialEq)]
struct Node {
    position: (usize, usize),
    cost: usize,
    heuristic: usize,
}

impl Node {
    fn f(&self) -> usize {
        self.cost + self.heuristic
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.f().cmp(&self.f())
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn heuristic(a: (usize, usize), b: (usize, usize)) -> usize {
    a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
}

fn a_star(grid: &Vec<Vec<usize>>, start: (usize, usize), goal: (usize, usize)) -> Option<Vec<(usize, usize)>> {
    let mut open_set = BinaryHeap::new();
    let mut came_from: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
    let mut g_score: HashMap<(usize, usize), usize> = HashMap::new();

    g_score.insert(start, 0);
    open_set.push(Node {
        position: start,
        cost: 0,
        heuristic: heuristic(start, goal),
    });

    let directions = vec![
        (1, 0),
        (0, 1),
        (-1, 0),
        (0, -1),
    ];

    while let Some(current) = open_set.pop() {
        if current.position == goal {
            let mut path = vec![goal];
            let mut curr = goal;
            while let Some(&prev) = came_from.get(&curr) {
                path.push(prev);
                curr = prev;
            }
            path.reverse();
            return Some(path);
        }

        for (dx, dy) in &directions {
            let (x, y) = (current.position.0 as isize + dx, current.position.1 as isize + dy);
            if x < 0 || y < 0 || x >= grid.len() as isize || y >= grid[0].len() as isize {
                continue;
            }
            let neighbor = (x as usize, y as usize);
            if grid[neighbor.0][neighbor.1] == 1 {
                continue;
            }
            let tentative_g_score = g_score[&current.position] + 1;
            if tentative_g_score < *g_score.get(&neighbor).unwrap_or(&usize::MAX) {
                came_from.insert(neighbor, current.position);
                g_score.insert(neighbor, tentative_g_score);
                open_set.push(Node {
                    position: neighbor,
                    cost: tentative_g_score,
                    heuristic: heuristic(neighbor, goal),
                });
            }
        }
    }
    None
}

fn main() {
    let grid = vec![
        vec![0, 0, 0, 0, 0],
        vec![0, 1, 1, 1, 0],
        vec![0, 1, 0, 1, 0],
        vec![0, 1, 0, 1, 0],
        vec![0, 0, 0, 0, 0],
    ];

    let start = (0, 0);
    let goal = (4, 4);

    match a_star(&grid, start, goal) {
        Some(path) => {
            println!("Jalur ditemukan:");
            for (i, (x, y)) in path.iter().enumerate() {
                println!("Langkah {}: ({}, {})", i + 1, x, y);
            }
        }
        None => println!("Tidak ada jalur yang ditemukan."),
    }
}
