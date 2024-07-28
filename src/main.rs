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
const NUM_ROWS: i32 = 64;
const GRID_COLOR: Color = Color::srgb(0.5, 0.5, 0.5);
const ALIVE_CELL_COLOR: Color = Color::WHITE;
const DEAD_CELL_COLOR: Color = Color::BLACK;

#[derive(Resource)]
struct GameTimer(Timer);

#[derive(Component)]
enum CellState {
    Alive,
    Dead,
}

#[derive(Bundle)]
struct CellBundle {
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
    for i in -(NUM_ROWS / 2)..=(NUM_ROWS / 2) {
        commands.spawn(MaterialMesh2dBundle {
            mesh: Mesh2dHandle(
                meshes.add(Rectangle::new(BORDER_SIZE, CELL_SIZE * NUM_ROWS as f32)),
            ),
            material: materials.add(GRID_COLOR),
            transform: Transform::from_xyz(i as f32 * CELL_SIZE, 0.0, 0.0),
            ..default()
        });
    }
    for i in -(NUM_ROWS / 2)..=(NUM_ROWS / 2) {
        commands.spawn(MaterialMesh2dBundle {
            mesh: Mesh2dHandle(
                meshes.add(Rectangle::new(CELL_SIZE * NUM_ROWS as f32, BORDER_SIZE)),
            ),
            material: materials.add(GRID_COLOR),
            transform: Transform::from_xyz(0.0, i as f32 * CELL_SIZE, 0.0),
            ..default()
        });
    }
}

fn spawn_cells(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let cell_mesh_handle = Mesh2dHandle(meshes.add(Rectangle::new(
        CELL_SIZE - BORDER_SIZE,
        CELL_SIZE - BORDER_SIZE,
    )));

    let mut occupied_cells: HashSet<(i32, i32)> = HashSet::new();
    let mut rng = rand::thread_rng();

    for _ in 0..NUM_CELLS {
        let mut i = rng.gen_range(-(NUM_ROWS / 2)..(NUM_ROWS / 2));
        let mut j = rng.gen_range(-(NUM_ROWS / 2)..(NUM_ROWS / 2));    
        loop {
            if !occupied_cells.contains(&(i, j)) {
                occupied_cells.insert((i, j));
                break;
            } else {
                i = rng.gen_range(-(NUM_ROWS / 2)..(NUM_ROWS / 2));
                j = rng.gen_range(-(NUM_ROWS / 2)..(NUM_ROWS / 2));    
            }
        }
        commands.spawn(CellBundle {
            state: CellState::Dead,
            material_mesh_bundle: MaterialMesh2dBundle {
                mesh: cell_mesh_handle.clone(),
                material: materials.add(DEAD_CELL_COLOR),
                transform: Transform::from_xyz(
                    i as f32 * CELL_SIZE + (CELL_SIZE / 2.0),
                    j as f32 * CELL_SIZE + (CELL_SIZE / 2.0),
                    0.0,
                ),
                ..default()
            },
        });
    }
}

fn update(
    time: Res<Time>,
    mut timer: ResMut<GameTimer>,
    mut query: Query<(&mut CellState, &Handle<ColorMaterial>)>,
    mut materials: ResMut<Assets<ColorMaterial>>,
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
        .insert_resource(GameTimer(Timer::from_seconds(1.0, TimerMode::Repeating)))
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
