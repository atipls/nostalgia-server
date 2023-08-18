mod mesher;
mod uv;

use bevy::log::LogPlugin;
use bevy::window::CursorGrabMode;
use std::num::NonZeroU8;
use std::path::PathBuf;
use std::time::Duration;

use bevy::pbr::wireframe::{WireframeConfig, WireframePlugin};
use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    render::{
        render_resource::{AddressMode, FilterMode, SamplerDescriptor},
        texture::ImageSampler,
    },
};
use bevy_fly_camera::{FlyCamera, FlyCameraPlugin};

#[derive(Resource)]
struct TextureAtlasImage(Handle<Image>);

#[derive(Resource)]
struct SuzanneBlenderMonkey(Handle<Mesh>);

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin {
                    default_sampler: SamplerDescriptor {
                        address_mode_u: AddressMode::Repeat,
                        address_mode_v: AddressMode::Repeat,
                        mag_filter: FilterMode::Nearest,
                        min_filter: FilterMode::Nearest,
                        mipmap_filter: FilterMode::Nearest,
                        anisotropy_clamp: NonZeroU8::new(16u8),
                        ..Default::default()
                    },
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "What if minecraft was in rust".into(),
                        ..default()
                    }),
                    ..default()
                })
                .set(LogPlugin {
                    filter: "warn".into(),
                    level: bevy::log::Level::DEBUG,
                }),
        )
        .add_plugin(FlyCameraPlugin)
        .add_plugin(FrameTimeDiagnosticsPlugin)
        .add_plugin(LogDiagnosticsPlugin {
            debug: false,
            wait_duration: Duration::from_secs(10),
            filter: None,
        })
        .add_plugin(WireframePlugin)
        .insert_resource(Msaa::default())
        .add_startup_system(setup_camera)
        .add_startup_system(load_texture_atlas_image.in_base_set(StartupSet::PreStartup))
        .add_startup_system(update_texture_atlas_repeat.in_base_set(StartupSet::PostStartup))
        .add_startup_system(load_world_file)
        .add_system(grab_mouse)
        .add_system(display_axis_arrows)
        .run();
}

fn display_axis_arrows(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let arrow_mesh = meshes.add(
        Mesh::try_from(shape::Icosphere {
            radius: 0.1,
            subdivisions: 1,
        })
        .unwrap(),
    );

    let arrow_material = StandardMaterial {
        base_color: Color::RED,
        ..Default::default()
    };

    let arrow_material = materials.add(arrow_material);

    spawn_pbr(
        &mut commands,
        arrow_material.clone(),
        arrow_mesh.clone(),
        Transform::from_translation(Vec3::new(16.0 * 8.0 + 1.0, 150.0, 16.0 * 8.0)),
    );

    let arrow_material = StandardMaterial {
        base_color: Color::GREEN,
        ..Default::default()
    };

    let arrow_material = materials.add(arrow_material);

    spawn_pbr(
        &mut commands,
        arrow_material.clone(),
        arrow_mesh.clone(),
        Transform::from_translation(Vec3::new(16.0 * 8.0, 150.0, 16.0 * 8.0)),
    );

    let arrow_material = StandardMaterial {
        base_color: Color::BLUE,
        ..Default::default()
    };

    let arrow_material = materials.add(arrow_material);

    spawn_pbr(
        &mut commands,
        arrow_material.clone(),
        arrow_mesh.clone(),
        Transform::from_translation(Vec3::new(16.0 * 8.0, 150.0, 16.0 * 8.0 + 1.0)),
    );
}

fn load_texture_atlas_image(mut commands: Commands, server: Res<AssetServer>) {
    let handle: Handle<Image> = server.load("textures/terrain-atlas.tga");
    commands.insert_resource(TextureAtlasImage(handle));
}

