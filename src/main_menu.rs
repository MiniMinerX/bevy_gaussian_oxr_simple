use std::sync::Arc;

use bevy::{asset::embedded_asset, prelude::*, render::{camera::RenderTarget, render_resource::{Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages}, view::RenderLayers}, ui::update};
use bevy_egui::{EguiContext, egui};
use bevy_gaussian_splatting::{GaussianCamera, GaussianCloudSettings, GaussianSplattingBundle};
use bevy_mod_xr::camera::XrCamera;
use bevy_spatial_egui::SpawnSpatialEguiWindowCommand;
use bevy_suis::{Field, InputHandler};
use egui_aesthetix::Aesthetix;
//use space_editor::prelude::events_dispatcher::inspect;

use crate::{gaussian::GaussianMarker, grabbing::{self, Grabble}, inspector_ws::InspectorWSMenu};





#[derive(Component)]
pub struct MainMenu {
    splats_showing: bool,
    insepctor_showing: bool,
    slider_menu_showing: bool,
    base_file_path: String,
    splat_file_name: String,
    splat_id: Option<Entity>,
    cached_transform: Option<Transform>,  // Add this field to cache the splat transform
}


pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {

        app.add_systems(Startup, setup_main_menu)
        
        .add_systems(Update, main_menu_ui);

    }
}



fn setup_main_menu(
    mut commands: Commands,
) {


    let main_menu_window = commands.spawn((
        MainMenu {
            splats_showing: false,
            insepctor_showing: false,
            slider_menu_showing: false,
            base_file_path: "embedded://".into(),
            splat_file_name: "cat1.gcloud".into(),
            splat_id: None,
            cached_transform: None,
        },
        
    )).id();
    commands.push(SpawnSpatialEguiWindowCommand {
        target_entity: Some(main_menu_window),
        position: Vec3::new(0.0, 2.0, -0.5),
        rotation: Quat::from_axis_angle(Vec3::new(0.,1.,0.), 3.1415),
        resolution: UVec2::splat(512),
        height: 1.0,
        unlit: true,
    });
}


#[derive(Default)]
struct SplatMenuSettings {
    splat_transform_showing: bool,
    splat_opacity: f32,
    splat_size_scale: f32,
    splat_transform_id: Option<Entity>,
    splat_showing: bool,
    inspector_showing: bool,
    temp_gaus_name: String,
    inspector_window_id: Option<Entity>,

    hand_cam_showing: bool,
    hand_cam_id: Option<Entity>,
}



