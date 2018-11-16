// Copyright © 2018 Bart Massey
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.


//! Implementation of the "Command Pattern"
//! <http://gameprogrammingpatterns.com/command.html>.

use std::collections::HashMap;
use std::hash::Hash;
use std::borrow::{Borrow, ToOwned};

/// Type of actions with the given result type.
pub type Action<'a, R> = &'a (Fn () -> R + 'a);

// A `HashMap` is a great way to represent bindings:
// efficient lookup and interior mutability. Newtype this
// to avoid confusion in larger programs and for
// readability.

/// A `Bindings` object manages bindings between events
/// and actions. It has the capability to execute the
/// selected action given an event.
pub struct Bindings<'a, E, R>(HashMap<E, Action<'a, R>>)
    where E: Hash + Eq, R: 'a;

impl <'a, E, R> Bindings<'a, E, R>
    where E: Hash + Eq, R: 'a
{
    /// Make a new empty binding.
    pub fn new() -> Self {
        Bindings(HashMap::new())
    }

    /// Make a new keybinding containing each binding in the
    /// list.
    ///
    /// # Examples:
    ///
    /// ```
    /// use kbehdz::{Action, Bindings};
    /// fn build_action<'a>(n: usize) -> Box<Fn() -> usize> {
    ///     Box::new(move || { n })
    /// }
    /// let aok = build_action(1);
    /// let bok = build_action(2);
    /// let bindings: &[_] = &[("a", &*aok), ("b", &*bok)];
    /// let mut kc = Bindings::from_list(bindings);
    /// assert_eq!(kc.run_action("a").unwrap(), 1);
    /// ```
    pub fn from_list<T, U>(bindings: U) -> Self
        where U: AsRef<[(&'a T, Action<'a, R>)]> + 'a,
              E: Borrow<T>, T: ToOwned<Owned=E> + ?Sized + 'a
    {
        let mut kbs = Bindings::new();
        let b = bindings.as_ref();
        for (ref key, action) in b.iter() {
            kbs.bind_action(key.to_owned(), action);
        }
        kbs
    }

    /// Given an event that is in the bindings, run the
    /// corresponding action and return the result.  Return
    /// `None` if no such event is bound.
    ///
    /// # Examples:
    ///
    /// ```
    /// use kbehdz::{Action, Bindings};
    /// let aok: Action<String> = &|| {
    ///     "aok".to_string()
    /// };
    /// let bindings = &[("a", aok)];
    /// let mut kc = Bindings::from_list(bindings);
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
    /// use kbehdz::Bindings;
    /// let one = || {1};
    /// let mut kc = Bindings::new();
    /// kc.bind_action(&'a', &one);
    /// assert_eq!(kc.run_action(&'a').unwrap(), 1);
    /// ```
    pub fn bind_action<T>(&mut self, key: &T, action: Action<'a, R>)
        where E: Borrow<T>, T: ToOwned<Owned=E> + ?Sized
    {
        self.0.insert(key.borrow().to_owned(), action);
    }
    
    /// Given an event that is in the bindings, return the
    /// corresponding action unexecuted.  Return
    /// `None` if no such event is bound.
    ///
    /// # Examples:
    ///
    /// ```
    /// use kbehdz::{Action, Bindings};
    /// let aok: Action<String> = &|| {
    ///     "aok".to_string()
    /// };
    /// let bindings = &[("a", aok)];
    /// let mut kc = Bindings::from_list(bindings);
    /// let ax = kc.get_action("a").unwrap();
    /// kc.bind_action("b", ax);
    /// assert_eq!(kc.run_action("b").unwrap(), "aok");
    /// ```
    pub fn get_action<T>(&self, key: &T) -> Option<Action<'a, R>>
        where E: Borrow<T>, T: Hash + Eq + ?Sized
    {
        self.0.get(key.borrow()).and_then(|&action| Some(action))
    }
}
