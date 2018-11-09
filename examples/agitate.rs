// Copyright © 2018 Bart Massey
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.


//! A silly little demo of the keybindings crate exercising
//! basic features.

extern crate keybindings;

use keybindings::*;

/// Default list of keycodes and corresponding actions.
const KEYCODES: &[(&str, Action<String>)] = &[
    ("X", &yell),
    ("Y", &scream),
];

/// A sample action.
fn yell() -> String {
    "yell".to_string()
}

/// Another sample action.
fn scream() -> String {
    "scream".to_string()
}

/// Mess around with the keybindings.
fn main() {
    let mut kbs = KeyBindings::new_with_bindings(KEYCODES);
    println!("{}", kbs.run_action("X").unwrap());
    let y_action = kbs.get_action("Y").unwrap();
    kbs.bind_key("X", y_action);
    println!("{}", kbs.run_action(&"X".to_string()).unwrap());
}
