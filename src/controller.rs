use anyhow::{Result, Context};
use gilrs::{Gilrs, Event, EventType, Button, Axis};
use log::{info, warn};
use serde::{Deserialize, Serialize};
use crate::config::Config;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ControllerEvent {
    ButtonPressed(String),
    ButtonReleased(String),
    StickMoved {
        stick: String,
        x: f32,
        y: f32,
    },
    TriggerMoved {
        trigger: String,
        value: f32,
    },
}

pub struct ControllerManager {
    gilrs: Gilrs,
    active_gamepad_id: Option<usize>,
    config: Config,
}

impl ControllerManager {
    pub fn new(config: Config) -> Result<Self> {
        let gilrs = Gilrs::new().context("Failed to initialize Gilrs")?;

        info!("Available gamepads:");
        let mut active_id = None;

        for (id, gamepad) in gilrs.gamepads() {
            info!("  - ID: {}, Name: {}", id, gamepad.name());
            if active_id.is_none() {
                active_id = Some(id);
                info!("Using gamepad: {}", gamepad.name());
            }
        }

        if active_id.is_none() {
            warn!("No gamepad detected. Waiting for connection...");
        }

        Ok(Self {
            gilrs,
            active_gamepad_id: active_id,
            config,
        })
    }

    pub fn poll_event(&mut self) -> Result<Option<ControllerEvent>> {
        while let Some(Event { id, event, .. }) = self.gilrs.next_event() {
            // Update active gamepad
            if self.active_gamepad_id.is_none() {
                self.active_gamepad_id = Some(id);
                info!("Gamepad connected: {}", self.gilrs.gamepad(id).name());
            }

            // Only process events from the active gamepad
            if self.active_gamepad_id != Some(id) {
                continue;
            }

            match event {
                EventType::ButtonPressed(btn, _) => {
                    let button_name = button_to_string(btn);
                    info!("Button pressed: {}", button_name);
                    return Ok(Some(ControllerEvent::ButtonPressed(button_name)));
                }
                EventType::ButtonReleased(btn, _) => {
                    let button_name = button_to_string(btn);
                    info!("Button released: {}", button_name);
                    return Ok(Some(ControllerEvent::ButtonReleased(button_name)));
                }
                EventType::AxisChanged(axis, value, _) => {
                    if should_report_axis(axis, value, self.config.deadzone) {
                        let axis_name = axis_to_string(axis);
                        info!("Axis changed: {} = {:.2}", axis_name, value);

                        if is_stick_axis(axis) {
                            let (stick, coords) = parse_stick_axis(axis, value);
                            return Ok(Some(ControllerEvent::StickMoved {
                                stick,
                                x: coords.0,
                                y: coords.1,
                            }));
                        } else if is_trigger_axis(axis) {
                            let trigger = axis_to_string(axis);
                            return Ok(Some(ControllerEvent::TriggerMoved {
                                trigger,
                                value: (value + 1.0) / 2.0, // Normalize to 0-1
                            }));
                        }
                    }
                }
                EventType::Disconnected => {
                    warn!("Gamepad disconnected");
                    self.active_gamepad_id = None;
                }
                EventType::Connected => {
                    info!("Gamepad connected: {}", self.gilrs.gamepad(id).name());
                    self.active_gamepad_id = Some(id);
                }
                _ => {}
            }
        }

        Ok(None)
    }
}

fn button_to_string(btn: Button) -> String {
    match btn {
        Button::South => "A",
        Button::East => "B",
        Button::West => "X",
        Button::North => "Y",
        Button::LB => "LB",
        Button::RB => "RB",
        Button::LT => "LT",
        Button::RT => "RT",
        Button::Select => "Select",
        Button::Start => "Start",
        Button::LeftThumb => "LeftThumb",
        Button::RightThumb => "RightThumb",
        Button::Mode => "Mode",
        Button::DPadUp => "DPadUp",
        Button::DPadDown => "DPadDown",
        Button::DPadLeft => "DPadLeft",
        Button::DPadRight => "DPadRight",
        _ => "Unknown",
    }
    .to_string()
}

fn axis_to_string(axis: Axis) -> String {
    match axis {
        Axis::LeftStickX => "LeftStickX",
        Axis::LeftStickY => "LeftStickY",
        Axis::LeftZ => "LeftTrigger",
        Axis::RightStickX => "RightStickX",
        Axis::RightStickY => "RightStickY",
        Axis::RightZ => "RightTrigger",
        _ => "Unknown",
    }
    .to_string()
}

fn is_stick_axis(axis: Axis) -> bool {
    matches!(
        axis,
        Axis::LeftStickX | Axis::LeftStickY | Axis::RightStickX | Axis::RightStickY
    )
}

fn is_trigger_axis(axis: Axis) -> bool {
    matches!(axis, Axis::LeftZ | Axis::RightZ)
}

fn parse_stick_axis(axis: Axis, value: f32) -> (String, (f32, f32)) {
    match axis {
        Axis::LeftStickX | Axis::LeftStickY => {
            // This is a simplified version; real implementation would track both axes
            ("LeftStick".to_string(), (value, value))
        }
        Axis::RightStickX | Axis::RightStickY => {
            ("RightStick".to_string(), (value, value))
        }
        _ => ("Unknown".to_string(), (0.0, 0.0)),
    }
}

fn should_report_axis(axis: Axis, value: f32, deadzone: f32) -> bool {
    if is_stick_axis(axis) || is_trigger_axis(axis) {
        value.abs() > deadzone
    } else {
        false
    }
}
