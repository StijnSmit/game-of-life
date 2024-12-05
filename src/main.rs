use bevy::{
    prelude::*,
    window::PrimaryWindow,
};

use std::collections::HashMap;

pub mod cell;
pub mod cell_state;
pub mod wall;
pub mod lifes;

use crate::cell::Cell; use crate::cell_state::CellState; use crate::wall::{WallLocation, WallBundle};

const GRID_WIDTH: f32 = 40.;
const GRID_HEIGHT: f32 = 40.;

// Walls
const WALL_THICKNESS: f32 = 15.0;

// Cell
pub const CELL_SIZE: f32 = 10.;

// Colors
const BACKGROUND_COLOR: Color = Color::srgb(1.0, 1.0, 1.0);
const WALL_COLOR: Color = Color::srgb(0.2, 0.2, 0.2);
const CELL_COLOR: Color = Color::srgb(0.3, 0.3, 0.3);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<MyGameModeState>()
        .insert_resource(Time::<Fixed>::from_hz(2.0))
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .init_resource::<MyWorldCoords>()
        .add_systems(Startup, setup_resolution)
        .add_systems(Startup, setup_camera)
        .add_systems(Startup, setup)
        .add_systems(Update, (my_cursor_system, mouse_input, keyboard_input))
        .add_systems(FixedUpdate, update_cells.run_if(in_state(MyGameModeState::Playing)))
        .add_systems(Update, draw_cells)
        .run();
}

// Sets up a shape at a position
fn setup_shape(shape: &[IVec2], position: IVec2) -> Vec<IVec2>{
    shape.iter().map(|&cell| cell + position).collect()
}

#[derive(Resource, Default)]
struct MyWorldCoords(Vec2);

#[derive(Component)]
struct MainCamera;

fn setup_resolution(
    mut windows: Query<&mut Window>
) {
    let mut window = windows.single_mut();
    window.resolution.set(700., 500.);
}

// Setups
fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), MainCamera));
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
                            visibility: Visibility::Hidden,
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
                        CellState { is_alive: false }
                    ));
                }
            }
        });
}

fn my_cursor_system(
    mut mycoords: ResMut<MyWorldCoords>,
    // query to get the window (so we can read the current cursor position)
    q_window: Query<&Window, With<PrimaryWindow>>,
    // query to get the camera transform
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    // Get the camera info and transform
    let (camera, camera_transform) = q_camera.single();

    // There is only one primary window, so can use single
    let window = q_window.single();

    // Check if the cursor is inside the window and get its position
    // then, ask bevy to convert  into world coordinates, nad truncate to discard Z
    if let Some(world_position) = window.cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        mycoords.0 = world_position;
    }
}

fn mouse_input(
    mycoords: Res<MyWorldCoords>,
    buttons: Res<ButtonInput<MouseButton>>,
    mut query: Query<(&Cell, &mut CellState)>,
) {
    let mut position = mycoords.0.as_ivec2();// + IVec2::new(CELL_SIZE as i32 / 2, CELL_SIZE as i32 / 2);
    let half_size = CELL_SIZE as i32 / 2;
    if position.x > 0 {
        position.x += half_size;
    } else {
        position.x -= half_size;
    }
    if position.y > 0 {
        position.y += half_size;
    } else {
        position.y -= half_size;
    }
    let localized_position = (position / (CELL_SIZE as i32)) + IVec2::new(GRID_WIDTH as i32 / 2, GRID_HEIGHT as i32 / 2);


    if buttons.just_pressed(MouseButton::Left) {
        println!("{} l: {}", position, localized_position);
        // fn setup_shape(shape: &[IVec2], position: IVec2) -> Vec<IVec2>{
        let shape = setup_shape(&lifes::PULSAR_SHAPE, localized_position);
        println!("{:?}", shape);

        for (cell, mut state) in &mut query {
            if shape.contains(&cell.coords) {
                state.toggle();
            }
        }
    }
}

fn keyboard_input(
    keys: Res<ButtonInput<KeyCode>>,
    state: Res<State<MyGameModeState>>,
    mut next_state: ResMut<NextState<MyGameModeState>>,
) {
    if keys.just_pressed(KeyCode::Space) {
        match state.get() {
            MyGameModeState::Paused => next_state.set(MyGameModeState::Playing),
            MyGameModeState::Playing => next_state.set(MyGameModeState::Paused),
        }
    }
}

fn draw_cells(
    mut query: Query<(&CellState, &mut Visibility)>
) {
    for (cell_state, mut visibility) in &mut query {
        *visibility = if cell_state.is_alive { Visibility::Visible } else { Visibility::Hidden };
    }
}


fn update_cells(
    mut commands: Commands,
    mut query: Query<(Entity, &Cell, &CellState, &mut Visibility)>
) {
    let map: HashMap<IVec2, CellState> = query
        .iter()
        .map(|(_, cell, state, _)| (cell.coords.clone(), state.clone()))
        .collect();

    for (entity, cell, cell_state, visibility) in &mut query {
        let neighbor_coords = cell.get_neighbor_coords();
        let neighbor_states: Vec<CellState> = neighbor_coords
            .into_iter()
            .filter_map(|i| map.get(&i).cloned())
            .collect();

        let alive_count = neighbor_states.into_iter().filter(|x| x.is_alive == true).count();
        let alive = matches!((cell_state.is_alive, alive_count), (true, 2 | 3) | (false, 3));
        
        commands.entity(entity).try_insert(CellState { is_alive: alive });
    }
    return;
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
enum MyGameModeState {
    #[default]
    Paused,
    Playing,
}

