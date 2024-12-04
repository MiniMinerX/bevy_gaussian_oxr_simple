use bevy::input::keyboard::KeyboardInput;
use bevy::input::ButtonState;
use bevy::prelude::*;

use bevy::window::PrimaryWindow;
use bevy_egui::EguiContext;
use bevy_spatial_egui::SpawnSpatialEguiWindowCommand;
use get_egui_keys::{first_row, fn_row, number_row, second_row, third_row};
use bevy_egui::egui::{Key, Ui, WidgetText};
use bevy_egui::systems::bevy_to_egui_physical_key;

#[derive(Clone, Copy, Default)]
pub struct ModifierState {
	caps_lock: bool,
}

#[derive(Clone, Debug)]
pub enum KeyValue {
	Key(Key, bevy::input::keyboard::Key),
	CharKey(String, Key),
}

impl KeyValue {
	pub fn symbol_or_name(&self, modifier_state: &ModifierState) -> String {
		match self {

			KeyValue::Key(key, logic_key) => key.symbol_or_name().to_string(),
			KeyValue::CharKey(char, _) => match modifier_state.caps_lock {
				true => char.to_uppercase(),
				false => char.to_lowercase(),
			},
		}
	}
}

pub fn draw_keyboard(
	ui: &mut Ui,
	primary_window: Entity,
	previously_pressed: &mut Local<Option<KeyValue>>,
	keyboard_writer: &mut EventWriter<KeyboardInput>,
	char_writer: &mut EventWriter<ReceivedCharacter>,
	modifier_state: &mut ModifierState,
) {
	let curr_modifier_state = *modifier_state;
	let mut key_pressed = None;
	ui.horizontal(|ui| {
		show_row(ui, fn_row(), &mut key_pressed, &curr_modifier_state);
	});
	ui.horizontal(|ui| {
		show_row(ui, number_row(), &mut key_pressed, &curr_modifier_state)
	});
	ui.horizontal(|ui| {
		show_row(ui, first_row(), &mut key_pressed, &curr_modifier_state);
	});
	ui.horizontal(|ui| {
		if ui
			.checkbox(&mut modifier_state.caps_lock, "Caps Lock")
			.clicked()
		{
			let button_state = match modifier_state.caps_lock {
				true => ButtonState::Pressed,
				false => ButtonState::Released,
			};
			keyboard_writer.send(KeyboardInput {
				key_code: KeyCode::CapsLock,
				logical_key: bevy::input::keyboard::Key::CapsLock,
				state: button_state,
				window: primary_window,
			});
		}
		show_row(ui, second_row(), &mut key_pressed, &curr_modifier_state);
	});
	ui.horizontal(|ui| {
		show_row(ui, third_row(), &mut key_pressed, &curr_modifier_state);
	});

	if let Some(key) = key_pressed {
		match key.clone() {

			KeyValue::Key(key_og, logic_key) => {
				let key = convert_egui_key(key_og);
				keyboard_writer.send(KeyboardInput {
					key_code: key,
					logical_key: logic_key,
					state: ButtonState::Pressed,
					window: primary_window,
				});
			}
			KeyValue::CharKey(mut char, key) => {

				let char = match curr_modifier_state.caps_lock {
					true => char.to_uppercase(),
					false => char.to_lowercase(),
				};

				let key = convert_egui_key(key);
				keyboard_writer.send(KeyboardInput {
					key_code: key,
					logical_key: bevy::input::keyboard::Key::Character(char.into()),
					state: ButtonState::Pressed,
					window: primary_window,
				});
			}
		}
		previously_pressed.replace(key);
	}
}

fn show_row(
	ui: &mut Ui,
	row: Vec<KeyValue>,
	key_code: &mut Option<KeyValue>,
	modifier_state: &ModifierState,
) {
	for key in row {
		if let Some(key) = print_key(ui, key, modifier_state) {
			key_code.replace(key);
		}
	}
}

fn print_key(
	ui: &mut Ui,
	key: KeyValue,
	modifier_state: &ModifierState,
) -> Option<KeyValue> {
	let text: WidgetText = key.symbol_or_name(modifier_state).into();
	match ui.button(text.monospace()).clicked() {
		true => Some(key),
		false => None,
	}
}

