use std::collections::BinaryHeap;
use std::cmp::Ordering;
use super::grid;
use super::util::Vec2;

const INF: u16 = 0xFFFF;

#[derive(Clone, Copy)]
struct Node {
    came_from: Vec2,
    cost: u16,
}

impl Node {
    fn new() -> Self {
        Self {
            came_from: Vec2 {x: INF, y: INF},
            cost: INF,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct State {
    n: Vec2,
    cost: u16,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
            .then_with(|| self.n.cmp(&other.n))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for Vec2 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.y.cmp(&other.y)
            .then_with(|| self.x.cmp(&other.x))
    }
}

impl PartialOrd for Vec2 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

const GRID_SIZE: usize = grid::GRID_SIZE as usize;

fn heuristic(from: Vec2, to: Vec2) -> u16 {
    let (fx, fy, tx, ty) = (from.x as i32, from.y as i32, to.x as i32, to.y as i32);
    i32::max((fx - tx).abs(), (fy - ty).abs()) as u16 * 10
}

pub fn pf_a_star(g: &grid::Grid, from: Vec2, to: Vec2) -> (Vec<Vec2>, u16) {
    let mut grid = [[Node::new(); GRID_SIZE]; GRID_SIZE];
    grid[from.y as usize][from.x as usize].cost = 0;

    let mut q = BinaryHeap::new();
    q.push(State {
        cost: 0,
        n: from,
    });

    while let Some(State { n, cost }) = q.pop() {
        if n == to { break; }
        let c = grid[n.y as usize][n.x as usize].cost;
        if cost > c + heuristic(n, to) { continue; }

        let neighbours = [
            (n.x - 1, n.y - 1), (n.x, n.y - 1), (n.x + 1, n.y - 1),
            (n.x - 1, n.y), (n.x + 1, n.y),
            (n.x - 1, n.y + 1), (n.x, n.y + 1), (n.x + 1, n.y + 1),
        ];
        let neigh_costs = [
            14, 10, 14,
            10, 10,
            14, 10, 14
        ];
        for ((x,y), d) in neighbours.iter().zip(neigh_costs.iter()) {
            if !g.walkable(*x, *y) { continue; }
            let m = &mut grid[*y as usize][*x as usize];
            if c + d < m.cost {
                let pos = Vec2 { x: *x, y: *y };
                m.cost = c + d;
                m.came_from = n;
                q.push(State {
                    cost: c + d + heuristic(pos, to),
                    n: pos,
                });
            }
        }
    }

    let cost = grid[to.y as usize][to.x as usize].cost;

    if cost == INF {
        return (vec![], cost);
    } 

    let mut path = vec![];
    path.push(to);
    while path.last().unwrap() != &from {
        let n = path.last().unwrap().clone();
        path.push(grid[n.y as usize][n.x as usize].came_from);
    }

    (path, cost)
}

//slightly modified dijkstra
pub fn farthest(g: &grid::Grid, start: Vec2) -> (Vec<Vec2>, u16) {
    let mut grid = [[Node::new(); GRID_SIZE]; GRID_SIZE];
    grid[start.y as usize][start.x as usize].cost = 0;
    let mut q = BinaryHeap::new();
    q.push(State { n: start, cost: 0 });

    let (mut farthest, mut max_cost) = (start, 0);

    while let Some(State { n, cost }) = q.pop() {
        let c = grid[n.y as usize][n.x as usize].cost;
        if cost > c { continue; }

        let neighbours = [
            (n.x - 1, n.y - 1), (n.x, n.y - 1), (n.x + 1, n.y - 1),
            (n.x - 1, n.y), (n.x + 1, n.y),
            (n.x - 1, n.y + 1), (n.x, n.y + 1), (n.x + 1, n.y + 1),
        ];
        let neigh_costs = [
            14, 10, 14,
            10, 10,
            14, 10, 14
        ];

        if c > max_cost {
            max_cost = c;
            farthest = n;
        }

        for ((x,y), d) in neighbours.iter().zip(neigh_costs.iter()) {
            if !g.walkable(*x, *y) { continue; }
            let m = &mut grid[*y as usize][*x as usize];
            if c + d < m.cost {
                m.cost = c + d;
                m.came_from = n;
                q.push(State {
                    cost: c + d,
                    n: Vec2 { x: *x, y: *y },
                });
            }
        }
    }

    let mut path = vec![];
    path.push(farthest);
    while path.last().unwrap() != &start {
        let n = path.last().unwrap().clone();
        path.push(grid[n.y as usize][n.x as usize].came_from);
    }

    (path, max_cost)
}
