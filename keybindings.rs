// Copyright © 2018 Bart Massey
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.


//! Implementation of the "Command Pattern"
//! <http://gameprogrammingpatterns.com/command.html> in
//! Rust using "function pointer" trait objects.

use std::collections::HashMap;
use std::hash::Hash;
use std::borrow::{Borrow, ToOwned};

/// Commands will be represented as trait objects with a
/// common interface. As implemented, `'static` is
/// required. It would be straightforward to implement this
/// for arbitrary lifetimes, but it would noise up the code
/// a bit.
pub type Action<'a, R> = &'a (Fn () -> R + 'a);

/// A `HashMap` is a great way to represent keybindings:
/// efficient lookup and interior mutability. Newtype this
/// to avoid confusion in larger programs and for
/// readability.
pub struct KeyBindings<'a, E, R>(HashMap<E, Action<'a, R>>)
    where E: Clone + Hash + Eq, R: 'a;

impl <'a, E, R> KeyBindings<'a, E, R>
    where E: Clone + Hash + Eq, R: 'a
{
    /// Make a new empty keybinding.
    pub fn new() -> Self {
        let h: HashMap<E, Action<R>> = HashMap::new();
        KeyBindings(h)
    }

    /// Make a new keybinding containing each binding in the
    /// list.
    pub fn new_with_bindings<T>(bindings: &[(T, Action<'a, R>)]) -> Self
        where T: Borrow<E>
    {
        let mut kbs = KeyBindings::new();
        for &(ref key, action) in bindings {
            kbs.bind_key(key.borrow(), action);
        }
        kbs
    }
    
    /// Given a key present in the map, get the
    /// corresponding action.
    pub fn get_action<T>(&self, key: T) -> Option<Action<'a, R>>
        where T: AsRef<E>
    {
        self.0.get(key.as_ref()).and_then(|&action| Some(action))
    }

    /// Given a key present in the map, run the
    /// corresponding action and return the result.
    pub fn run_action<T>(&self, key: T) -> Option<R>
        where T: AsRef<E>
    {
        self.get_action(key).and_then(|action| Some(action()))
    }
    
    /// Overwrite or create a keybinding.
    /// `self.get_action()` is useful for rebinding keys.
    pub fn bind_key<T>(&mut self, key: T, action: Action<'a, R>)
        where T: Borrow<E>
    {
        let key: E = key.borrow().clone();
        self.0.insert(key, action);
    }
}