fn convert_egui_key(key: bevy_egui::egui::Key) -> KeyCode {
	match key {
		Key::Escape => KeyCode::Escape,
		Key::Tab => KeyCode::Tab,
		Key::Backspace => KeyCode::Backspace,
		Key::Enter => KeyCode::Enter,
		Key::Space => KeyCode::Space,
		Key::Delete => KeyCode::Delete,
		Key::Colon => todo!(),
		Key::Comma => KeyCode::Comma,
		Key::Backslash => KeyCode::Backslash,
		Key::Slash => KeyCode::Slash,
		Key::Pipe => todo!(),
		Key::Questionmark => todo!(),
		Key::OpenBracket => KeyCode::BracketLeft,
		Key::CloseBracket => KeyCode::BracketRight,
		Key::Backtick => KeyCode::Backquote,
		Key::Minus => KeyCode::Minus,
		Key::Period => KeyCode::Period,
		Key::Plus => todo!(),
		Key::Equals => KeyCode::Equal,
		Key::Semicolon => KeyCode::Semicolon,
		Key::Num0 => KeyCode::Digit0,
		Key::Num1 => KeyCode::Digit1,
		Key::Num2 => KeyCode::Digit2,
		Key::Num3 => KeyCode::Digit3,
		Key::Num4 => KeyCode::Digit4,
		Key::Num5 => KeyCode::Digit5,
		Key::Num6 => KeyCode::Digit6,
		Key::Num7 => KeyCode::Digit7,
		Key::Num8 => KeyCode::Digit8,
		Key::Num9 => KeyCode::Digit9,
		Key::A => KeyCode::KeyA,
		Key::B => KeyCode::KeyB,
		Key::C => KeyCode::KeyC,
		Key::D => KeyCode::KeyD,
		Key::E => KeyCode::KeyE,
		Key::F => KeyCode::KeyF,
		Key::G => KeyCode::KeyG,
		Key::H => KeyCode::KeyH,
		Key::I => KeyCode::KeyI,
		Key::J => KeyCode::KeyJ,
		Key::K => KeyCode::KeyK,
		Key::L => KeyCode::KeyL,
		Key::M => KeyCode::KeyM,
		Key::N => KeyCode::KeyN,
		Key::O => KeyCode::KeyO,
		Key::P => KeyCode::KeyP,
		Key::Q => KeyCode::KeyQ,
		Key::R => KeyCode::KeyR,
		Key::S => KeyCode::KeyS,
		Key::T => KeyCode::KeyT,
		Key::U => KeyCode::KeyU,
		Key::V => KeyCode::KeyV,
		Key::W => KeyCode::KeyW,
		Key::X => KeyCode::KeyX,
		Key::Y => KeyCode::KeyY,
		Key::Z => KeyCode::KeyZ,
		Key::F1 => KeyCode::F1,
		Key::F2 => KeyCode::F2,
		Key::F3 => KeyCode::F3,
		Key::F4 => KeyCode::F4,
		Key::F5 => KeyCode::F5,
		Key::F6 => KeyCode::F6,
		Key::F7 => KeyCode::F7,
		Key::F8 => KeyCode::F8,
		Key::F9 => KeyCode::F9,
		Key::F10 => KeyCode::F10,
		Key::F11 => KeyCode::F11,
		Key::F12 => KeyCode::F12,
		_ => panic!("Unhandled key"),
	}
}

mod get_egui_keys {
	// use bevy_egui::egui::Key;
	use bevy_egui::egui::Key::*;

	use super::KeyValue;
	use super::KeyValue::Key;
	use super::KeyValue::CharKey;

	use bevy::input::keyboard::Key as BevyLogicKey;

	pub fn fn_row() -> Vec<KeyValue> {
		vec![
			Key(Escape, BevyLogicKey::Escape),
			Key(F1, BevyLogicKey::F1),
			Key(F2, BevyLogicKey::F2),
			Key(F3, BevyLogicKey::F3),
			Key(F4, BevyLogicKey::F4),
			Key(F5, BevyLogicKey::F5),
			Key(F6, BevyLogicKey::F6),
			Key(F7, BevyLogicKey::F7),
			Key(F8, BevyLogicKey::F8),
			Key(F9, BevyLogicKey::F9),
			Key(F10, BevyLogicKey::F10),
			Key(F11, BevyLogicKey::F11),
			Key(F12, BevyLogicKey::F12),

		]
	}

