use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle, Wireframe2dPlugin},
};

const WINDOW_WIDTH: f32 = 1600.0;
const WINDOW_HEIGHT: f32 = 1048.0;
const CELL_SIZE: f32 = 16.0;
const BORDER_SIZE: f32 = 2.0;
const NUM_ROWS: i32 = 64;

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
    // Add assets
    let cell_mesh_handle = Mesh2dHandle(meshes.add(Rectangle::new(
        CELL_SIZE - BORDER_SIZE,
        CELL_SIZE - BORDER_SIZE,
    )));

    // Spawn camera
    commands.spawn(Camera2dBundle::default());

    // Spawn grid
    for i in -(NUM_ROWS / 2)..=(NUM_ROWS / 2) {
        commands.spawn(MaterialMesh2dBundle {
            mesh: Mesh2dHandle(
                meshes.add(Rectangle::new(BORDER_SIZE, CELL_SIZE * NUM_ROWS as f32)),
            ),
            material: materials.add(Color::srgb(0.5, 0.5, 0.5)),
            transform: Transform::from_xyz(i as f32 * CELL_SIZE, 0.0, 0.0),
            ..default()
        });
    }
    for i in -(NUM_ROWS / 2)..=(NUM_ROWS / 2) {
        commands.spawn(MaterialMesh2dBundle {
            mesh: Mesh2dHandle(
                meshes.add(Rectangle::new(CELL_SIZE * NUM_ROWS as f32, BORDER_SIZE)),
            ),
            material: materials.add(Color::srgb(0.5, 0.5, 0.5)),
            transform: Transform::from_xyz(0.0, i as f32 * CELL_SIZE, 0.0),
            ..default()
        });
    }

    // Spawn cells
    for i in -(NUM_ROWS / 2)..(NUM_ROWS / 2) {
        for j in -(NUM_ROWS / 2)..(NUM_ROWS / 2) {
            commands.spawn(CellBundle {
                state: CellState::Dead,
                material_mesh_bundle: MaterialMesh2dBundle {
                    mesh: cell_mesh_handle.clone(),
                    material: materials.add(Color::BLACK),
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
                        material.color = Color::BLACK;
                    }
                }
                CellState::Dead => {
                    *state = CellState::Alive;
                    if let Some(material) = materials.get_mut(handle) {
                        material.color = Color::WHITE;
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
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Meno".into(),
                    resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                    ..default()
                }),
                ..default()
            }),
            Wireframe2dPlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, (update, exit_system))
        .run();
}
