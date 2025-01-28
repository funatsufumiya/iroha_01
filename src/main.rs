use bevy::prelude::*;

use bevy_asset_loader::asset_collection::AssetCollection;

#[cfg(feature = "egui")]
use bevy_egui::{egui, EguiContexts, EguiPlugin};

fn main() {
    use bevy_asset_loader::loading_state::{config::ConfigureLoadingState, LoadingState, LoadingStateAppExt};

    let mut app = App::new();

    app
        .add_plugins(DefaultPlugins)
        .insert_resource(AmbientLight {
            brightness: 750.0,
            ..default()
        })
        .init_state::<AssetLoadingState>()
        .add_loading_state(
            LoadingState::new(AssetLoadingState::Loading)
                .continue_to_state(AssetLoadingState::Loaded)
                .load_collection::<GltfAssets>()
        )
        .add_systems(OnEnter(AssetLoadingState::Loaded), setup);

    #[cfg(feature = "egui")]
    app
        .add_plugins(EguiPlugin)
        .add_systems(Update, ui_system);

    app
        .run();
}

#[derive(AssetCollection, Resource)]
pub struct GltfAssets {
  #[asset(path = "models/iroha.glb")]
  pub iroha: Handle<Gltf>,
}

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum AssetLoadingState {
    #[default]
    Loading,
    Loaded,
}

fn setup(
    mut commands: Commands,
    gltf_res: Res<GltfAssets>,
    assets_gltf: Res<Assets<Gltf>>,
) {
    // Create a camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::new(0.0, 1.0, 0.0), Vec3::Y),
    ));

    let gltf = assets_gltf.get(&gltf_res.iroha).unwrap();
    let scene = gltf.scenes[0].clone();
    // let scene = GltfAssetLabel::Scene(0).from_asset("models/iroha.glb");

    commands.spawn((
        SceneRoot(scene),
        // Transform::from_xyz(0.0 , 0.0, 0.0),
    ));
}

#[cfg(feature = "egui")]
fn ui_system(mut contexts: EguiContexts) {
    egui::Window::new("Hello").show(contexts.ctx_mut(), |ui| {
        ui.label("world");
    });
}