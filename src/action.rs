use anyhow::Result;
use log::{debug, warn};
use serde::{Deserialize, Serialize};
use crate::config::Config;
use crate::controller::ControllerEvent;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Action {
    PressKeys(Vec<String>),
    NavigatePage(PageDirection),
    Scroll(ScrollDirection),
    Zoom(ZoomDirection),
    ToggleFullscreen,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PageDirection {
    Next,
    Previous,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ScrollDirection {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ZoomDirection {
    In,
    Out,
    Reset,
}

pub fn process_event(event: &ControllerEvent, config: &Config) -> Result<Option<Action>> {
    match event {
        ControllerEvent::ButtonPressed(btn) => {
            debug!("Processing button press: {}", btn);

            if let Some(mapping) = config.button_mappings.get(btn) {
                let action = match mapping.action.as_str() {
                    "next_page" => Some(Action::NavigatePage(PageDirection::Next)),
                    "previous_page" => Some(Action::NavigatePage(PageDirection::Previous)),
                    "scroll_up" => Some(Action::Scroll(ScrollDirection::Up)),
                    "scroll_down" => Some(Action::Scroll(ScrollDirection::Down)),
                    "zoom_in" => Some(Action::Zoom(ZoomDirection::In)),
                    "zoom_out" => Some(Action::Zoom(ZoomDirection::Out)),
                    "zoom_reset" => Some(Action::Zoom(ZoomDirection::Reset)),
                    "toggle_fullscreen" => Some(Action::ToggleFullscreen),
                    _ => {
                        // Generic key press
                        Some(Action::PressKeys(mapping.keys.clone()))
                    }
                };

                return Ok(action);
            } else {
                warn!("No mapping found for button: {}", btn);
            }
        }
        ControllerEvent::ButtonReleased(_) => {
            debug!("Button released");
            // Could handle button release events here if needed
        }
        ControllerEvent::StickMoved { stick, x, y } => {
            debug!("Stick moved: {} X={:.2} Y={:.2}", stick, x, y);

            if let Some(mapping) = config.stick_mappings.get(stick) {
                // Could process stick movements here
                debug!("Stick mapping found: {:?}", mapping);
            }
        }
        ControllerEvent::TriggerMoved { trigger, value } => {
            debug!("Trigger moved: {} = {:.2}", trigger, value);

            if let Some(mapping) = config.stick_mappings.get(trigger) {
                // Could process trigger presses here
                debug!("Trigger mapping found: {:?}", mapping);
            }
        }
    }

    Ok(None)
}
