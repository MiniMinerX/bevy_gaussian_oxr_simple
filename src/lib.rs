//! A simple 3D scene with light shining over a cube sitting on a plane

use bevy::asset::{embedded_asset, AssetMetaCheck};
use bevy::audio::{PlaybackMode, Volume};
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::prelude::*;
use bevy::render::camera::{ManualTextureViewHandle, RenderTarget};
use bevy::render::render_resource::{Extent3d, TextureUsages};
use bevy::window::PresentMode;
use bevy_egui::EguiPlugin;
use bevy_embedded_assets::EmbeddedAssetPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_inspector_egui::DefaultInspectorConfigPlugin;
use bevy_mod_openxr::session::OxrSession;
use bevy_mod_openxr::types::EnvironmentBlendMode;
use bevy_mod_openxr::{add_xr_plugins, init::OxrInitPlugin, types::OxrExtensions};
use bevy_mod_picking::backends::raycast::RaycastPickable;
use bevy_mod_xr::camera::XrCamera;
use bevy_mod_xr::hands::HandBoneRadius;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use bevy_spatial_egui::SpawnSpatialEguiWindowCommand;
use bevy_suis::debug::SuisDebugGizmosPlugin;
use bevy_suis::window_pointers::SuisWindowPointerPlugin;
use bevy_suis::xr::SuisXrPlugin;
use bevy_suis::xr_controllers::SuisXrControllerPlugin;
use bevy_suis::SuisCorePlugin;
//use bevy_transform_gizmo::TransformGizmoPlugin;
//use bevy_vr_controller::animation::defaults::default_character_animations;
//use bevy_vr_controller::VrControllerPlugin;

use keyboard::KeybaordWSPlugin;
use main_menu::MainMenuPlugin;


pub mod gaussian;

pub mod inspector_ws;
pub mod main_menu;
pub mod grabbing;
pub mod keyboard;

use bevy_gaussian_splatting::{GaussianCamera, GaussianCloudSettings, GaussianSplattingBundle, GaussianSplattingPlugin};
use crate::inspector_ws::{InspectorWSMenu, update_inspector_ws};
//use bevy_vr_controller::player::PlayerSettings;

#[bevy_main]
pub fn main() {
    let mut app = App::new();
    
    #[cfg(feature = "native")]
    app.add_plugins(add_xr_plugins(DefaultPlugins).set(OxrInitPlugin {
        app_info: default(),
        exts: {
            let mut exts = OxrExtensions::default();
            exts.enable_fb_passthrough();
            //exts.enable_hand_tracking();
            //exts.enable_custom_refresh_rates();

            exts
        },
        /* 
        blend_modes: Some({
            let mut v = Vec::new();
                v.push(EnvironmentBlendMode::ALPHA_BLEND);
                v.push(EnvironmentBlendMode::ADDITIVE);
                v.push(EnvironmentBlendMode::OPAQUE);
                v
            }),
            */
        blend_modes: default(),
        backends: default(),
        formats: default(),
        resolutions: default(),
        synchronous_pipeline_compilation: default(),
    }).set(WindowPlugin {
        primary_window: Some(Window {
            //present_mode: bevy::window::PresentMode::AutoVsync,
            ..default()
        }),

        // #[cfg(target_os = "android")]
        // exit_condition: bevy::window::ExitCondition::DontExit,

        #[cfg(target_os = "android")]
        close_when_requested: true,
        ..default()
    }).set(AssetPlugin {
        meta_check: AssetMetaCheck::Never,
        ..default()
    }));
    

    #[cfg(feature = "pcvr")]
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            //present_mode: bevy::window::PresentMode::AutoVsync,
            ..default()
        }),
        // #[cfg(target_os = "android")]
        // exit_condition: bevy::window::ExitCondition::DontExit,
        #[cfg(target_os = "android")]
        close_when_requested: true,
        ..default()
    }).set(AssetPlugin {
        meta_check: AssetMetaCheck::Never,
        ..default()
    }));

    // System for requesting refresh rate ( should refactor and upstream into bevy_openxr )
    //.add_systems(Update, set_requested_refresh_rate)
    
    
    // Our plugins
    app.add_plugins(GaussianSplattingPlugin)
    
    
    //.add_systems(Startup, gaussian_setup)

    //.add_plugins((CubeCreationPlugin, CustomPhysicsIntegrations))
    
    
    // Third party plugins
    .add_plugins((
        EmbeddedAssetPlugin::default(),
        //PhysicsDebugPlugin::default(),
        //bevy_xr_utils::hand_gizmos::HandGizmosPlugin,
    ))


    // Setup
    //.add_systems(Startup, gaussian_setup)
    .add_systems(Startup, setup)
    // Realtime lighting is expensive, use ambient light instead
    .insert_resource(AmbientLight {
        color: Default::default(),
        brightness: 1000.0,
    })
    .insert_resource(Msaa::Off)
    .insert_resource(ClearColor(Color::NONE))

     
    //.add_plugins(bevy_mod_picking::DefaultPickingPlugins)


    //.add_systems(Update, restore_visibility_system)
    
    .add_plugins(PanOrbitCameraPlugin)

    .add_plugins(EguiPlugin)
    .add_plugins(bevy_spatial_egui::SpatialEguiPlugin)

    .add_plugins(WorldInspectorPlugin::new())

    .add_plugins(MainMenuPlugin)
    .add_plugins(KeybaordWSPlugin)

    .add_systems(Update, update_inspector_ws) 


    .add_plugins((
        SuisCorePlugin,
        SuisWindowPointerPlugin,
        SuisDebugGizmosPlugin,

        #[cfg(feature = "native")]
        SuisXrPlugin,
        #[cfg(feature = "native")]
        SuisXrControllerPlugin,
        #[cfg(feature = "native")]
        (bevy_suis_lasers::draw_lasers::LaserPlugin, bevy_suis_lasers::laser_input_methods::LaserInputMethodPlugin)

    ));
    

    // Player Controller
    #[cfg(feature = "native")]
    app.add_plugins(
        bevy_xr_utils::xr_utils_actions::XRUtilsActionsPlugin,
    )
    .add_plugins((
    //    VrControllerPlugin,
    ))
    .add_systems(Update, grabbing::move_grabble);

    //.add_systems(Startup, setup_player)
    app.run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
    
) {


    #[cfg(feature = "pcvr")] 
    commands.spawn((
        Camera3dBundle {
            camera: Camera{
                ..default()
            },
            transform: Transform::from_xyz(0.,0.,0.),
            ..default()
        },
        PanOrbitCamera::default(),
        GaussianCamera { warmup: true},

        //bevy_transform_gizmo::GizmoPickSource::default(),
    ));
    
    
}


#[derive(Component, Clone, Copy)]
pub struct HandBoneColider(Entity);

fn set_requested_refresh_rate(mut local: Local<bool>, mut session: Option<ResMut<OxrSession>>) {
    if session.is_none() {
        return;
    }
    if *local {
        return;
    }
    *local = true;
    session
        .as_mut()
        .unwrap()
        .request_display_refresh_rate(72.0)
        .unwrap();
}


/* 
fn setup_player(asset_server: Res<AssetServer>, mut commands: Commands) {
    PlayerSettings {
        animations: Some(default_character_animations(&asset_server)),
        vrm: Some(asset_server.load("embedded://Test1.vrm")),
        void_level: Some(-20.0),
        spawn: Vec3::new(0.0, 3.0, 0.0),
        ..default()
    }
    .spawn(&mut commands);
}
*/