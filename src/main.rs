use bevy::{
    prelude::*,
    utils::HashMap,
};
use rand::Rng;

// Constants
// These constants are defined in `Transform` units.
// Using the default 2D camera they correspond 1:1 with screen pixels.

const GRID_WIDTH: f32 = 100.;
const GRID_HEIGHT: f32 = 100.;

const GRID_WIDTH_OFFSET: f32 = GRID_WIDTH * 0.5;
const GRID_HEIGHT_OFFSET: f32 = GRID_HEIGHT * 0.5;

// Walls
const WALL_THICKNESS: f32 = 10.0;
const LEFT_WALL: f32 = -450.;
const RIGHT_WALL: f32 = 450.;
const BOTTOM_WALL: f32 = -300.;
const TOP_WALL: f32 = 300.;

// Cell
const CELL_SIZE: Vec2 = Vec2::new(10.0, 10.0);

// Colors
const BACKGROUND_COLOR: Color = Color::srgb(1.0, 1.0, 1.0);
const WALL_COLOR: Color = Color::srgb(0.3, 0.3, 0.3);
const CELL_COLOR: Color = Color::srgb(0., 0., 0.);
const CELL_OFF_COLOR: Color = Color::srgb(1.0, 0.4, 0.1);

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
            WallLocation::Left => Vec2::new(-GRID_WIDTH_OFFSET - WALL_THICKNESS, -WALL_THICKNESS),
            WallLocation::Right => Vec2::new(GRID_WIDTH - GRID_WIDTH_OFFSET - WALL_THICKNESS, -WALL_THICKNESS),
            WallLocation::Bottom => Vec2::new(-WALL_THICKNESS, -GRID_HEIGHT_OFFSET - WALL_THICKNESS),
            WallLocation::Top => Vec2::new(-WALL_THICKNESS, GRID_HEIGHT - GRID_HEIGHT_OFFSET - WALL_THICKNESS),
        }
    }

    fn size(&self) -> Vec2 {
        match self {
            WallLocation::Left | WallLocation::Right => {
                Vec2::new(WALL_THICKNESS, GRID_HEIGHT + (2.0 * WALL_THICKNESS))
            }
            WallLocation::Bottom | WallLocation::Top => {
                Vec2::new(GRID_WIDTH + (2.0 * WALL_THICKNESS), WALL_THICKNESS)
            }
        }
    }

    fn color(&self) -> Color {
        match self {
            WallLocation::Left => Color::srgb(0.0, 0.0, 0.0),
            WallLocation::Right => Color::srgb(1.0, 0.0, 0.0),
            WallLocation::Bottom => Color::srgb(0.0, 0.0, 0.0),
            WallLocation::Top => Color::srgb(0.0, 1.0, 0.0),
        }
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
                    scale: location.size().extend(1.0),
                    ..default() 
                },
                sprite: Sprite {
                    color: location.color(),
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

struct CellState {
    alive: bool,
}

#[derive(Clone, Resource)]
struct CellMap {
    cells: HashMap<IVec2, Entity>,
}

// Setups
fn setup_camera(mut commands: Commands) {
    // Camera
    commands.spawn(Camera2dBundle::default());
}

fn setup(mut commands: Commands) {

    commands.spawn(WallBundle::new(WallLocation::Left));
    commands.spawn(WallBundle::new(WallLocation::Right));
    commands.spawn(WallBundle::new(WallLocation::Top));
    commands.spawn(WallBundle::new(WallLocation::Bottom));


    commands
        .spawn(SpatialBundle::from_transform(Transform::from_xyz(
                    -GRID_WIDTH_OFFSET,
                    -GRID_HEIGHT_OFFSET,
                    0.
                )))
        .with_children(|builder| {
    // Walls
//    builder.spawn(WallBundle::new(WallLocation::Left));
//    builder.spawn(WallBundle::new(WallLocation::Right));
//    builder.spawn(WallBundle::new(WallLocation::Top));
//    builder.spawn(WallBundle::new(WallLocation::Bottom));

            for y in 0..(GRID_HEIGHT as i32) {
                for x in 0..(GRID_WIDTH as i32) {
                    let color = if (x + y) % 2 == 0 { CELL_COLOR } else { CELL_OFF_COLOR };
                    println!("x: {}, y: {}    {}", x, y, (x + y) % 2 == 0);
                    builder.spawn((
                            SpriteBundle {
                                transform: Transform {
                                    translation: Vec3::new((x as f32), (y as f32), 0.0),
                                    scale: CELL_SIZE.extend(1.0),
                                    ..default()
                                },
                                sprite: Sprite {
                                    color: color,
                                    ..default()
                                },
                                ..default()
                            },
                    ));
                }
            }

     builder.spawn((
                        SpriteBundle {
                            sprite: Sprite {
                                color: Color::srgb(0.5, 0., 1.0), 
                                ..default()
                            },
                            transform: Transform {
                                translation: Vec3::new(0.0, 0.0, 0.),
                                scale: Vec3::new(5.0, 5.0, 1.),
                                ..default()
                            },
                            ..default()
                        },
                    ));



        });

    println!("Generated!");

}
    /*


    for y in 0..(GRID_HEIGHT as i32) {
        for x in 0..(GRID_WIDTH as i32) {
            let color = if (x + y) % 2 == 0 { CELL_COLOR } else { CELL_OFF_COLOR };
            println!("x: {}, y: {}    {}", x, y, (x + y) % 2 == 0);
            commands.spawn((
                    SpriteBundle {
                        transform: Transform {
                            translation: Vec3::new((x as f32)- GRID_WIDTH_OFFSET, - GRID_HEIGHT_OFFSET + (y as f32), 0.0),
                            scale: CELL_SIZE.extend(1.0),
                            ..default()
                        },
                        sprite: Sprite {
                            color: color,
                            ..default()
                        },
                        ..default()
                    },
            ));
        }
    }
}

     builder.spawn((
                        SpriteBundle {
                            sprite: Sprite {
                                color: Color::srgb(0.5, 0., 1.0), 
                                ..default()
                            },
                            transform: Transform {
                                translation: Vec3::new(0.0, 50.0, 0.),
                                scale: Vec3::new(10.0, 10.0, 1.),
                                ..default()
                            },
                            ..default()
                        },
                    ));



     builder.spawn((
                        SpriteBundle {
                            sprite: Sprite {
                                color: Color::srgb(0., 0., 0.), 
                                ..default()
                            },
                            transform: Transform {
                                translation: Vec3::new(100.0, 100.0, 0.),
                                scale: Vec3::new(10.0, 10.0, 1.),
                                ..default()
                            },
                            ..default()
                        },
                    ));

*/
