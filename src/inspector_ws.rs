use bevy::prelude::*;
use bevy_egui::EguiRenderToTextureHandle;
use bevy_inspector_egui::bevy_inspector::hierarchy::SelectedEntities;



#[derive(Component, Reflect, Default)]
pub struct InspectorWSMenu;


pub fn update_inspector_ws(
    world: &mut World,
    mut selected_entities: Local<SelectedEntities>,
) {
    let Ok(egui_context) = world
        .query_filtered::<&mut bevy_egui::EguiContext, With<InspectorWSMenu>>()
        .get_single(world)
        else {
            return;
        };

    let mut egui_context_2 = egui_context.clone();
    
    bevy_egui::egui::Window::new("other ui")
    .scroll(true)
    .movable(false)
    .show(egui_context_2.get_mut(), |ui| {
        ui.heading("Hierarchy");

        bevy_inspector_egui::bevy_inspector::hierarchy::hierarchy_ui(
            world,
            ui,
            &mut selected_entities,
        );

         ui.heading("Inspector");

        match selected_entities.as_slice() {
            &[entity] => {
                bevy_inspector_egui::bevy_inspector::ui_for_entity(world, entity, ui);
            }
            entities => {
                bevy_inspector_egui::bevy_inspector::ui_for_entities_shared_components(
                    world, entities, ui,
                );
            }
        }        
        
    });
    

}