use bevy::prelude::*;

use crate::{WALL_THICKNESS, CELL_SIZE, GRID_WIDTH, GRID_HEIGHT, WALL_COLOR};


// Wall
pub enum WallLocation {
    Left, Right, Bottom, Top
}

impl WallLocation {
    fn position(&self) -> Vec2 {
        match self {
            WallLocation::Left => Vec2::new(-(WALL_THICKNESS * 0.5) -(CELL_SIZE * 0.5), GRID_HEIGHT * CELL_SIZE * 0.5),
            WallLocation::Right => Vec2::new((CELL_SIZE * 0.5) + (GRID_WIDTH * CELL_SIZE) + (WALL_THICKNESS * 0.5), GRID_HEIGHT * CELL_SIZE * 0.5),
            WallLocation::Bottom => Vec2::new(GRID_WIDTH * CELL_SIZE * 0.5, -(WALL_THICKNESS * 0.5) -(CELL_SIZE * 0.5)),
            WallLocation::Top => Vec2::new(GRID_WIDTH * CELL_SIZE * 0.5, (GRID_HEIGHT * CELL_SIZE) + (WALL_THICKNESS * 0.5) + (CELL_SIZE * 0.5)),
        }
    }

    fn size(&self) -> Vec2 {
        match self {
            WallLocation::Left | WallLocation::Right => {
                Vec2::new(WALL_THICKNESS, GRID_HEIGHT * CELL_SIZE + (WALL_THICKNESS * 2.) + CELL_SIZE)
            }
            WallLocation::Bottom | WallLocation::Top => {
                Vec2::new(GRID_WIDTH * CELL_SIZE + CELL_SIZE, WALL_THICKNESS)
            }
        }
    }

    fn color(&self) -> Color {
        WALL_COLOR
    }
}


#[derive(Bundle)]
pub struct WallBundle {
    sprite_bundle: SpriteBundle,
}

impl WallBundle {
    pub fn new(location: WallLocation) -> WallBundle {
        WallBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: location.position().extend(0.0),
                    ..default()
                },
                sprite: Sprite {
                    color: location.color(),
                    custom_size: Some(location.size()),
                    ..default()
                },
                ..default()
            }
        }
    }
}