	pub fn number_row() -> Vec<KeyValue> {
		vec![
			//Key(Backtick, BevyLogicKey::Backquote),
			CharKey("0".into(), Num0),
			CharKey("1".into(), Num1),
			CharKey("2".into(), Num2),
			CharKey("3".into(), Num3),
			CharKey("4".into(), Num4),
			CharKey("5".into(), Num5),
			CharKey("6".into(), Num6),
			CharKey("7".into(), Num7),
			CharKey("8".into(), Num8),
			CharKey("9".into(), Num9),
			CharKey("0".into(), Num0),
			CharKey("-".into(), Minus),
			CharKey("=".into(), Equals),
			Key(Backspace, BevyLogicKey::Backspace),
		]

	}

	pub fn first_row() -> Vec<KeyValue> {
		vec![
			Key(Tab, BevyLogicKey::Tab),
			CharKey("q".into(), Q),
			CharKey("w".into(), W),
			CharKey("e".into(), E),
			CharKey("r".into(), R),
			CharKey("t".into(), T),
			CharKey("y".into(), Y),
			CharKey("u".into(), U),
			CharKey("i".into(), I),
			CharKey("o".into(), O),
			CharKey("p".into(), P),
			CharKey("\\".into(), Backslash),
		]

	}

	pub fn second_row() -> Vec<KeyValue> {
		vec![
			CharKey("a".into(), A),
			CharKey("s".into(), S),
			CharKey("d".into(), D),
			CharKey("f".into(), F),
			CharKey("g".into(), G),
			CharKey("h".into(), H),
			CharKey("j".into(), J),
			CharKey("k".into(), K),
			CharKey("l".into(), L),
			CharKey(";".into(), Semicolon),
			Key(Enter, BevyLogicKey::Enter),
		]

	}

	pub fn third_row() -> Vec<KeyValue> {
		vec![
			CharKey("z".into(), Z),
			CharKey("x".into(), X),
			CharKey("c".into(), C),
			CharKey("v".into(), V),
			CharKey("b".into(), B),
			CharKey("n".into(), N),
			CharKey("m".into(), M),
			CharKey(",".into(), Comma),
			CharKey(".".into(), Period),
			CharKey("/".into(), Slash),
		]

	}
}















#[derive(Component)]
struct KeyboardWS;

pub fn update_keyboard_ws(
	mut ctxs: Query<&mut EguiContext, With<KeyboardWS>>,
	window: Query<Entity, With<PrimaryWindow>>,
	mut event_writer: EventWriter<KeyboardInput>,
	mut char_writer: EventWriter<ReceivedCharacter>,
	mut previously_pressed: Local<Option<KeyValue>>,
	mut modifier_state: Local<ModifierState>,
) {
	for mut ctx in ctxs.iter_mut() {
		bevy_egui::egui::Window::new("Main Menu")
        .resizable(false)
        //.default_size([4000.0, 4000.0])
        .movable(false)
        .show(ctx.get_mut(), |ui| {
			draw_keyboard(
				ui,
				window.get_single().unwrap(),
				&mut previously_pressed,
				&mut event_writer,
				&mut char_writer,
				&mut modifier_state,
			);
		});

	}
}

pub struct KeybaordWSPlugin;

impl Plugin for KeybaordWSPlugin {
    fn build(&self, app: &mut App) {

        app.add_systems(Startup, setup_keyboard_ws)
        .add_systems(Update, update_keyboard_ws)

        ;
    }
}



fn setup_keyboard_ws(
    mut commands: Commands,
) {
    let keyboard_window = commands.spawn((
        //Name::new("Inspector".to_string()),
        KeyboardWS,
        
    )).id();
    commands.push(SpawnSpatialEguiWindowCommand {
        target_entity: Some(keyboard_window),
        position: Vec3::new(0.0, 1.0, -0.5),
        rotation: Quat::from_axis_angle(Vec3::new(0.,1.,0.), 3.1415),
        resolution: UVec2::splat(512),
        height: 1.0,
        unlit: true,
    });
}