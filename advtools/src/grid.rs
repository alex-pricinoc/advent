use std::ops::{Add, Index, IndexMut, Mul};

use itertools::Itertools;
use num::Integer;

#[derive(Clone, Copy, Debug)]
pub struct Pos<N = usize> {
    pub x: N,
    pub y: N,
}

impl From<(usize, usize)> for Pos {
    fn from((x, y): (usize, usize)) -> Self {
        Self { x, y }
    }
}

#[allow(non_snake_case)]
pub const fn Pos<N>(x: N, y: N) -> Pos<N> {
    Pos { x, y }
}

impl Add<Pos<isize>> for Pos {
    type Output = Option<Self>;

    fn add(self, other: Pos<isize>) -> Self::Output {
        let x = self.x.checked_add_signed(other.x)?;
        let y = self.y.checked_add_signed(other.y)?;

        Some(Pos(x, y))
    }
}

impl<N: Integer + Copy> Mul<N> for Pos<N> {
    type Output = Self;

    fn mul(self, other: N) -> Self::Output {
        Pos(self.x * other, self.y * other)
    }
}

pub const UL: Pos<isize> = Pos(-1, -1);
pub const UU: Pos<isize> = Pos(-1, 0);
pub const UR: Pos<isize> = Pos(-1, 1);
pub const LL: Pos<isize> = Pos(0, -1);
pub const RR: Pos<isize> = Pos(0, 1);
pub const DL: Pos<isize> = Pos(1, -1);
pub const DD: Pos<isize> = Pos(1, 0);
pub const DR: Pos<isize> = Pos(1, 1);

pub const DIRECTIONS: [Pos<isize>; 8] = [UL, UU, UR, LL, RR, DL, DD, DR];

pub struct Grid<T> {
    w: usize,
    h: usize,
    v: Vec<T>,
}

impl<T> Grid<T> {
    #[must_use]
    pub fn new<I: IntoIterator<Item = T>>(it: impl IntoIterator<Item = I>) -> Self {
        let mut v = Vec::new();
        let mut it = it.into_iter();
        let first = it.next().unwrap();
        v.extend(first);
        let w = v.len();
        for item in it {
            v.extend(item);
            assert_eq!(v.len() % w, 0);
        }
        Self {
            w,
            h: v.len() / w,
            v,
        }
    }

    pub fn positions(&self) -> impl Iterator<Item = Pos> {
        (0..self.h).cartesian_product(0..self.w).map(Pos::from)
    }

    #[must_use]
    pub fn in_bounds(&self, pos: Pos) -> bool {
        pos.x < self.w && pos.y < self.h
    }

    #[must_use]
    pub fn get(&self, pos: Pos) -> Option<&T> {
        if !self.in_bounds(pos) {
            return None;
        }

        Some(&self[pos])
    }

    #[must_use]
    pub fn get_mut(&mut self, pos: Pos) -> Option<&mut T> {
        if !self.in_bounds(pos) {
            return None;
        }

        Some(&mut self[pos])
    }
}

impl<T> Index<Pos> for Grid<T> {
    type Output = T;

    fn index(&self, Pos { x, y }: Pos) -> &Self::Output {
        &self.v[y * self.w + x]
    }
}

impl<T> IndexMut<Pos> for Grid<T> {
    fn index_mut(&mut self, Pos { x, y }: Pos) -> &mut Self::Output {
        &mut self.v[y * self.w + x]
    }
}