fn main_menu_ui(
    mut ctxs: Query<(&mut bevy_egui::EguiContext, &mut MainMenu)>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query_transforms: Query<&Transform, With<SplatTransformTool>>,
    mut query_gaussian_settings: Query<&mut GaussianCloudSettings>,
    xr_cams: Query<Entity, (With<XrCamera>, Without<GaussianCamera>)>,
    xr_cams2: Query<Entity, (With<XrCamera>, With<GaussianCamera>)>,
    
    mut images: ResMut<Assets<Image>>,

    mut sms: Local<SplatMenuSettings>,

) {
    for (mut ctx, mut menu) in ctxs.iter_mut() {
        
        let ctx: &mut EguiContext = &mut ctx;

        ctx.get_mut().set_style(
			Arc::new(egui_aesthetix::themes::NordLight).custom_style(),
		);
        
        bevy_egui::egui::Window::new("Main Menu")
        .resizable(false)
        //.default_size([4000.0, 4000.0])
        .movable(false)
        .scroll(true)
        .show(ctx.get_mut(), |ui| {

            //ui.allocate_space([512.0,512.0].into());

            ui.label("Main Menu");
            

            ui.label("Splat Name");
            ui.add(egui::TextEdit::singleline(&mut sms.temp_gaus_name));

            if ui.button("splat fix for vr?").clicked() {
                add_guassian_to_xr(
                    &mut commands,
                    &xr_cams
                );
            }

            if ui.button("undo vr fix").clicked() {
                remove_gaussian_from_xr(&mut commands, &xr_cams2);
            }

            if ui.button("Load Splat").clicked() {
                menu.splat_file_name = (sms.temp_gaus_name.clone()).to_string();
            }

            ui.label(format!("File Path: {}{}", &menu.base_file_path, &menu.splat_file_name));

            ui.toggle_value(&mut sms.splat_showing, "toggle splats");
            if sms.splat_showing && menu.splat_id == None  {

                let concat_string = format!("{}{}", &menu.base_file_path, &menu.splat_file_name);


                let gaussian_entity = commands.spawn((
                    GaussianSplattingBundle {
                        cloud: asset_server.load(concat_string),
                        settings: GaussianCloudSettings {
                            aabb: false,
                            global_opacity: 1.0,
                            global_scale: 1.0,
                            transform: Transform::from_xyz(0.0, 0.0, 0.0).with_rotation(Quat::from_rotation_z(std::f32::consts::PI)),
                            opacity_adaptive_radius: false,
                            ..Default::default()
                        },
                        visibility: Visibility::Visible,
                        ..Default::default()
                    },
                    GaussianMarker {
                        prev_visibility: Visibility::Visible,
                    },
                    Name::new("Gaussian"),
                    //RenderLayers::layer(2),
                    //RenderLayers::layer(1),
                )).id();

                menu.splat_id = Some(gaussian_entity);
            }
            
            if !sms.splat_showing && menu.splat_id.is_some() {
                match menu.splat_id {
                    Some(splat_id) => {
                        commands.entity(splat_id).despawn_recursive();
                        menu.splat_id = None;
                    },
                    None => {},
                }
                
            }


            let slider = ui.add(egui::Slider::new(&mut sms.splat_opacity, 0.0..=1.0).text("Opacity"));
            
            if slider.changed() {
                if let Some(splat_entity) = menu.splat_id {
                    if let Ok(mut settings) = query_gaussian_settings.get_mut(splat_entity) {
                        settings.global_opacity = sms.splat_opacity;
                    }
                }
            }


            let slider = ui.add(egui::Slider::new(&mut sms.splat_size_scale, 0.0..=1.0).text("Splat Size"));

            if slider.changed() {
                if let Some(splat_entity) = menu.splat_id {
                    if let Ok(mut settings) = query_gaussian_settings.get_mut(splat_entity) {
                        let mut prev_transform = settings.transform.clone();
                        let scale = Vec3::splat(sms.splat_size_scale);
                        settings.transform = prev_transform.with_scale(scale);
                    }
                }
            }
            
            ui.toggle_value(&mut sms.inspector_showing, "Toggle Inspector");
            if sms.inspector_showing && sms.inspector_window_id == None {
                let inspector_window = commands.spawn((
                    Name::new("Inspector".to_string()),
                    InspectorWSMenu,
                    
                )).id();

                commands.push(SpawnSpatialEguiWindowCommand {
                    target_entity: Some(inspector_window),
                    position: Vec3::new(0.0, 1.0, -0.5),
                    rotation: Quat::IDENTITY,
                    resolution: UVec2::splat(512),
                    height: 1.0,
                    unlit: true,
                });

                sms.inspector_window_id = Some(inspector_window);
            }

            if !sms.inspector_showing && sms.inspector_window_id.is_some() {
                match sms.inspector_window_id {
                    Some(inspector_id) => {
                        commands.entity(inspector_id).despawn_recursive();
                        sms.inspector_window_id = None;
                    },
                    None => {},
                }
            }


            // Hand Camera Spawn
            ui.toggle_value(&mut sms.hand_cam_showing, "Hand Cam");

            if sms.hand_cam_showing && sms.hand_cam_id == None {
                sms.hand_cam_id = Some(spawn_hand_cam(
                    &mut commands,
                    &asset_server,
                    &mut images,
                    &mut materials,
                    &mut meshes,
                ))
            }

            if !sms.hand_cam_showing && sms.hand_cam_id.is_some() {
                match sms.hand_cam_id {
                    Some(hcam_id) => {
                        commands.entity(hcam_id).despawn_recursive();
                        sms.hand_cam_id = None;
                    },
                    None => {},
                }
            }
            

            ui.toggle_value(&mut sms.splat_transform_showing, "Toggle Splat Transform Tool");

            if sms.splat_transform_showing && sms.splat_transform_id == None {
                if let Some(splat_entity) = menu.splat_id {
                    if let Ok(gaussian_settings) = query_gaussian_settings.get(splat_entity) {
                        // Cache the splat's transform
                        menu.cached_transform = Some(gaussian_settings.transform.clone());
                    }
                }
                

                let sphere_mesh = meshes.add(Sphere::new(0.3));

                let mut initial_transform = Transform::from_xyz(0.5, 0.5, 0.0);
                if let Some(cached_transform) = menu.cached_transform.clone() {
                    initial_transform = cached_transform; // Use cached transform if available
                }


                let splat_t_id = commands.spawn((
                    PbrBundle {
                        mesh: sphere_mesh.clone(),
                        material: materials.add(StandardMaterial {
                            base_color: Srgba::hex("#ffd891").unwrap().into(),
                            unlit: true,
                            ..default()
                        }),
                        transform: initial_transform,
                        ..default()
                    },
                    Name::new("Splat Transform Tool"),

                    SplatTransformTool,
                    //bevy_transform_gizmo::GizmoTransformable,
                    InputHandler::new(grabbing::capture_condition),
                    Field::Sphere(0.3),
                    Grabble,
                )).id();

                sms.splat_transform_id = Some(splat_t_id);

            }

            if sms.splat_transform_showing && sms.splat_transform_id.is_some(){
                
                if let Some(trans_entity) = sms.splat_transform_id {

                    if let Ok(trans_tool) = query_transforms.get(trans_entity) {
                        // Cache the splat's transform
                        menu.cached_transform = Some(*trans_tool);


                        if sms.splat_showing && menu.splat_id.is_some() {
                            if let Some(splat_id) = menu.splat_id {
                                if let Ok(mut splat_settings) = query_gaussian_settings.get_mut(splat_id) {
                                    splat_settings.transform.translation = trans_tool.translation;
                                    splat_settings.transform.rotation = trans_tool.rotation;
                                }
                            }
                        }
                    }
                }
            }

            if !sms.splat_transform_showing && sms.splat_transform_id.is_some() {
                if let Some(splat_t_id) = sms.splat_transform_id {
                    // Despawn the Transform Tool along with its children (Virtual Points)
                    commands.entity(splat_t_id).despawn_recursive();
                    // Reset the transform tool ID
                    sms.splat_transform_id = None;
                }

            }
            
        });
    }
}


