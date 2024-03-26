use gilrs::{Button, Event, Gilrs};

fn main() {
    let mut gilrs = Gilrs::new().unwrap();

    // Iterate over all connected gamepads
    for (_id, gamepad) in gilrs.gamepads() {
        println!("Connected gamepad: {:?}", gamepad.name());
    }

    loop {
        // Examine new events
        while let Some(Event { id, event, time }) = gilrs.next_event() {
            println!("{:?} New event from {}: {:?}", time, id, event);
            match event {
                gilrs::EventType::ButtonPressed(Button::South, _) => {
                    println!("Button South pressed");
                }
                gilrs::EventType::ButtonPressed(Button::East, _) => {
                    println!("Button East pressed");
                }
                gilrs::EventType::ButtonPressed(Button::North, _) => {
                    println!("Button North pressed");
                }
                gilrs::EventType::ButtonPressed(Button::West, _) => {
                    println!("Button West pressed");
                }
                gilrs::EventType::ButtonPressed(Button::LeftTrigger, _) => {
                    println!("Left Trigger pressed");
                }
                gilrs::EventType::ButtonPressed(Button::LeftTrigger2, _) => {
                    println!("Left Trigger 2 pressed");
                }
                gilrs::EventType::ButtonPressed(Button::RightTrigger, _) => {
                    println!("Right Trigger pressed");
                }
                gilrs::EventType::ButtonPressed(Button::RightTrigger2, _) => {
                    println!("Right Trigger 2 pressed");
                }
                gilrs::EventType::ButtonPressed(Button::Select, _) => {
                    println!("Select button pressed");
                }
                gilrs::EventType::ButtonPressed(Button::Start, _) => {
                    println!("Start button pressed");
                }
                gilrs::EventType::ButtonPressed(Button::Mode, _) => {
                    println!("Mode button pressed");
                }
                gilrs::EventType::ButtonPressed(Button::LeftThumb, _) => {
                    println!("Left Thumb pressed");
                }
                gilrs::EventType::ButtonPressed(Button::RightThumb, _) => {
                    println!("Right Thumb pressed");
                }
                gilrs::EventType::ButtonPressed(Button::DPadUp, _) => {
                    println!("D-Pad Up pressed");
                }
                gilrs::EventType::ButtonPressed(Button::DPadDown, _) => {
                    println!("D-Pad Down pressed");
                }
                gilrs::EventType::ButtonPressed(Button::DPadLeft, _) => {
                    println!("D-Pad Left pressed");
                }
                gilrs::EventType::ButtonPressed(Button::DPadRight, _) => {
                    println!("D-Pad Right pressed");
                }
                _ => {}
            }
        }
    }
}
