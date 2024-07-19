use std::default;

use crate::*;
use rand::Rng;

#[derive(Default, PartialEq, Eq, Hash, Clone, Copy, Debug)]

pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]

pub enum FourDirs {
    #[default]
    Left,
    Right,
    Bottom,
    Top,
}

pub fn get_eight_dirs() -> [(i32, i32); 8] {
    [
        FourDirs::Left.value(),
        FourDirs::Right.value(),
        FourDirs::Bottom.value(),
        FourDirs::Top.value(),
        (-1, 1),
        (1, 1),
        (1, -1),
        (-1, -1),
    ]
}

impl FourDirs {
    pub fn get_rand_dir() -> Self {
        let mut rng = rand::thread_rng();
        match rng.gen_range(0..4) {
            0 => Self::Left,
            1 => Self::Right,
            2 => Self::Bottom,
            _ => Self::Top,
        }
    }
    pub fn value(&self) -> (i32, i32) {
        match self {
            Self::Left => (-1, 0),
            Self::Right => (1, 0),
            Self::Bottom => (0, -1),
            Self::Top => (0, 1),
        }
    }
}
