use bevy::prelude::*;
use std::ops::Add;

// Coordinates of all its 8 neighbouring cells
const NEIGHBOR_COORDINATES: [IVec2; 8] = [
// Top Left
    IVec2::new(-1, -1),
// Top
    IVec2::new(0, -1),
// Top Right
    IVec2::new(1, -1),

// Left
    IVec2::new(-1, -0),
// Right
    IVec2::new(1, -0),

// Bottom Left
    IVec2::new(-1, 1),
// Bottom
    IVec2::new(0, 1),
// Bottom Right
    IVec2::new(1, 1),
];

#[derive(Component)]
pub struct Cell {
    // The 2d coordinates
    pub coords: IVec2,
}

impl Cell {
    pub fn get_neighbor_coords(&self) -> [IVec2; 8] {
        NEIGHBOR_COORDINATES
            .map(|i| i.add(self.coords))
    }
}
