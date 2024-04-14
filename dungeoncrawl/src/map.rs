use crate::prelude::*;

const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
}