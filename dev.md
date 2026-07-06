# Rust Xbox Controller → Okular Control

## Overview
A custom Rust daemon that reads Xbox controller input and translates it into keyboard shortcuts or system input events to control :contentReference[oaicite:0]{index=0}.

# compilation

build.sh recieve target  cpu and compile to this cpu 

# usage structure

controlled program and maping of gamepad to messages are saved in XDG/config/ decker used  and messegas
controlled programm is online i.e can contor other programs then okular and the mapping  and messages changes


---

## Architecture

Xbox Controller
      ↓
Rust input daemon (gilrs)
      ↓
Action mapping layer
      ↓
Input injection (uinput / xdotool / keypress simulation)
      ↓
Okular actions (page nav, zoom, scroll)

---

## Rust Stack

- Input handling: `gilrs` (Xbox / gamepad support)
- Input injection:
  - Linux: `uinput` or `evdev`
  - fallback: `xdotool`
- Optional IPC: `zbus` (DBus for KDE integration)

---

## Core Logic Flow

1. Poll Xbox controller events
2. Map buttons/axes → document actions
3. Convert actions → keyboard shortcuts
4. Inject events into system
5. Okular reacts via shortcut bindings

---

## Example Button Mapping

| Xbox Button | Action in Okular |
|-------------|------------------|
| D-Pad Up    | Scroll up        |
| D-Pad Down  | Scroll down      |
| RB          | Next page        |
| LB          | Previous page    |
| A           | Zoom in          |
| B           | Zoom out         |
| Start       | Toggle fullscreen|

---

## Rust Skeleton

```rust
use gilrs::{Gilrs, Event};

fn main() {
    let mut gilrs = Gilrs::new().unwrap();

    loop {
        while let Some(Event { event, .. }) = gilrs.next_event() {
            match event {
                gilrs::EventType::ButtonPressed(btn, _) => {
                    match btn {
                        gilrs::Button::South => press_key("Right"), // next page
                        gilrs::Button::West  => press_key("Left"),  // prev page
                        gilrs::Button::North => press_key("Ctrl+Plus"), // zoom in
                        gilrs::Button::East  => press_key("Ctrl+Minus"), // zoom out
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }
}
