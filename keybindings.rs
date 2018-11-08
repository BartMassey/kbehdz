// Copyright © 2018 Bart Massey

//! Implementation of the "Command Pattern"
//! <http://gameprogrammingpatterns.com/command.html> in
//! Rust using "function pointer" trait objects.

use std::collections::HashMap;

/// Commands will be represented as trait objects with a
/// common interface. As implemented, `'static` is
/// required. It would be straightforward to implement this
/// for arbitrary lifetimes, but it would noise up the code
/// a bit.
pub type Action = &'static Fn () -> String;

/// A `HashMap` is a great way to represent keybindings:
/// efficient lookup and interior mutability. Newtype this
/// to avoid confusion in larger programs and for
/// readability.
pub struct KeyBindings(HashMap<String, Action>);

impl KeyBindings {

    /// Make a new empty keybinding.
    pub fn new() -> Self {
        KeyBindings(HashMap::new())
    }

    /// Make a new keybinding containing each binding in the
    /// list.
    pub fn new_with_bindings(bindings: &[(&str, Action)]) -> Self {
        let mut kbs = KeyBindings::new();
        for &(key, action) in bindings.iter() {
            kbs.bind_key(key, action);
        }
        kbs
    }
    
    /// Given a key present in the map, get the
    /// corresponding action.
    pub fn get_action(&self, key: &str) -> Option<Action> {
        self.0.get(key).and_then(|&action| Some(action))
    }

    /// Given a key present in the map, run the
    /// corresponding action and return the result.
    pub fn run_action(&self, key: &str) -> Option<String> {
        self.get_action(key).and_then(|action| Some(action()))
    }
    
    /// Overwrite or create a keybinding.
    /// `self.get_action()` is useful for rebinding keys.
    pub fn bind_key(&mut self, key: &str, action: Action) {
        self.0.insert(key.to_string(), action);
    }
}
