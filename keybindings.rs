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
    where E: Hash + Eq, R: 'a;

impl <'a, E, R> KeyBindings<'a, E, R>
    where E: Hash + Eq, R: 'a
{
    /// Make a new empty keybinding.
    pub fn new() -> Self {
        KeyBindings(HashMap::new())
    }

    /// Make a new keybinding containing each binding in the
    /// list.
    ///
    /// # Examples:
    ///
    /// ```
    /// use keybindings::{Action, KeyBindings};
    /// fn build_action<'a>(n: usize) -> Box<Fn() -> usize> {
    ///     Box::new(move || { n })
    /// }
    /// let aok = build_action(1);
    /// let bok = build_action(2);
    /// let bindings: &[_] = &[("a", &*aok), ("b", &*bok)];
    /// let mut kc = KeyBindings::new_with_bindings(bindings);
    /// assert_eq!(kc.run_action("a").unwrap(), 1);
    /// ```
    pub fn new_with_bindings<'b, T>(bindings: &'a [(&'b T, Action<'a, R>)])
                                    -> Self
        where E: Borrow<T>, T: ToOwned<Owned=E> + ?Sized
    {
        let mut kbs = KeyBindings::new();
        for (key, action) in bindings.iter() {
            kbs.bind_key(key.to_owned(), action);
        }
        kbs
    }

    /// Given a key present in the map, run the
    /// corresponding action and return the result.
    ///
    /// # Examples:
    ///
    /// ```
    /// use keybindings::{Action, KeyBindings};
    /// let aok: Action<String> = &|| {
    ///     "aok".to_string()
    /// };
    /// let bindings = &[("a", aok)];
    /// let mut kc = KeyBindings::new_with_bindings(bindings);
    /// assert_eq!(kc.run_action("a").unwrap(), "aok");
    /// ```
    pub fn run_action<T>(&self, key: &T) -> Option<R>
        where E: Borrow<T>, T: Hash + Eq + ?Sized
    {
        self.get_action(key).and_then(|action| Some(action()))
    }
    
    // XXX See
    // <https://github.com/rust-lang/rust/issues/31228> for
    // why the types are a little funny.

    /// Overwrite or create a keybinding.
    /// `self.get_action()` is useful for rebinding keys.
    ///
    /// # Examples:
    ///
    /// ```
    /// use keybindings::KeyBindings;
    /// let one = || {1};
    /// let mut kc = KeyBindings::new();
    /// kc.bind_key(&'a', &one);
    /// assert_eq!(kc.run_action(&'a').unwrap(), 1);
    /// ```
    pub fn bind_key<T>(&mut self, key: &T, action: Action<'a, R>)
        where E: Borrow<T>, T: ToOwned<Owned=E> + ?Sized
    {
        self.0.insert(key.borrow().to_owned(), action);
    }
    
    /// Given a key present in the map, get the
    /// corresponding action.
    /// ```
    /// use keybindings::{Action, KeyBindings};
    /// let aok: Action<String> = &|| {
    ///     "aok".to_string()
    /// };
    /// let bindings = &[("a", aok)];
    /// let mut kc = KeyBindings::new_with_bindings(bindings);
    /// let ax = kc.get_action("a").unwrap();
    /// kc.bind_key("b", ax);
    /// assert_eq!(kc.run_action("b").unwrap(), "aok");
    /// ```
    pub fn get_action<T>(&self, key: &T) -> Option<Action<'a, R>>
        where E: Borrow<T>, T: Hash + Eq + ?Sized
    {
        self.0.get(key.borrow()).and_then(|&action| Some(action))
    }
}
