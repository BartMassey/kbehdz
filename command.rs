use std::collections::HashMap;

const KEYCODES: &[(&str, Action)] = &[
    ("X", &yell),
    ("Y", &scream),
];

type Action = &'static Fn () -> String;

struct KeyBindings(HashMap<String, Action>);

impl KeyBindings {

    fn new() -> Self {
        KeyBindings(HashMap::new())
    }
    
    fn new_with_bindings(bindings: &[(&str, Action)]) -> Self {
        let mut kbs = KeyBindings::new();
        for &(key, action) in bindings.iter() {
            kbs.bind_key(key, action);
        }
        kbs
    }
    
    fn run_action(&self, key: &str) -> Option<String> {
        self.0.get(key).and_then(|&action| Some(action()))
    }
    
    fn bind_key(&mut self, key: &str, action: Action) {
        self.0.insert(key.to_string(), action);
    }
}

fn yell() -> String {
    "yell".to_string()
}

fn scream() -> String {
    "scream".to_string()
}

fn main() {
    let mut kbs = KeyBindings::new_with_bindings(KEYCODES);
    println!("{}", kbs.run_action("X").unwrap());
    kbs.bind_key("X", &scream);
    println!("{}", kbs.run_action("X").unwrap());
}
