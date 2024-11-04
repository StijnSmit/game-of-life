use bevy::{
    prelude::*,
};

use std::collections::HashMap;
use rand::Rng;

pub mod cell;
pub mod cell_state;
pub mod wall;

use crate::cell::Cell;
use crate::cell_state::CellState;
use crate::wall::{WallLocation, WallBundle};

// Constants
// These constants are defined in `Transform` units.
// Using the default 2D camera they correspond 1:1 with screen pixels.

const GRID_WIDTH: f32 = 10.;
const GRID_HEIGHT: f32 = 10.;

// Walls
const WALL_THICKNESS: f32 = 10.0;

// Cell
pub const CELL_SIZE: f32 = 15.;

// Colors
const BACKGROUND_COLOR: Color = Color::srgb(1.0, 1.0, 1.0);
const WALL_COLOR: Color = Color::srgb(0.3, 0.3, 0.3);
const CELL_COLOR: Color = Color::srgb(0., 0., 0.);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Time::<Fixed>::from_hz(4.0))
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_systems(Startup, setup_camera)
        .add_systems(Startup, setup)
        //.add_systems(FixedUpdate, (check_cells, update_cells))
        .add_systems(FixedUpdate, update_cells)
        .run();
}

// Setups
fn setup_camera(mut commands: Commands) {
    // Camera
    commands.spawn(Camera2dBundle::default());
}

fn setup(mut commands: Commands) {
    let mut rng = rand::thread_rng();
    commands
        .spawn(SpatialBundle::from_transform(Transform::from_xyz(
            -(GRID_WIDTH * CELL_SIZE) / 2.,
            -(GRID_HEIGHT * CELL_SIZE) / 2.,
            0.,
        )))
        .with_children(|builder| {
            builder.spawn(WallBundle::new(WallLocation::Left));
            builder.spawn(WallBundle::new(WallLocation::Right));
            builder.spawn(WallBundle::new(WallLocation::Top));
            builder.spawn(WallBundle::new(WallLocation::Bottom));
            for y in 0..=(GRID_HEIGHT as i32) {
                for x in 0..=(GRID_WIDTH as i32) {

                    let is_alive = rng.gen_bool(1. / 4.);

                    builder.spawn((
                        SpriteBundle {
                            sprite: Sprite {
                                custom_size: Some(Vec2::splat(CELL_SIZE)),
                                color: WALL_COLOR,
                                ..default()
                            },
                            transform: Transform::from_xyz(
                                CELL_SIZE * x as f32,
                                CELL_SIZE * y as f32,
                                0.,
                            ),
                            ..default()
                        },
                        Cell { coords: IVec2::new(x, y) },
                        CellState { is_alive }
                    ));
                }
            }
        });
}

fn check_cells(mut query: Query<&mut Sprite, With<Cell>>) {
    let mut rng = rand::thread_rng();
    let random_color = Color::srgb(rng.gen(), rng.gen(), rng.gen());
    let mut count = 0 ;
    for mut sprite in &mut query {
        sprite.color = random_color;
        count += 1;
    }
    println!("Count: {}", count);
}

fn update_cells(
    mut commands: Commands,
    mut query: Query<(Entity, &Cell, &mut CellState, &mut Visibility)>
) {
    let map: HashMap<IVec2, CellState> = query
        .iter()
        .map(|(_, cell, state, _)| (cell.coords.clone(), state.clone()))
        .collect();

    for (coord, state) in map {
        println!("{state:?}");
    }
    let mut rng = rand::thread_rng();
    for (entity, cell, mut cellState, mut visibility) in &mut query {
        cellState.toggle();

        *visibility = if cellState.is_alive { Visibility::Visible } else { Visibility::Hidden };
                    let is_alive = rng.gen_bool(1. / 4.);
        commands.entity(entity).try_insert(CellState { is_alive });
    }
}
