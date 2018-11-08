// Copyright © 2018 Bart Massey

//! Demonstrate implementation of the "Command Pattern"
//! <http://gameprogrammingpatterns.com/command.html>
//! in Rust using "function pointer" trait objects.

use std::collections::HashMap;

/// Default list of keycodes and corresponding actions.
const KEYCODES: &[(&str, Action)] = &[
    ("X", &yell),
    ("Y", &scream),
];

/// Commands will be represented as trait objects with a
/// common interface. As implemented, `'static` is
/// required. It would be straightforward to implement this
/// for arbitrary lifetimes, but it would noise up the code
/// a bit.
type Action = &'static Fn () -> String;

/// A `HashMap` is a great way to represent keybindings:
/// efficient lookup and interior mutability. Newtype this
/// to avoid confusion in larger programs and for
/// readability.
struct KeyBindings(HashMap<String, Action>);

impl KeyBindings {

    /// Make a new empty keybinding.
    fn new() -> Self {
        KeyBindings(HashMap::new())
    }

    /// Make a new keybinding containing each binding in the
    /// list.
    fn new_with_bindings(bindings: &[(&str, Action)]) -> Self {
        let mut kbs = KeyBindings::new();
        for &(key, action) in bindings.iter() {
            kbs.bind_key(key, action);
        }
        kbs
    }
    
    /// Given a key present in the map, get the
    /// corresponding action.
    fn get_action(&self, key: &str) -> Option<Action> {
        self.0.get(key).and_then(|&action| Some(action))
    }

    /// Given a key present in the map, run the
    /// corresponding action and return the result.
    fn run_action(&self, key: &str) -> Option<String> {
        self.get_action(key).and_then(|action| Some(action()))
    }
    
    /// Overwrite or create a keybinding.
    /// `self.get_action()` is useful for rebinding keys.
    fn bind_key(&mut self, key: &str, action: Action) {
        self.0.insert(key.to_string(), action);
    }
}

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
    kbs.bind_key("X", &scream);
    println!("{}", kbs.run_action("X").unwrap());
}
