use autopilot::key;
use std::vec::Vec;

// Types the given char for you.
// Using the autopilot crate
pub fn type_chr(chr: char, flags: &[key::Flag], duration: u64) {
	key::tap(&key::Character(chr), flags, duration, 0);
}

// Types a given KeyCode for you.
// Using the autopilot crate
pub fn type_key(key: key::KeyCode, flags: &[key::Flag], duration: u64) {
	key::tap(&key::Code(key), flags, duration, 0);
}

// Types a given string for you.
// Using the autopilot crate
pub fn type_str(string: &str, flags: &[key::Flag], wpm: f64) {
	key::type_string(string, flags, wpm, 0.);
}

pub fn go_to_line(line: u64, wpm: f64, duration: u64) {
	type_str(format!(":{}", line).as_str(), &[], wpm);
	type_key(key::KeyCode::Return, &[], duration);
}

pub fn remove_lines(lines: u64, wpm: f64) {
	for _ in 0..lines {
		type_str("dd", &[], wpm);
	}
}

pub fn enter_insert_mode(duration: u64, before_chr: bool) {
	let chr = if before_chr {'i'} else {'a'};
	type_chr(chr, &[], duration);
}

pub fn exit_insert_mode(duration: u64) {
	type_key(key::KeyCode::Escape, &[], duration);
}

pub fn new_line(duration: u64) {
	type_chr('o', &[], duration);
	type_key(key::KeyCode::Escape, &[], duration);
}

pub fn perform_actions(actions: Vec<KeyboardAction>, duration: u64, wpm: f64) {
	for action in actions.iter() {
		match action.action {
			Action::TypeChr => type_chr(action.chr, &[], duration),
			Action::TypeKey => type_key(action.key, &[], duration),
			Action::TypeStr => type_str(action.string.as_str(), &[], wpm),
			Action::GoToLine => go_to_line(action.line, wpm, duration),
			Action::RemoveLine => remove_lines(action.lines, wpm),
			Action::AddLine => {
				type_key(key::KeyCode::Home, &[], duration);
				enter_insert_mode(duration, true);
				type_str(action.string.as_str(), &[], wpm);
				exit_insert_mode(duration);
				new_line(duration);
			},
			Action::InsertBefore => {
				type_key(key::KeyCode::Home, &[], duration);
				enter_insert_mode(duration, true);
				type_str(action.string.as_str(), &[], wpm);
				exit_insert_mode(duration);
				type_chr('j', &[], duration);
			},
			Action::AppendAfter => {
				type_key(key::KeyCode::End, &[], duration);
				enter_insert_mode(duration, false);
				type_str(action.string.as_str(), &[], wpm);
				exit_insert_mode(duration);
				type_chr('j', &[], duration);
			}
			Action::None => print!("Empty Action"),
		}
	}
}

#[allow(unused)]
pub enum Action {
	TypeChr,
	TypeKey,
	TypeStr,
	GoToLine,
	RemoveLine,
	AddLine,
	InsertBefore,
	AppendAfter,
	None,
}

pub struct KeyboardAction {
	pub action: Action,     // The action to do

	pub lines: u64,			// for Action::RemoveLine
	pub line: u64,			// for Action::GoToLine
	pub key: key::KeyCode,	// for Action::TypeKey
	pub string: String,		// for Action::TypeStr and Action::AddLine
	pub chr: char,			// for Action::TypeChr
}
impl Default for KeyboardAction {
	fn default() -> KeyboardAction {
		KeyboardAction {
			action: Action::None,
			lines: 0,
			line: 0,
			key: key::KeyCode::Escape,
			string: String::from(" "),
			chr: ' '
		}
	}
}
