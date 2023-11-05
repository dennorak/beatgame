/*

    BEATGAME
    - uses two keyboad keys
    - kinda like beatsaber

    partial poc, partially because I want to
    and I think it could be somewhat popular.

    I know I'd like it, even if it's really hard
    and kinda jank

    *note: i should make this one run on the web, cuz I can
    - aside from assets, though I think i can do that w/ 0.12

*/

use bevy::prelude::*;

#[derive(Component)]
struct LeftIndicator;

#[derive(Component)]
struct RightIndicator;

#[derive(Component)]
struct MovingNote;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::BLACK))
        .add_systems(Startup, setup)
        .add_systems(Update, keyboard_input)
        .add_systems(FixedUpdate, stream_keys)
        .insert_resource(Time::<Fixed>::from_seconds(0.016)) 
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {

    // cam
    commands.spawn(Camera3dBundle {
        transform: Transform::from_translation(Vec3::new(0.0, 2.0, 4.0))
            .looking_at(Vec3::new(0.0, 0.0, -1.0), Vec3::Y),
        ..Default::default()
    });

    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 8.0, 4.0),
        ..default()
    });

    // board
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::from_corners(Vec3::new(1.0, 0.0, 1.0), Vec3::new(-1.0, 0.0, -100.0)))),
            material: materials.add(Color::rgb(0.3, 0.3, 0.3).into()),
            ..default()
        },
    ));

    // center rail
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::from_corners(Vec3::new(0.0625, 0.05, 1.0), Vec3::new(-0.0625, 0.0, -100.0)))),
            material: materials.add(Color::rgb(0.0, 0.0, 0.0).into()),
            ..default()
        },
    ));

    // left rail
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::from_corners(Vec3::new(0.0625, 0.05, 1.0), Vec3::new(-0.0625, 0.0, -100.0)))),
            material: materials.add(Color::rgb(0.0, 0.0, 0.0).into()),
            transform: Transform::from_xyz(-1.0, 0.0, 0.0),
            ..default()
        },
    ));

    // right rail
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::from_corners(Vec3::new(0.0625, 0.05, 1.0), Vec3::new(-0.0625, 0.0, -100.0)))),
            material: materials.add(Color::rgb(0.0, 0.0, 0.0).into()),
            transform: Transform::from_xyz(1.0, 0.0, 0.0),
            ..default()
        },
    ));

    // right marker
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::from_corners(Vec3::new(-0.375, 0.0, -0.375), Vec3::new(0.375, 0.125, 0.375)))),
            material: materials.add(Color::rgba(0.78, 0.36, 0.36, 0.5).into()),
            transform: Transform::from_xyz(0.5, 0.0, 0.0),
            ..default()
        },
        RightIndicator
    ));

    // left marker
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::from_corners(Vec3::new(-0.375, 0.0, -0.375), Vec3::new(0.375, 0.125, 0.375)))),
            material: materials.add(Color::rgba(0.36, 0.36, 0.78, 0.5).into()),
            transform: Transform::from_xyz(-0.5, 0.0, 0.0),
            ..default()
        },
        LeftIndicator
    ));
}

fn keyboard_input(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    keys: Res<Input<KeyCode>>,
    mut set: ParamSet<(
        Query<&mut Handle<StandardMaterial>, With<LeftIndicator>>,
        Query<&mut Handle<StandardMaterial>, With<RightIndicator>>,
    )>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    if keys.just_pressed(KeyCode::F) {
        info!("left");
        if let Ok(handle) = set.p0().get_single()
        {
            if let Some(mat) = materials.get_mut(handle)
            {
                mat.base_color.set_a(1.0);
            } else { error!("Failed to get mat") }
        } else { error!("Failed to get handle") }

        commands.spawn((
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Box::from_corners(Vec3::new(-0.375, 0.0, -0.375), Vec3::new(0.375, 0.125, 0.375)))),
                material: materials.add(Color::rgba(0.36, 0.36, 0.78, 0.5).into()),
                transform: Transform::from_xyz(-0.5, 0.0, 0.0),
                ..default()
            },
            MovingNote
        ));
    }

    if keys.just_pressed(KeyCode::J) {
        info!("right");
        if let Ok(handle) = set.p1().get_single()
        {
            if let Some(mat) = materials.get_mut(handle)
            {
                mat.base_color.set_a(1.0);
            } else { error!("Failed to get mat") }
        } else { error!("Failed to get handle") }

        commands.spawn((
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Box::from_corners(Vec3::new(-0.375, 0.0, -0.375), Vec3::new(0.375, 0.125, 0.375)))),
                material: materials.add(Color::rgba(0.78, 0.36, 0.36, 0.5).into()),
                transform: Transform::from_xyz(0.5, 0.0, 0.0),
                ..default()
            },
            MovingNote
        ));
    }
}

fn stream_keys(mut movingnotes: Query<&mut Transform, With<MovingNote>>)
{
    movingnotes.for_each_mut(|mut transf| {
        transf.translation.z -= 0.1;
    })
}