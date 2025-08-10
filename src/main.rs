use bevy::prelude::*;

#[derive(Component)]
struct Ground;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(
            Startup,
            (spawn_floor, spawn_camera, spawn_light, spawn_robot),
        )
        .add_systems(Update, (draw_cursor, robot_move))
        .run();
}

fn draw_cursor(
    camera_query: Single<(&Camera, &GlobalTransform)>,
    ground: Single<&GlobalTransform, With<Ground>>,
    window: Single<&Window>,
    mut gizmos: Gizmos,
) {
    let (camera, camera_transform) = *camera_query;

    if let Some(cursor_position) = window.cursor_position()
        // Calculate a ray pointing from the camera into the world based on the cursor's position.
        && let Ok(ray) = camera.viewport_to_world(camera_transform, cursor_position)
        // Calculate if and at what distance the ray is hitting the ground plane.
        && let Some(distance) =
            ray.intersect_plane(ground.translation(), InfinitePlane3d::new(ground.up()))
    {
        let point = ray.get_point(distance);

        // Draw a circle just above the ground plane at that position.
        gizmos.circle(
            Isometry3d::new(
                point + ground.up() * 0.01,
                Quat::from_rotation_arc(Vec3::Z, ground.up().as_vec3()),
            ),
            0.2,
            Color::BLACK,
        );
    }
}

//-------------------------------------------------------------------------
//                        robot component
//-------------------------------------------------------------------------
#[derive(Component)]
struct Robot;

// TODO(elsuizo: 2025-07-21): cuando hacemos una
fn robot_move(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut player_query: Query<&mut Transform, With<Robot>>,
    camera_query: Query<&Transform, (With<Camera3d>, Without<Robot>)>,
) {
    for mut robot_transform in player_query.iter_mut() {
        // tratamos de obtener la camara
        let cam = match camera_query.single() {
            Ok(c) => c,
            Err(e) => Err(format!("Error retrieving the camerabbb: {}", e)).unwrap(),
        };
        // hacemos que el robot se mueva
        let mut direction = Vec3::ZERO;

        // forward
        if keys.pressed(KeyCode::KeyW) {
            direction += *cam.forward();
        }
        // back
        if keys.pressed(KeyCode::KeyS) {
            direction += *cam.back();
        }
        // left
        if keys.pressed(KeyCode::KeyD) {
            direction += *cam.left();
        }
        // right
        if keys.pressed(KeyCode::KeyA) {
            direction += *cam.right();
        }
        direction.y = 0.0;
        let movement = direction.normalize_or_zero() * 2.0 * time.delta_secs();
        robot_transform.translation += movement;
    }
}

fn spawn_robot(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::BLACK)),
        Robot,
    ));
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(15.0, 5.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

fn spawn_light(mut commands: Commands) {
    commands.spawn((
        DirectionalLight::default(),
        Transform::from_translation(Vec3::ONE).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

fn spawn_floor(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(20., 20.))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
        Ground,
    ));
}
