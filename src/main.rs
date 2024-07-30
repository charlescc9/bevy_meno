use rand::prelude::*;
use std::collections::HashSet;

use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

const NUM_CELLS: i32 = 500;
const WINDOW_WIDTH: f32 = 1600.0;
const WINDOW_HEIGHT: f32 = 1048.0;
const CELL_SIZE: f32 = 16.0;
const BORDER_SIZE: f32 = 2.0;
const BOARD_SIZE: i32 = 64;
const BOARD_COLOR: Color = Color::srgb(0.5, 0.5, 0.5);
const ALIVE_CELL_COLOR: Color = Color::WHITE;
const DEAD_CELL_COLOR: Color = Color::BLACK;

#[derive(Resource)]
struct GameTimer(Timer);

#[derive(Resource)]
struct GameState {
    board: [[CellState; BOARD_SIZE as usize]; BOARD_SIZE as usize]
}

impl Default for GameState {
    fn default() -> Self {
        GameState {
            board: [[CellState::Dead; BOARD_SIZE as usize]; BOARD_SIZE as usize]
        }
    }
}

#[derive(Component)]
struct CellPosition {
    x: i32,
    y: i32,
}

#[derive(Component, Clone, Copy, Debug)]
enum CellState {
    Alive,
    Dead,
}

#[derive(Bundle)]
struct CellBundle {
    cell_position: CellPosition,
    state: CellState,
    material_mesh_bundle: MaterialMesh2dBundle<ColorMaterial>,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Spawn camera
    commands.spawn(Camera2dBundle::default());

    // Spawn grid
    for i in -(BOARD_SIZE / 2)..=(BOARD_SIZE / 2) {
        commands.spawn(MaterialMesh2dBundle {
            mesh: Mesh2dHandle(
                meshes.add(Rectangle::new(BORDER_SIZE, CELL_SIZE * BOARD_SIZE as f32)),
            ),
            material: materials.add(BOARD_COLOR),
            transform: Transform::from_xyz(i as f32 * CELL_SIZE, 0.0, 0.0),
            ..default()
        });
    }
    for i in -(BOARD_SIZE / 2)..=(BOARD_SIZE / 2) {
        commands.spawn(MaterialMesh2dBundle {
            mesh: Mesh2dHandle(
                meshes.add(Rectangle::new(CELL_SIZE * BOARD_SIZE as f32, BORDER_SIZE)),
            ),
            material: materials.add(BOARD_COLOR),
            transform: Transform::from_xyz(0.0, i as f32 * CELL_SIZE, 0.0),
            ..default()
        });
    }
}

fn spawn_cells(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut game_state: ResMut<GameState>
) {
    let cell_mesh_handle = Mesh2dHandle(meshes.add(Rectangle::new(
        CELL_SIZE - BORDER_SIZE,
        CELL_SIZE - BORDER_SIZE,
    )));

    let mut occupied_cells: HashSet<(i32, i32)> = HashSet::new();
    let mut rng = rand::thread_rng();

    for _ in 0..NUM_CELLS {
        
        // Get new board index
        let mut i = rng.gen_range(-(BOARD_SIZE / 2)..(BOARD_SIZE / 2));
        let mut j = rng.gen_range(-(BOARD_SIZE / 2)..(BOARD_SIZE / 2));    
        loop {
            if !occupied_cells.contains(&(i, j)) {
                occupied_cells.insert((i, j));
                break;
            } else {
                i = rng.gen_range(-(BOARD_SIZE / 2)..(BOARD_SIZE / 2));
                j = rng.gen_range(-(BOARD_SIZE / 2)..(BOARD_SIZE / 2));    
            }
        }
        let i_absolute = i + (BOARD_SIZE / 2);
        let j_absolute = j + (BOARD_SIZE / 2);

        // Spawn cell
        commands.spawn(CellBundle {
            cell_position: CellPosition { x: i_absolute, y: j_absolute },
            state: CellState::Dead,
            material_mesh_bundle: MaterialMesh2dBundle {
                mesh: cell_mesh_handle.clone(),
                material: materials.add(ALIVE_CELL_COLOR),
                transform: Transform::from_xyz(
                    i as f32 * CELL_SIZE + (CELL_SIZE / 2.0),
                    j as f32 * CELL_SIZE + (CELL_SIZE / 2.0),
                    0.0,
                ),
                ..default()
            },
        });

        // Update board
        game_state.board[i_absolute as usize][j_absolute as usize] = CellState::Alive;
    }
}

fn update(
    time: Res<Time>,
    mut timer: ResMut<GameTimer>,
    mut query: Query<(&mut CellState, &Handle<ColorMaterial>)>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    game_state: ResMut<GameState>
) {
    if timer.0.tick(time.delta()).just_finished() {
        for (mut state, handle) in &mut query {
            match *state {
                CellState::Alive => {
                    *state = CellState::Dead;
                    if let Some(material) = materials.get_mut(handle) {
                        material.color = DEAD_CELL_COLOR;
                    }
                }
                CellState::Dead => {
                    *state = CellState::Alive;
                    if let Some(material) = materials.get_mut(handle) {
                        material.color = ALIVE_CELL_COLOR;
                    }
                }
            }
        }
    }
}

fn exit_system(mut exit: EventWriter<AppExit>, keyboard: Res<ButtonInput<KeyCode>>) {
    if keyboard.just_pressed(KeyCode::Escape) {
        exit.send(AppExit::Success);
    }
}

fn main() {
    App::new()
        .insert_resource(ClearColor(DEAD_CELL_COLOR))
        .insert_resource(GameTimer(Timer::from_seconds(1.0, TimerMode::Repeating)))
        .insert_resource(GameState::default())
        .add_plugins((DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Meno".into(),
                resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                ..default()
            }),
            ..default()
        }),))
        .add_systems(Startup, (setup, spawn_cells).chain())
        .add_systems(Update, (update, exit_system))
        .run();
}