fn update_texture_atlas_repeat(
    mut event_reader: EventReader<AssetEvent<Image>>,
    mut textures: ResMut<Assets<Image>>,
    texture_atlas_image: Res<TextureAtlasImage>,
) {
    for event in event_reader.iter() {
        match event {
            AssetEvent::Created { handle } => {
                if *handle == texture_atlas_image.0 {
                    let mut texture = textures.get_mut(&texture_atlas_image.0).unwrap();
                    texture.sampler_descriptor = ImageSampler::Descriptor(SamplerDescriptor {
                        address_mode_u: AddressMode::Repeat,
                        address_mode_v: AddressMode::Repeat,
                        mag_filter: FilterMode::Nearest,
                        min_filter: FilterMode::Linear,
                        mipmap_filter: FilterMode::Nearest,
                        anisotropy_clamp: NonZeroU8::new(16u8),
                        ..Default::default()
                    });
                }
            }
            AssetEvent::Modified { handle: _ } => {}
            AssetEvent::Removed { handle: _ } => {}
        }
    }
}

fn spawn_pbr(
    commands: &mut Commands,
    material: Handle<StandardMaterial>,
    mesh: Handle<Mesh>,
    transform: Transform,
) {
    commands.spawn(PbrBundle {
        mesh,
        material,
        transform,
        ..Default::default()
    });
}

fn load_world_file(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    texture_atlas: Res<TextureAtlasImage>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut wireframe_config: ResMut<WireframeConfig>,
) {
    wireframe_config.global = false;

    let texture_atlas_material = materials.add(StandardMaterial {
        base_color_texture: Some(texture_atlas.0.clone()),
        alpha_mode: AlphaMode::Mask(0.5),
        reflectance: 0.2,
        ..default()
    });

    commands.spawn(PointLightBundle {
        transform: Transform::from_translation(Vec3::new(4.0 * 16.0, 146.0, 14.0 * 16.0)),
        point_light: PointLight {
            range: 32.0 * 32.0 * 128.0,
            intensity: 800000.0,
            ..Default::default()
        },
        ..Default::default()
    });

    commands.spawn(PointLightBundle {
        transform: Transform::from_translation(Vec3::new(4.0 * 16.0, 146.0, 14.0 * 16.0)),
        point_light: PointLight {
            range: 32.0 * 32.0 * 128.0,
            intensity: 800000.0,
            ..Default::default()
        },
        ..Default::default()
    });

    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.2,
    });

    let world_path = PathBuf::from("assets/MainWorld");
    let world = world::World::from_file(world_path).expect("Failed to load the world");

    info!("Seed: {}", world.seed);

    for (chunk_index, chunk) in world.chunks.iter().enumerate() {
        let chunk_x = chunk_index % 16;
        let chunk_z = chunk_index / 16;

        let corner_chunks: [Option<&world::Chunk>; 4] = [
            world
                .chunks
                .get((chunk_x.checked_sub(1).unwrap_or(0xFFF)) + (chunk_z * 16)),
            world.chunks.get((chunk_x + 1) + (chunk_z * 16)),
            world
                .chunks
                .get(chunk_x + ((chunk_z.checked_sub(1).unwrap_or(0xFFF)) * 16)),
            world.chunks.get(chunk_x + ((chunk_z + 1) * 16)),
        ];

        let chunk_mesh = mesher::build_cube_meshes(chunk, corner_chunks);
        let added_mesh = meshes.add(chunk_mesh);

        spawn_pbr(
            &mut commands,
            texture_atlas_material.clone(),
            added_mesh,
            Transform::from_translation(Vec3::new(
                chunk_x as f32 * 16.0,
                0.0,
                chunk_z as f32 * 16.0,
            )),
        );
    }
}

fn setup_camera(mut commands: Commands) {
    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(128.0, 70.0, 128.0))
                .looking_at(Vec3::new(-512.0, 0.0, 512.0), Vec3::Y),
            ..Default::default()
        })
        .insert(FlyCamera {
            accel: 2.0,
            max_speed: 0.75,
            ..Default::default()
        });
}

fn grab_mouse(
    mut windows: Query<&mut Window>,
    mouse: Res<Input<MouseButton>>,
    key: Res<Input<KeyCode>>,
) {
    let mut window = windows.single_mut();

    if mouse.just_pressed(MouseButton::Left) {
        window.cursor.visible = false;
        window.cursor.grab_mode = CursorGrabMode::Locked;
    }

    if key.just_pressed(KeyCode::Escape) {
        window.cursor.visible = true;
        window.cursor.grab_mode = CursorGrabMode::None;
    }
}
