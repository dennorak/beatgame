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
        // .insert_resource(ClearColor(Color::BLACK))
        .add_systems(Startup, setup)
        .add_systems(Update, keyboard_input)
        .add_systems(Update, unlight_markers)
        .add_systems(FixedUpdate, stream_keys)
        .add_systems(FixedUpdate, prune_keys)
        .insert_resource(Time::<Fixed>::from_seconds(0.008))
        .run();
}

fn create_anim(label: &Name, x: f32) -> AnimationClip {
    let mut animation = AnimationClip::default();
    // A curve can modify a single part of a transform, here the translation
    animation.add_curve_to_path(
        EntityPath {
            parts: vec![label.clone()],
        },
        VariableCurve {
            keyframe_timestamps: vec![0.0, 0.125, 0.25],
            keyframes: Keyframes::Translation(vec![
                Vec3::new(x, -0.125, 0.0),
                Vec3::new(x, 0.0, 0.0),
                // loop back
                Vec3::new(x, -0.125, 0.0),
            ]),
        },
    );
    return animation;
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut animations: ResMut<Assets<AnimationClip>>,
) {
    // name labels for amin
    let left = Name::new("left");
    let right = Name::new("right");

    // right anim
    let r_anim = create_anim(&right, 0.5);
    let mut r_player = AnimationPlayer::default();
    r_player.play(animations.add(r_anim));

    // left anim
    let l_anim = create_anim(&left, -0.5);
    let mut l_player = AnimationPlayer::default();
    l_player.play(animations.add(l_anim));

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
        transform: Transform::from_xyz(0.0, 8.0, 4.0)
            .looking_at(Vec3::new(0.0, 0.0, -10.0), Vec3::Y),
        ..default()
    });

    let rail_mesh = meshes.add(Mesh::from(shape::Box::from_corners(
        Vec3::new(0.0625, 0.05, 1.0),
        Vec3::new(-0.0625, 0.0, -100.0),
    )));
    let rail_mat = materials.add(Color::rgb(0.0, 0.0, 0.0).into());

    let marker_mesh = meshes.add(Mesh::from(shape::Box::from_corners(
        Vec3::new(-0.375, 0.0, -0.375),
        Vec3::new(0.375, 0.125, 0.375),
    )));

    // board
    commands.spawn((PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Box::from_corners(
            Vec3::new(1.0, 0.0, 1.0),
            Vec3::new(-1.0, 0.0, -100.0),
        ))),
        material: materials.add(Color::rgb(0.3, 0.3, 0.3).into()),
        ..default()
    },));

    // center rail
    commands.spawn((PbrBundle {
        mesh: rail_mesh.clone(),
        material: rail_mat.clone(),
        ..default()
    },));

    // left rail
    commands.spawn((PbrBundle {
        mesh: rail_mesh.clone(),
        material: rail_mat.clone(),
        transform: Transform::from_xyz(-1.0, 0.0, 0.0),
        ..default()
    },));

    // right rail
    commands.spawn((PbrBundle {
        mesh: rail_mesh.clone(),
        material: rail_mat.clone(),
        transform: Transform::from_xyz(1.0, 0.0, 0.0),
        ..default()
    },));

    // right marker
    commands.spawn((
        PbrBundle {
            mesh: marker_mesh.clone(),
            material: materials.add(Color::rgba(0.78, 0.36, 0.36, 0.5).into()),
            transform: Transform::from_xyz(0.5, -0.125, 0.0),
            ..default()
        },
        r_player,
        right,
        RightIndicator,
    ));

    // left marker
    commands.spawn((
        PbrBundle {
            mesh: marker_mesh.clone(),
            material: materials.add(Color::rgba(0.36, 0.36, 0.78, 0.5).into()),
            transform: Transform::from_xyz(-0.5, -0.125, 0.0),
            ..default()
        },
        l_player,
        left,
        LeftIndicator,
    ));
}

fn unlight_markers(
    marks: Query<(&Handle<StandardMaterial>, &AnimationPlayer)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    marks.for_each(|(mat, anim)| {
        if anim.is_finished() {
            if let Some(mat) = materials.get_mut(mat) {
                mat.base_color.set_a(0.5);
            }
        }
    });
}

fn keyboard_input(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    keys: Res<Input<KeyCode>>,
    mut set: ParamSet<(
        Query<(&Handle<StandardMaterial>, &mut AnimationPlayer), With<LeftIndicator>>,
        Query<(&Handle<StandardMaterial>, &mut AnimationPlayer), With<RightIndicator>>,
    )>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if keys.just_pressed(KeyCode::F) {
        if let Ok((handle, mut anim)) = set.p0().get_single_mut() {
            if let Some(mat) = materials.get_mut(handle) {
                mat.base_color.set_a(1.0);
            }
            anim.replay();
        }

        commands.spawn((
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Box::from_corners(
                    Vec3::new(-0.375, 0.0, -0.375),
                    Vec3::new(0.375, 0.125, 0.375),
                ))),
                material: materials.add(Color::rgba(0.36, 0.36, 0.78, 0.5).into()),
                transform: Transform::from_xyz(-0.5, 0.0, 0.0),
                ..default()
            },
            MovingNote,
        ));
    }

    if keys.just_pressed(KeyCode::J) {
        if let Ok((handle, mut anim)) = set.p1().get_single_mut() {
            if let Some(mat) = materials.get_mut(handle) {
                mat.base_color.set_a(1.0);
            }
            anim.replay();
        }

        commands.spawn((
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Box::from_corners(
                    Vec3::new(-0.375, 0.0, -0.375),
                    Vec3::new(0.375, 0.125, 0.375),
                ))),
                material: materials.add(Color::rgba(0.78, 0.36, 0.36, 0.5).into()),
                transform: Transform::from_xyz(0.5, 0.0, 0.0),
                ..default()
            },
            MovingNote,
        ));
    }
}

fn stream_keys(mut movingnotes: Query<&mut Transform, With<MovingNote>>) {
    movingnotes.for_each_mut(|mut transf| {
        transf.translation.z -= 0.1;
    })
}

fn prune_keys(mut commands: Commands, keys: Query<(Entity, &Transform)>) {
    keys.for_each(|(entity, transf)| {
        if transf.translation.z <= -100.0 {
            commands.entity(entity).despawn();
        }
    })
}
