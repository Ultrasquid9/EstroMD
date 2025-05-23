use cosmic::{
	iced_core::keyboard,
	widget::menu::{KeyBind as CosmicMenuBind, key_bind::Modifier},
};
use rhai::{Array, CustomType, TypeBuilder};
use smol_str::SmolStr;
use tracing::warn;

use crate::create_rhai_mod;

#[derive(PartialEq, Clone, CustomType)]
pub struct Key(pub SmolStr);

#[derive(Clone, CustomType)]
pub struct Keybind {
	pub key: Key,
	pub modifiers: Vec<Key>,
}

impl Key {
	pub fn new(str: String) -> Self {
		Self(str.into())
	}
}

impl Keybind {
	pub fn new(key: Key, modifiers: Array) -> Self {
		let modifiers = modifiers
			.iter()
			.filter_map(|item| item.clone().try_cast())
			.collect();

		Self { key, modifiers }
	}
}

impl From<Key> for keyboard::Key {
	fn from(key: Key) -> Self {
		// TODO: handle some "Named" variants
		keyboard::Key::Character(key.0)
	}
}

impl From<Key> for Option<Modifier> {
	fn from(key: Key) -> Self {
		Some(match () {
			_ if key == modifiers::Super => Modifier::Super,
			_ if key == modifiers::Ctrl => Modifier::Ctrl,
			_ if key == modifiers::Alt => Modifier::Alt,
			_ if key == modifiers::Shift => Modifier::Shift,
			_ => {
				warn!("{} is not a modifier!", key.0);
				return None;
			}
		})
	}
}

impl From<Keybind> for CosmicMenuBind {
	fn from(bind: Keybind) -> Self {
		Self {
			key: bind.key.into(),
			modifiers: bind
				.modifiers
				.iter()
				.filter_map(|item| item.clone().into())
				.collect(),
		}
	}
}

// Macros are expanded outwards -> inwards.
// Therefore, the entire module must be created within the macro.
macro_rules! keys {
	( $( $name:ident , )* ) => {
		create_rhai_mod! { modifiers { $(
			#[allow(non_upper_case_globals)]
			pub const $name: super::Key = super::Key(smol_str::SmolStr::new_static(stringify!($name)));
		)* } }
	};
}

keys! {
	Super,
	Ctrl,
	Alt,
	Shift,
}
