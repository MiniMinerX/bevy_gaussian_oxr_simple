// grabbing.rs

use bevy::prelude::*;
use bevy_suis::{
    window_pointers::MouseInputMethodData, xr::{Hand, HandInputMethodData},
    xr_controllers::XrControllerInputMethodData, CaptureContext, Field, InputHandler,
    InputHandlerCaptures, PointerInputMethod,
};

#[derive(Clone, Copy, Component)]
pub struct Grabble;

#[derive(Clone, Copy, Component)]
pub struct Grabbed(pub Transform);

pub const GRAB_SEPARATION: f32 = 0.005;

/// System to move grabbable objects when grabbed
pub fn move_grabble(
    mut grabbles: Query<
        (
            Entity,
            &InputHandlerCaptures,
            &GlobalTransform,
            &mut Transform,
            Option<&mut Grabbed>,
            Option<&Parent>,
        ),
        With<Grabble>,
    >,
    method_query: Query<(
        &GlobalTransform,
        Option<&HandInputMethodData>,
        Option<&XrControllerInputMethodData>,
        Option<&MouseInputMethodData>,
    )>,
    parent_query: Query<&GlobalTransform>,
    mut cmds: Commands,
) {
    for (handler_entity, handler, handler_gt, mut handler_transform, mut grabbed, parent) in
        &mut grabbles
    {
        let Some((method_transform, hand_data, controller_data, mouse_data)) = handler
            .captured_methods
            .first()
            .copied()
            .and_then(|v| method_query.get(v).ok())
        else {
            cmds.entity(handler_entity).remove::<Grabbed>();
            continue;
        };
        let mut grabbing = false;
        if let Some(hand) = hand_data {
            let hand = hand.get_in_relative_space(handler_gt);
            grabbing |= finger_separation(&hand, GRAB_SEPARATION);
        }
        if let Some(controller) = controller_data {
            grabbing |= controller.squeezed;
        }
        if let Some(mouse) = mouse_data.as_ref() {
            grabbing |= mouse.left_button.pressed;
        }
        match (grabbed.is_some(), grabbing) {
            (false, true) => {
                cmds.entity(handler_entity)
                    .insert(Grabbed(Transform::from_matrix(
                        method_transform.compute_matrix().inverse() * handler_gt.compute_matrix(),
                    )));
            }
            (true, false) => {
                cmds.entity(handler_entity).remove::<Grabbed>();
            }
            _ => {}
        }
        if let Some(mut t) = grabbed {
            let w = parent
                .and_then(|v| parent_query.get(v.get()).ok())
                .copied()
                .unwrap_or(GlobalTransform::IDENTITY);
            if let Some(mouse) = mouse_data {
                t.0.translation.z += mouse.discrete_scroll.y * 0.1;
            }

            *handler_transform = Transform::from_matrix(
                method_transform.mul_transform(t.0).compute_matrix() * w.compute_matrix().inverse(),
            );
        }
    }
}

/// Function to determine if an input method should capture the object
pub fn capture_condition(
    ctx: In<CaptureContext>,
    query: Query<(
        Option<&HandInputMethodData>,
        Has<PointerInputMethod>,
        Option<&MouseInputMethodData>,
        Option<&XrControllerInputMethodData>,
    )>,
    handler_query: Query<&InputHandlerCaptures>,
) -> bool {
    // Only capture one method
    if !handler_query
        .get(ctx.handler)
        .is_ok_and(|v| v.captured_methods.is_empty())
    {
        return false;
    }
    let method_distance = ctx
        .closest_point
        .distance(ctx.input_method_location.translation);

    // Threshold for capturing
    let mut capture = method_distance <= 0.001;
    let Ok((hand_data, is_pointer, mouse_data, controller_data)) = query.get(ctx.input_method)
    else {
        return capture;
    };
    if let Some(hand_data) = hand_data {
        let hand = hand_data.get_in_relative_space(&ctx.handler_location);
        if method_distance < 0.1 {
            capture |= finger_separation(&hand, GRAB_SEPARATION * 1.5);
        }
    }
    if capture {
        let mut grabbing = false;
        if let Some(hand) = hand_data {
            let hand = hand.get_in_relative_space(&ctx.handler_location);
            grabbing |= finger_separation(&hand, GRAB_SEPARATION);
        }
        if let Some(controller) = controller_data {
            grabbing |= controller.squeezed;
        }
        if let Some(mouse) = mouse_data {
            grabbing |= mouse.left_button.pressed;
        }
        return grabbing;
    }
    if is_pointer {
        if let Some(mouse) = mouse_data {
            return mouse.left_button.pressed;
        }
        return true;
    }
    capture
}

/// Function to check finger separation for grabbing with hands
pub fn finger_separation(hand: &Hand, max_separation: f32) -> bool {
    hand.thumb.tip.pos.distance(hand.index.tip.pos)
        < hand.index.tip.radius + hand.thumb.tip.radius + max_separation
}
