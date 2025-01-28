use bevy::prelude::*;

#[cfg(feature = "egui")]
use bevy_egui::{egui, EguiContexts, EguiPlugin};

fn main() {
    let mut app = App::new();

    app
        .add_plugins(DefaultPlugins)
        .insert_resource(AmbientLight {
            brightness: 750.0,
            ..default()
        })
        .add_systems(Startup, setup);

    #[cfg(feature = "egui")]
    app
        .add_plugins(EguiPlugin)
        .add_systems(Update, ui_system);

    app
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Create a camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::new(0.0, 1.0, 0.0), Vec3::Y),
    ));

    commands.spawn(SceneRoot(asset_server.load(
        GltfAssetLabel::Scene(0).from_asset("models/iroha.glb"),
    )));
}

#[cfg(feature = "egui")]
fn ui_system(mut contexts: EguiContexts) {
    egui::Window::new("Hello").show(contexts.ctx_mut(), |ui| {
        ui.label("world");
    });
}