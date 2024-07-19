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

    for i in -20..=20 {
        commands.spawn(MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Rectangle::new(5.0, 2000.0))),
            material: materials.add(Color::srgb(0.5, 0.5, 0.5)),
            transform: Transform::from_xyz(i as f32 * 50.0, 0.0, 0.0),
            ..default()
        });
    }

    for i in -20..=20 {
        commands.spawn(MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Rectangle::new(2000.0, 5.0))),
            material: materials.add(Color::srgb(0.5, 0.5, 0.5)),
            transform: Transform::from_xyz(0.0, i as f32 * 50.0, 0.0),
            ..default()
        });
    }

    commands.spawn(MaterialMesh2dBundle {
        mesh: Mesh2dHandle(meshes.add(Rectangle::new(45.0, 45.0))),
        material: materials.add(Color::srgb(1.0, 1.0, 1.0)),
        transform: Transform::from_xyz(25.0, 25.0, 0.0),
        ..default()
    });
}

fn exit_system(mut exit: EventWriter<AppExit>, keyboard: Res<ButtonInput<KeyCode>>) {
    if keyboard.just_pressed(KeyCode::Escape) {
        exit.send(AppExit::Success);
    }
}
