use std::collections::HashMap;

use bevy::{gltf::{GltfMesh, GltfNode}, prelude::*};

use bevy_asset_loader::asset_collection::AssetCollection;

#[cfg(feature = "egui")]
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bimap::BiMap;
use rand::{rngs::StdRng, Rng as _, SeedableRng};

fn main() {
    use bevy_asset_loader::loading_state::{config::ConfigureLoadingState, LoadingState, LoadingStateAppExt};

    let mut app = App::new();

    app
        .add_plugins(DefaultPlugins)
        .insert_resource(AmbientLight {
            brightness: 750.0,
            ..default()
        })
        .insert_resource(Iroha {
            mesh_map: BiMap::new(),
        })
        .init_state::<AssetLoadingState>()
        .add_loading_state(
            LoadingState::new(AssetLoadingState::Loading)
                .continue_to_state(AssetLoadingState::Loaded)
                .load_collection::<GltfAssets>()
        )
        .add_systems(Startup, spawn_loading_text)
        .add_systems(OnEnter(AssetLoadingState::Loaded), cleanup_loading_text.before(setup))
        .add_systems(OnEnter(AssetLoadingState::Loaded), setup)
        .add_systems(Update, rotate_mesh)
        ;

    #[cfg(feature = "egui")]
    app
        .add_plugins(EguiPlugin)
        .add_systems(Update, ui_system);

    app
        .run();
}

#[derive(Component)]
struct LoadingText;

#[derive(Resource)]
struct Iroha {
    pub mesh_map: BiMap<String, Handle<Mesh>>,
}

impl Iroha {
    pub fn get_mesh(&self, name: &str) -> Option<Handle<Mesh>> {
        self.mesh_map.get_by_left(name).cloned()
    }

    pub fn all_meshes(&self) -> Vec<Handle<Mesh>> {
        self.mesh_map.right_values().map(Clone::clone).collect::<Vec<_>>()
    }

    pub fn get_name(&self, mesh: &Handle<Mesh>) -> Option<String> {
        self.mesh_map.get_by_right(mesh).cloned()
    }

    pub fn add_mesh(&mut self, name: &str, mesh: Handle<Mesh>) {
        self.mesh_map.insert(name.to_string(), mesh);
    }
}

// Add this system to spawn the loading text
fn spawn_loading_text(mut commands: Commands) {
    commands
        .spawn( (
            Text::new("loading..."),
            Node {
                position_type: PositionType::Relative,
                top: Val::Percent(50.0),
                left: Val::Percent(50.0),
                ..default()
            },
            LoadingText,
        ));
}

// Add this system to cleanup the loading text
fn cleanup_loading_text(
    mut commands: Commands,
    loading_text: Query<Entity, With<LoadingText>>,
) {
    for entity in loading_text.iter() {
        commands.entity(entity).despawn_recursive();
    }
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
    // mut asset_server: ResMut<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    gltf_res: Res<GltfAssets>,
    assets_gltf: Res<Assets<Gltf>>,
    assets_gltfmeshes: Res<Assets<GltfMesh>>,
    assets_gltfnodes: Res<Assets<GltfNode>>,
    mut iroha: ResMut<Iroha>,
    // mut meshes: ResMut<Assets<Mesh>>,
) {
    // Create a camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::new(0.0, 1.0, 0.0), Vec3::Y),
    ));

    let gltf = assets_gltf.get(&gltf_res.iroha).unwrap();
    // let scene = GltfAssetLabel::Scene(0).from_asset("models/iroha.glb");
    // let scene = gltf.scenes[0].clone();

    // commands.spawn((
    //     SceneRoot(scene),
    //     // Transform::from_xyz(0.0 , 0.0, 0.0),
    // ));

    // let gltfmesh = gltf.meshes[0];

    let mesh_count = gltf.meshes.len();

    let seed: [u8; 32] = [0; 32];
    let mut rng = StdRng::from_seed(seed);

    for i in 0..mesh_count {
        let node = assets_gltfnodes.get(&gltf.nodes[i]).unwrap();
        let gltfmesh = assets_gltfmeshes.get(&gltf.meshes[i]).unwrap();
        // println!("{:?}", node.name);
        let mesh_handle = gltfmesh.primitives[0].mesh.clone();
        iroha.add_mesh(&node.name, mesh_handle.clone());

        // let mut mesh = meshes.get(&mesh_handle).unwrap().clone();

        // make 10 x 5 matrix for x, y
        let x = ((i % 10) as f32 - 3.0) as f32;
        let y = ((i / 10) as f32 - 1.0) as f32;
        let z = (0.0) as f32;

        // random initial rotation by rad from 0 to 2pi
        let from = 0.0 as f32;
        let to = 2.0 * std::f32::consts::PI;

        let rand_x = rng.random_range(from..to);
        let rand_y = rng.random_range(from..to);
        let rand_z = rng.random_range(from..to);
        let mut rotation = Quat::from_euler(EulerRot::XYZ, rand_x, rand_y, rand_z);

        commands.spawn((
            Mesh3d(mesh_handle.clone()),
            Transform::from_xyz(x, y, z).with_rotation(rotation),
            MeshMaterial3d( materials.add(
                StandardMaterial {
                    base_color: Color::srgb(0.8, 0.7, 0.6),
                    ..default()
                }
            ))
        ));
        
    }
}

fn rotate_mesh(
    time: Res<Time>,
    mut meshes: Query<&mut Transform, With<Mesh3d>>,
) {
    for mut transform in meshes.iter_mut() {
        transform.rotate_x(time.delta_secs());
        transform.rotate_y(time.delta_secs());
    }
}

#[cfg(feature = "egui")]
fn ui_system(mut contexts: EguiContexts) {
    egui::Window::new("Hello").show(contexts.ctx_mut(), |ui| {
        ui.label("world");
    });
}