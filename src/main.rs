use bevy::{
    prelude::*,
};

use std::collections::HashMap;
use rand::Rng;

// Constants
// These constants are defined in `Transform` units.
// Using the default 2D camera they correspond 1:1 with screen pixels.

const GRID_WIDTH: f32 = 100.;
const GRID_HEIGHT: f32 = 100.;

// Walls
const WALL_THICKNESS: f32 = 10.0;

// Cell
const CELL_SIZE: f32 = 5.;

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

// Wall
enum WallLocation {
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
struct WallBundle {
    sprite_bundle: SpriteBundle,
}

impl WallBundle {
    fn new(location: WallLocation) -> WallBundle {
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
struct Cell { 
    // The 2d coordinates
    pub coords: IVec2,
}

#[derive(Component, Clone, Debug)]
struct CellState {
    is_alive: bool,
}

impl CellState {
    fn toggle(&mut self) {
        self.is_alive = !self.is_alive;
    }
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
