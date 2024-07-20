use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle, Wireframe2dPlugin},
};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, Wireframe2dPlugin))
        .add_systems(Startup, setup)
        .add_systems(Update, exit_system)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    let cell_size = 16.0;
    let border_size = 2.0;
    let num_rows = 20;

    for i in -num_rows..=num_rows {
        commands.spawn(MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Rectangle::new(
                border_size,
                2.0 * cell_size * num_rows as f32,
            ))),
            material: materials.add(Color::srgb(0.5, 0.5, 0.5)),
            transform: Transform::from_xyz(i as f32 * cell_size, 0.0, 0.0),
            ..default()
        });
    }

    for i in -num_rows..=num_rows {
        commands.spawn(MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Rectangle::new(
                2.0 * cell_size * num_rows as f32,
                border_size,
            ))),
            material: materials.add(Color::srgb(0.5, 0.5, 0.5)),
            transform: Transform::from_xyz(0.0, i as f32 * cell_size, 0.0),
            ..default()
        });
    }

    for i in -num_rows..num_rows {
        for j in -num_rows..num_rows {
            commands.spawn(MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(Rectangle::new(
                    cell_size - border_size,
                    cell_size - border_size,
                ))),
                material: materials.add(Color::srgb(1.0, 1.0, 1.0)),
                transform: Transform::from_xyz(
                    i as f32 * cell_size + (cell_size / 2.0),
                    j as f32 * cell_size + (cell_size / 2.0),
                    0.0,
                ),
                ..default()
            });
        }
    }
}

fn exit_system(mut exit: EventWriter<AppExit>, keyboard: Res<ButtonInput<KeyCode>>) {
    if keyboard.just_pressed(KeyCode::Escape) {
        exit.send(AppExit::Success);
    }
}
