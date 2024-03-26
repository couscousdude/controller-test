use device_query::{DeviceQuery, DeviceState, Keycode};
use std::collections::HashMap;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;

type ActionFn = Box<dyn Fn() + Send + 'static>;

struct KeybindManager {
    bindings: HashMap<Keycode, ActionFn>,
}

impl KeybindManager {
    fn new() -> Self {
        KeybindManager {
            bindings: HashMap::new(),
        }
    }

    fn bind_key<F>(&mut self, key: Keycode, action: F)
    where
        F: Fn() + Send + 'static,
    {
        self.bindings.insert(key, Box::new(action));
    }

    fn handle_key_event(&self, key: Keycode) {
        if let Some(action) = self.bindings.get(&key) {
            action();
        }
    }
}

fn main() {
    let (tx, rx): (Sender<Keycode>, Receiver<Keycode>) = channel();

    let v_x = Arc::new(Mutex::new(0));
    let v_y = Arc::new(Mutex::new(0));

    // Spawn a separate thread to listen for key events
    thread::spawn(move || {
        let device_state = DeviceState::new();
        loop {
            let keys: Vec<Keycode> = device_state.get_keys();
            for key in keys {
                match key {
                    Keycode::W | Keycode::Up => tx.send(Keycode::W).unwrap(),
                    Keycode::A | Keycode::Left => tx.send(Keycode::A).unwrap(),
                    Keycode::S | Keycode::Down => tx.send(Keycode::S).unwrap(),
                    Keycode::D | Keycode::Right => tx.send(Keycode::D).unwrap(),
                    Keycode::B | Keycode::Space => tx.send(Keycode::B).unwrap(),
                    _ => (),
                }
            }
            thread::sleep(std::time::Duration::from_millis(100));
        }
    });

    let mut keybind_manager = KeybindManager::new();

    // Bind actions to specific keys
    keybind_manager.bind_key(Keycode::W, {
        let v_x = v_x.clone();
        let v_y = v_y.clone();
        move || {
            *v_y.lock().unwrap() += 1;
            println!(
                "v_x: {}, v_y: {}",
                *v_x.lock().unwrap(),
                *v_y.lock().unwrap()
            );
        }
    });
    keybind_manager.bind_key(Keycode::A, {
        let v_x = v_x.clone();
        let v_y = v_y.clone();
        move || {
            *v_x.lock().unwrap() -= 1;
            println!(
                "v_x: {}, v_y: {}",
                *v_x.lock().unwrap(),
                *v_y.lock().unwrap()
            );
        }
    });
    keybind_manager.bind_key(Keycode::D, {
        let v_x = v_x.clone();
        let v_y = v_y.clone();
        move || {
            *v_x.lock().unwrap() += 1;
            println!(
                "v_x: {}, v_y: {}",
                *v_x.lock().unwrap(),
                *v_y.lock().unwrap()
            );
        }
    });
    keybind_manager.bind_key(Keycode::S, {
        let v_x = v_x.clone();
        let v_y = v_y.clone();
        move || {
            *v_y.lock().unwrap() -= 1;
            println!(
                "v_x: {}, v_y: {}",
                *v_x.lock().unwrap(),
                *v_y.lock().unwrap()
            );
        }
    });
    keybind_manager.bind_key(Keycode::B, {
        let v_x = v_x.clone();
        let v_y = v_y.clone();
        move || {
            *v_y.lock().unwrap() = 0;
            *v_x.lock().unwrap() = 0;
            println!(
                "v_x: {}, v_y: {}",
                *v_x.lock().unwrap(),
                *v_y.lock().unwrap()
            );
        }
    });

    // Main event loop
    loop {
        if let Ok(key) = rx.recv() {
            keybind_manager.handle_key_event(key);
        }
    }
}
