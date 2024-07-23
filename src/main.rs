use core::num;

use bevy::{
    prelude::*,
    sprite::{Material2d, MaterialMesh2dBundle, Mesh2dHandle, Wireframe2dPlugin},
};

#[derive(Resource)]
struct GameTimer(Timer);

enum State {
    Alive,
    Dead,
}

#[derive(Component)]
struct Cell {
    x: i32,
    y: i32,
    state: State,
}

#[derive(Bundle)]
struct CellMaterial<M: Material2d> {
    cell: Cell,
    mesh_material: MaterialMesh2dBundle<M>,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    let cell_size = 16.0;
    let border_size = 2.0;
    let num_rows = 64;

    // Spawn grid
    for i in -(num_rows / 2)..=(num_rows / 2) {
        commands.spawn(MaterialMesh2dBundle {
            mesh: Mesh2dHandle(
                meshes.add(Rectangle::new(border_size, cell_size * num_rows as f32)),
            ),
            material: materials.add(Color::srgb(0.5, 0.5, 0.5)),
            transform: Transform::from_xyz(i as f32 * cell_size, 0.0, 0.0),
            ..default()
        });
    }
    for i in -(num_rows / 2)..=(num_rows / 2) {
        commands.spawn(MaterialMesh2dBundle {
            mesh: Mesh2dHandle(
                meshes.add(Rectangle::new(cell_size * num_rows as f32, border_size)),
            ),
            material: materials.add(Color::srgb(0.5, 0.5, 0.5)),
            transform: Transform::from_xyz(0.0, i as f32 * cell_size, 0.0),
            ..default()
        });
    }

    // Spawn cells
    for i in -(num_rows / 2)..(num_rows / 2) {
        for j in -(num_rows / 2)..(num_rows / 2) {
            commands.spawn(CellMaterial {
                cell: Cell {
                    x: 1,
                    y: 1,
                    state: State::Alive,
                },
                mesh_material: MaterialMesh2dBundle {
                    mesh: Mesh2dHandle(meshes.add(Rectangle::new(
                        cell_size - border_size,
                        cell_size - border_size,
                    ))),
                    material: materials.add(Color::srgb(
                        i32::abs(i) as f32 / num_rows as f32,
                        i32::abs(i) as f32 / num_rows as f32,
                        i32::abs(i) as f32 / num_rows as f32,
                    )),
                    transform: Transform::from_xyz(
                        i as f32 * cell_size + (cell_size / 2.0),
                        j as f32 * cell_size + (cell_size / 2.0),
                        0.0,
                    ),
                    ..default()
                },
            });
        }
    }
}

fn update(time: Res<Time>, mut timer: ResMut<GameTimer>) {
    if timer.0.tick(time.delta()).just_finished() {
        println!("test");
    }
}

fn exit_system(mut exit: EventWriter<AppExit>, keyboard: Res<ButtonInput<KeyCode>>) {
    if keyboard.just_pressed(KeyCode::Escape) {
        exit.send(AppExit::Success);
    }
}

fn main() {
    App::new()
        .insert_resource(GameTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Meno".into(),
                    resolution: (1600., 1048.).into(),
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
