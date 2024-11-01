use bevy::{
    prelude::*,
};

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
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_systems(Startup, setup_camera)
        .add_systems(Startup, setup)
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

#[derive(Component)]
struct Cell { 
    // The 2d coordinates
    coords: IVec2,
}

#[derive(Component)]
struct CellState {
    alive: bool,
}

// Setups
fn setup_camera(mut commands: Commands) {
    // Camera
    commands.spawn(Camera2dBundle::default());
}

fn setup(mut commands: Commands) {
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
                    builder.spawn((
                        SpriteBundle {
                            sprite: Sprite {
                                custom_size: Some(Vec2::splat(CELL_SIZE)),
                                color: CELL_COLOR,
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
                        CellState { alive: false }
                    ));
                }
            }
        });
}
