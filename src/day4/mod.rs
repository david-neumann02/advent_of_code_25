use std::{
    cmp::min,
    collections::{btree_map::Range, vec_deque::Iter},
    iter::{Repeat, RepeatN, repeat_n},
    marker::PhantomData,
    ops::{Deref, Index, IndexMut},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Square {
    Paper,
    Empty,
}

struct Grid {
    grid: Vec<Vec<Square>>,
    width: usize,
    height: usize,
}

impl Index<(usize, usize)> for Grid {
    type Output = Square;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.grid[index.0][index.1]
    }
}

impl IndexMut<(usize, usize)> for Grid {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.grid[index.0][index.1]
    }
}

impl Grid {
    fn new(grid_base: &[&str]) -> Self {
        let grid: Vec<Vec<Square>> = grid_base
            .iter()
            .map(|line: &&str| {
                line.chars()
                    .map(|c| match c {
                        '@' => Square::Paper,
                        _ => Square::Empty,
                    })
                    .collect()
            })
            .collect();

        let width = grid.get(0).unwrap().len();

        let height = grid.len();

        Grid {
            grid,
            width,
            height,
        }
    }

    fn check_surrounding(&self, pos: (usize, usize)) -> u64 {
        let mut val = 0;
        for i in pos.0.saturating_sub(1)..=pos.0 + 1 {
            for j in pos.1.saturating_sub(1)..=pos.1 + 1 {
                if i >= self.height || j >= self.width || (i, j) == pos {
                    continue;
                }
                val += match self[(i, j)] {
                    Square::Paper => 1,
                    Square::Empty => 0,
                };
            }
        }
        val
    }
}

pub(crate) fn day4(input: &Vec<&str>) -> (u64, u64) {
    let mut grid = Grid::new(&input);

    (part1(&grid), part2(grid))
}

fn part1(grid: &Grid) -> u64 {
    (0..grid.height)
        .map(|i| repeat_n(i, grid.width).zip(0..grid.width))
        .flatten()
        .map(|pos| {
            if grid.check_surrounding(pos) < 4 && grid[pos] == Square::Paper {
                1
            } else {
                0
            }
        })
        .sum()
}

fn part2(mut grid: Grid) -> u64 {
    let mut changed = true;
    let mut sum = 0;
    let (grid_height, grid_width) = (grid.height, grid.width);

    while changed {
        changed = false;

        for pos in (0..grid_height)
            .map(|i| repeat_n(i, grid_width).zip(0..grid_width))
            .flatten()
        {
            sum += if grid.check_surrounding(pos) < 4 && grid[pos] == Square::Paper {
                changed = true;
                grid[pos] = Square::Empty;
                1
            } else {
                0
            }
        }
    }
    sum
}
