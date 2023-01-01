mod vim_type;
use vim_type::{
	Action,
	KeyboardAction,
	perform_actions,
};

use std::vec::Vec;
use std::{thread, time::Duration};
//use autopilot::key::KeyCode;

fn countdown(time: u64) {
	println!("Starting in:");
	for i in (0..time).rev() {
		println!("{}", i + 1);
		thread::sleep(Duration::from_millis(1000));
	}
}

fn main() {
	countdown(5);
	let mut actions: Vec<KeyboardAction> = Vec::new();
	
	actions.push(KeyboardAction {
		action: Action::AddLine,
		string: String::from("Hello, World!"),
		..Default::default()
	});
	actions.push(KeyboardAction {
		action: Action::AddLine,
		string: String::from("Testing 123..."),
		..Default::default()
	});
	actions.push(KeyboardAction {
		action: Action::GoToLine,
		line: 1,
		..Default::default()
	});
	actions.push(KeyboardAction {
		action: Action::InsertBefore,
		string: String::from("Before "),
		..Default::default()
	});
	actions.push(KeyboardAction {
		action: Action::AppendAfter,
		string: String::from(" After"),
		..Default::default()
	});

	perform_actions(actions, 20, 180.);
}
