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
