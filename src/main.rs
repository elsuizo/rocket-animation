use bevy::pbr::{CascadeShadowConfigBuilder, DirectionalLightShadowMap};
use bevy::{prelude::*, render::camera::Viewport, window::PrimaryWindow};
use bevy_egui::{
    EguiContext, EguiContexts, EguiGlobalSettings, EguiPlugin, EguiPrimaryContextPass,
    PrimaryEguiContext, egui,
};
use bevy_render::view::RenderLayers;
use std::f32::consts::*;

// Define a "marker" component to mark the custom mesh. Marker components are often used in Bevy for
// filtering entities in queries with `With`, they're usually not queried directly since they don't
// contain information within them.
#[derive(Component)]
struct Missile;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut egui_global_settings: ResMut<EguiGlobalSettings>,
) {
    egui_global_settings.auto_create_primary_context = false;
    let missile_model = asset_server
        .load(GltfAssetLabel::Scene(0).from_asset("DualSpin/CP30M_STD_2024_DS V_5_Comp.glb"));
    commands.spawn((
        // The `PrimaryEguiContext` component requires everything needed to render a primary context.
        PrimaryEguiContext,
        Camera2d,
        // Setting RenderLayers to none makes sure we won't render anything apart from the UI.
        RenderLayers::none(),
        Camera {
            order: 1,
            ..default()
        },
    ));

    commands.spawn((
        SceneRoot(missile_model),
        Transform::from_xyz(0.0, 0.0, 0.0),
        Missile,
    ));
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(1.2, 1.7, 1.5).looking_at(Vec3::new(0.0, 0.3, 0.0), Vec3::Y),
        EnvironmentMapLight {
            diffuse_map: asset_server.load("environment_maps/pisa_diffuse_rgb9e5_zstd.ktx2"),
            specular_map: asset_server.load("environment_maps/pisa_specular_rgb9e5_zstd.ktx2"),
            intensity: 350.0,
            ..default()
        },
    ));
}

fn main() {
    App::new()
        .insert_resource(DirectionalLightShadowMap { size: 4096 })
        .init_resource::<UiState>()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin::default())
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (move_camera, animate_light_direction, input_handler),
        )
        .add_systems(EguiPrimaryContextPass, ui_system)
        .run();
}

fn input_handler(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mesh_query: Query<&Mesh3d, With<Missile>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut query: Query<&mut Transform, With<Missile>>,
    time: Res<Time>,
) {
    // if keyboard_input.just_pressed(KeyCode::Space) {
    //     let mesh_handle = mesh_query.single().expect("Query not successful");
    //     let mesh = meshes.get_mut(mesh_handle).unwrap();
    //     toggle_texture(mesh);
    // }
    if keyboard_input.pressed(KeyCode::KeyX) {
        for mut transform in &mut query {
            transform.rotate_x(time.delta_secs() / 0.7);
        }
    }
    if keyboard_input.pressed(KeyCode::KeyY) {
        for mut transform in &mut query {
            transform.rotate_y(time.delta_secs() / 1.2);
        }
    }
    if keyboard_input.pressed(KeyCode::KeyZ) {
        for mut transform in &mut query {
            transform.rotate_z(time.delta_secs() / 1.2);
        }
    }
    if keyboard_input.pressed(KeyCode::KeyR) {
        for mut transform in &mut query {
            transform.look_to(Vec3::NEG_Z, Vec3::Y);
        }
    }
}

fn move_camera(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut player_query: Query<&mut Transform, With<Missile>>,
    camera_query: Query<&Transform, (With<Camera3d>, Without<Missile>)>,
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

fn animate_light_direction(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<DirectionalLight>>,
) {
    for mut transform in &mut query {
        transform.rotation = Quat::from_euler(
            EulerRot::ZYX,
            0.0,
            time.elapsed_secs() * PI / 5.0,
            -FRAC_PI_4,
        );
    }
}

#[derive(Default, Resource)]
struct UiState {
    label: String,
    value: f32,
    inverted: bool,
    egui_texture_handle: Option<egui::TextureHandle>,
    is_window_open: bool,
}

fn ui_system(mut ui_state: ResMut<UiState>, mut contexts: EguiContexts) -> Result {
    let ctx = contexts.ctx_mut()?;
    let mut _left = egui::SidePanel::left("left_panel")
        .resizable(true)
        .show(ctx, |ui| {
            ui.label("Valores");
            ui.vertical(|ui| {
                ui.label("Yaw: ");
                ui.label("Picth: ");
                ui.label("Roll: ");
            });
            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        })
        .response
        .rect
        .with_max_y(100.0);

    Ok(())
}
