use std::{fmt::Display, path::PathBuf};

use cosmic::{Element, app::Task};
use tracing::error;

use crate::{
	trans,
	utils::cfg::{
		flags::Flags,
		get_or_create_cfg_file,
		recent::{self, Recent},
		script::ScriptCfg,
	},
};

use super::{Screen, message::Message};

pub mod editor;
pub mod home;

pub enum State {
	Editor(editor::Editor),
	Home(home::Home),
}

impl State {
	pub fn new() -> Self {
		Self::Home(home::Home::new())
	}

	/// Checks if the current [State] should be overwritten by a new one
	pub fn can_overwrite(&self, message: &Message) -> bool {
		if matches!(message, Message::NewTab) {
			false
		} else {
			match self {
				Self::Editor(e) => e.can_close(),
				Self::Home(_) => true,
			}
		}
	}

	/// Creates a [State] from a [Message]
	///
	/// Both [Message::OpenHome] and [Message::NewTab] do the same thing.
	/// To determine what to do with the result, use [State::can_overwrite].
	pub fn from_message(flags: &Flags, message: &Message) -> Option<Self> {
		match message {
			Message::OpenEditor(path) => Some(Self::editor(flags, path)),
			Message::OpenHome | Message::NewTab => Some(Self::home()),

			_ => None,
		}
	}

	/// Creates a [State::Home]
	fn home() -> Self {
		Self::Home(home::Home::new())
	}

	/// Creates a [State::Editor], reading from the provided file if it exists
	fn editor(flags: &Flags, path: &Option<PathBuf>) -> Self {
		if let Some(path) = path {
			let mut recent = Recent::read(get_or_create_cfg_file::<_, Recent>(recent::DIR));
			recent.add(flags, path.clone());
			recent.write();
		}

		Self::Editor(editor::Editor::new(path.clone()))
	}
}

impl Screen for State {
	fn view<'cfg>(&'cfg self, cfg: &'cfg ScriptCfg) -> Element<'cfg, Message> {
		let screen: &dyn Screen = match self {
			Self::Editor(editor) => editor,
			Self::Home(home) => home,
		};
		screen.view(cfg)
	}

	fn update<'cfg>(&'cfg mut self, cfg: &'cfg mut ScriptCfg, message: Message) -> Task<Message> {
		let screen: &mut dyn Screen = match self {
			Self::Editor(editor) => editor,
			Self::Home(home) => home,
		};
		screen.update(cfg, message)
	}
}

impl Display for State {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			State::Editor(editor) => f.write_str(&editor.name()),
			State::Home(_) => f.write_str(&trans!("home")),
		}
	}
}

pub fn format_path(path: &PathBuf) -> String {
	let file_name = match path.file_name() {
		Some(file_name) => file_name,
		None => {
			error!("File {:?} has no name", path);
			return "Unnamed".into();
		}
	};

	let name = match file_name.to_str() {
		Some(name) => name,
		None => {
			error!("File {:?} has an invalid name", path);
			return "Invalid Name".into();
		}
	};

	name.into()
}
