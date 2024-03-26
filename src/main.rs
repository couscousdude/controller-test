use gilrs::{Button, Event, EventType, Gilrs};

fn main() {
    let mut gilrs = Gilrs::new().unwrap();

    // Iterate over all connected gamepads
    for (_id, gamepad) in gilrs.gamepads() {
        println!("Connected gamepad: {:?}", gamepad.name());
    }

    loop {
        // Examine new events
        while let Some(event) = gilrs.next_event() {
            match event.event {
                EventType::Connected => {
                    println!("Gamepad connected: {}", event.id);
                }

                EventType::Disconnected => {
                    println!("Gamepad disconnected: {}", event.id);
                }

                EventType::ButtonPressed(button, _code) => {
                    println!("Gamepad: {} Button Pressed: {:?}", event.id, button);
                }

                EventType::ButtonReleased(button, _code) => {
                    println!("Gamepad: {} Button Released: {:?}", event.id, button);
                }

                EventType::AxisChanged(axis, value, _code) => {
                    println!(
                        "Gamepad: {} Axis Changed: {:?} Value: {}",
                        event.id, axis, value
                    );
                }

                _ => {}
            }
        }
    }
}
