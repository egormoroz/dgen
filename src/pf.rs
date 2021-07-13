use std::collections::VecDeque;
use crate::{Cell, Vec2};

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

const GRID_SIZE: usize = crate::GRID_SIZE as usize;

pub fn pf_dijkstra(g: &[[Cell; GRID_SIZE]; GRID_SIZE], 
    from: Vec2, to: Vec2) -> Vec<Vec2> 
{
    let mut grid = [[Node::new(); GRID_SIZE]; GRID_SIZE];
    grid[from.y as usize][from.x as usize].cost = 0;
    let mut q = VecDeque::new();
    q.push_back(from);

    while let Some(n) = q.pop_front() {
        if n == to {
            break;
        }
        let neighbours = [
            (n.x - 1, n.y - 1), (n.x, n.y - 1), (n.x + 1, n.y - 1),
            (n.x - 1, n.y), (n.x + 1, n.y),
            (n.x - 1, n.y + 1), (n.x, n.y + 1), (n.x + 1, n.y + 1),
        ];
        let cost = [
            14, 10, 14,
            10, 10,
            14, 10, 14
        ];
        let c = grid[n.y as usize][n.x as usize].cost;
        for ((x,y), d) in neighbours.iter().zip(cost.iter()) {
            if !g[*y as usize][*x as usize].walkable() { continue; }
            let m = &mut grid[*y as usize][*x as usize];
            if m.cost == INF {
                m.cost = c + d;
                m.came_from = n;
                q.push_back(Vec2{x: *x, y: *y});
            } else if c + d < m.cost {
                m.cost = c + d;
                m.came_from = n;
            }
        }
    }

    if grid[to.y as usize][to.x as usize].cost == INF {
        return vec![];
    } 

    let mut path = vec![];
    path.push(to);
    while path.last().unwrap() != &from {
        let n = path.last().unwrap().clone();
        path.push(grid[n.y as usize][n.x as usize].came_from);
    }

    path
}
