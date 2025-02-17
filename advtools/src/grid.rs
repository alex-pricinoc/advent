use std::fmt::Display;
use std::ops::{Add, Index, IndexMut, Mul, Sub};

use forward_ref_generic::{commutative_binop, forward_ref_binop, forward_ref_commutative_binop};
use itertools::Itertools;
use num::Integer;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Pos<N = usize> {
    pub x: N,
    pub y: N,
}

#[allow(non_snake_case)]
pub const fn Pos<N>(x: N, y: N) -> Pos<N> {
    Pos { x, y }
}

impl From<(usize, usize)> for Pos {
    fn from((x, y): (usize, usize)) -> Self {
        Self { x, y }
    }
}

impl Sub for Pos {
    type Output = Pos<isize>;

    fn sub(self, rhs: Self) -> Self::Output {
        let x = rhs.x as isize - self.x as isize;
        let y = rhs.y as isize - self.y as isize;
        Pos(x, y)
    }
}

forward_ref_binop! { impl Sub for Pos }

impl<N> Add for Pos<N>
where
    N: Add<Output = N>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

forward_ref_binop! {
    [N]
    impl Add for Pos<N>
    where N: Copy + Add<Output = N>
}

impl Add<Pos<isize>> for Pos {
    type Output = Option<Self>;

    fn add(mut self, other: Pos<isize>) -> Self::Output {
        self.x = self.x.checked_add_signed(other.x)?;
        self.y = self.y.checked_add_signed(other.y)?;
        Some(self)
    }
}

commutative_binop! { impl Add for Pos, Pos<isize> }

forward_ref_commutative_binop! { impl Add for Pos, Pos<isize> }

impl<N: Integer + Copy> Mul<N> for Pos<N> {
    type Output = Self;

    fn mul(self, other: N) -> Self::Output {
        Pos(self.x * other, self.y * other)
    }
}

pub const UU: Pos<isize> = Pos(0, -1);
pub const DD: Pos<isize> = Pos(0, 1);
pub const LL: Pos<isize> = Pos(-1, 0);
pub const RR: Pos<isize> = Pos(1, 0);
pub const UL: Pos<isize> = Pos(-1, -1);
pub const UR: Pos<isize> = Pos(1, -1);
pub const DL: Pos<isize> = Pos(-1, 1);
pub const DR: Pos<isize> = Pos(1, 1);

pub const DIRECTIONS: [Pos<isize>; 8] = [UL, UU, UR, LL, RR, DL, DD, DR];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn to_pos(&self) -> Pos<isize> {
        use Direction::*;
        match self {
            Up => UU,
            Down => DD,
            Left => LL,
            Right => RR,
        }
    }

    pub fn turn_right(&mut self) {
        use Direction::*;
        *self = match self {
            Up => Right,
            Down => Left,
            Left => Up,
            Right => Down,
        }
    }

    pub fn all() -> std::array::IntoIter<Direction, 4> {
        use Direction::*;
        [Up, Down, Right, Left].into_iter()
    }
}

#[derive(Debug, Default, Clone)]
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
    pub fn in_bounds(&self, pos: &Pos) -> bool {
        pos.x < self.w && pos.y < self.h
    }

    #[must_use]
    pub fn get(&self, pos: &Pos) -> Option<&T> {
        if !self.in_bounds(pos) {
            return None;
        }

        Some(&self[*pos])
    }

    #[must_use]
    pub fn get_mut(&mut self, pos: &Pos) -> Option<&mut T> {
        if !self.in_bounds(pos) {
            return None;
        }

        Some(&mut self[*pos])
    }

    pub fn rows(&self) -> impl Iterator<Item = &[T]> {
        self.v.chunks_exact(self.w)
    }

    pub fn find_pos(&self, mut f: impl FnMut(&T) -> bool) -> Option<Pos> {
        self.positions().find(|&p| f(&self[p]))
    }

    pub fn neighbours(&self, from: Pos) -> impl Iterator<Item = Pos> + '_ {
        Direction::all().flat_map(move |d| (from + d.to_pos()).take_if(|t| self.in_bounds(t)))
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

impl Display for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}

impl<T: Display> Display for Grid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for row in self.rows() {
            for t in row {
                write!(f, "{t}")?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}