#[derive(Component)]
struct SplatTransformTool;


/* 
#[derive(Component, Deref, DerefMut)]
pub struct RestoreGausCamTimer(Timer);

pub fn restore_visibility_system(
    mut commands: Commands,
    time: Res<Time>,
    mut restore_timer: Query<(Entity, &mut RestoreGausCamTimer)>,
    mut gaussian_query: Query<(&mut Visibility, &mut GaussianCamera)>,
) {
    for (entity, mut timer) in restore_timer.iter_mut() {
        timer.0.tick(time.delta());
        if timer.0.finished() {
            for (mut visibility, mut gaus) in gaussian_query.iter_mut() {
                *visibility = gaus.prev_visibility;
            }
            commands.entity(entity).despawn_recursive();
        }
    }
}
*/



pub fn spawn_hand_cam(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    images: &mut ResMut<Assets<Image>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    meshes: &mut ResMut<Assets<Mesh>>,
) -> Entity {

    let hand_cam_box = commands.spawn((
        Transform::from_xyz(0.0, 0.0, 0.0),
        Name::new("Hand Cam Box".to_string()),
        Visibility::Visible,
        InputHandler::new(grabbing::capture_condition),
        Field::Sphere(0.1),
        Grabble,
        GlobalTransform::default(),
        InheritedVisibility::default(),
    )).id();

    let camera_glb = asset_server.load(GltfAssetLabel::Scene(0).from_asset("embedded://camera.glb"));

    let hand_cam_model = commands.spawn((
        SceneBundle {
            scene: camera_glb.clone(),
            transform: Transform::from_xyz(0.035, 0.185, -0.492 ),
            ..Default::default()
        },
        Name::new("Hand Cam Model".to_string()), 
    )).id();

    let size = Extent3d {
        width: 512,
        height: 512,
        ..default()
    };

    // This is the texture that will be rendered to.
    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        ..default()
    };

    // fill image.data with zeroes
    image.resize(size);
    let image_handle = images.add(image);

    let hand_cam_real = commands.spawn((
        Camera3dBundle {
            camera: Camera {
                target: RenderTarget::Image(image_handle.clone()),
                clear_color: Color::BLACK.into(),
                ..default()
            },
            // Configure your camera bundle as needed
            transform: Transform::from_translation((0.300, 0.276, -0.025).into())
                .with_rotation(Quat::from_rotation_y(-std::f32::consts::PI / 2.0)),
            ..Default::default()
        },
        Name::new("Hand Cam Real".to_string()),   
        GaussianCamera { warmup: true },
    )).id();

    let material_handle = materials.add(StandardMaterial{
        base_color_texture: Some(image_handle.clone()),
        reflectance: 0.02,
        unlit: false,
        ..default()
    });

    let screen_display = commands.spawn((
        PbrBundle {
            mesh: meshes.add(Plane3d::new(Vec3::Y, (0.5, 0.5).into()).mesh().size(1.0, 1.0)),
            material: material_handle.clone(),
            transform: Transform::from_xyz(0.0, 0.0, 1.0)
                .looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
    )).id();

    let screen_model = commands.spawn((

    )).id();

    let screen_box = commands.spawn((
        Transform::from_xyz(0.0, 0.0, 0.0),
        Name::new("Screen Box".to_string()),
        Visibility::Visible,
        Field::Cuboid(Cuboid::from_size(Vec3::new(1.,1.,0.2))),
        Grabble,
        InputHandler::new(grabbing::capture_condition),
        GlobalTransform::default(),
        InheritedVisibility::default(),
    )).id();

    let hand_cam_tool_box = commands.spawn((
        Transform::from_xyz(0.0, 0.0, 0.0),
        Name::new("Hand Cam Tool".to_string()),
        Visibility::Visible,
        GlobalTransform::default(),
        InheritedVisibility::default(),
    )).id();

    commands.entity(screen_box).push_children(&[
        screen_display,
        screen_model,
    ]);

    commands.entity(hand_cam_box).push_children(&[
        hand_cam_model,
        hand_cam_real,
    ]);

    commands.entity(hand_cam_tool_box).push_children(&[
        hand_cam_box,
        screen_box,
    ]);

    return hand_cam_tool_box
}

pub fn add_guassian_to_xr(
    commands: &mut Commands,
    xr_cams_entity: &Query<Entity, (With<XrCamera>, Without<GaussianCamera>)>,
) {
    for xr_cam in xr_cams_entity.iter() {
        commands.entity(xr_cam).insert((
            GaussianCamera { warmup: true},
            Name::new("Xr Cam"),
            //Tonemapping::None, makes thing look wonko
        ));
    }
}
pub fn remove_gaussian_from_xr(
    commands: &mut Commands,
    xr_cams_entity: &Query<Entity, (With<XrCamera>, With<GaussianCamera>)>,
) {
    for xr_cam in xr_cams_entity.iter() {
        commands.entity(xr_cam).remove::<GaussianCamera>();
    }
}