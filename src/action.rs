use anyhow::Result;
use log::{debug, warn};
use serde::{Deserialize, Serialize};
use crate::config::Config;
use crate::controller::{ControllerEvent, StickState};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Action {
    PressKeys(Vec<String>),
    NavigatePage(PageDirection),
    Scroll(ScrollDirection),
    Zoom(ZoomDirection),
    ToggleFullscreen,
    /// Left stick: move the mouse cursor (relative), so the user can position
    /// the text cursor / caret without changing any active selection.
    MoveCursor { dx: f32, dy: f32 },
    /// Right stick: click-and-drag the mouse to start/continue selecting text.
    /// `active` is false once the stick returns to the deadzone, which ends
    /// the drag (mouse button release).
    SelectText { dx: f32, dy: f32, active: bool },
    /// Y button: highlight the current text selection in yellow.
    Highlight(HighlightColor),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HighlightColor {
    Yellow,
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
                    "highlight_yellow" => Some(Action::Highlight(HighlightColor::Yellow)),
                    _ => {
                        // Generic key press
                        Some(Action::PressKeys(mapping.keys.clone()))
                    }
                };

                return Ok(action);
            } else if btn == "Y" {
                // Y has a sensible built-in default even without an explicit
                // config entry: highlight the current selection in yellow.
                return Ok(Some(Action::Highlight(HighlightColor::Yellow)));
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

/// Sensitivity multiplier: analog stick deflection (-1.0..=1.0) to screen pixels
/// moved per polling tick.
const CURSOR_SPEED: f32 = 12.0;

/// Translates the continuous analog stick state into cursor / selection
/// actions. Unlike `process_event`, this runs every tick (not just on
/// axis-changed events) so movement stays smooth while a stick is held.
///
/// `right_drag_active` tracks whether the right-stick drag-select is
/// currently "held" (mouse button down) across calls, so we know when to
/// emit the final `SelectText { active: false }` that releases the button.
pub fn process_stick_state(
    state: &StickState,
    config: &Config,
    right_drag_active: &mut bool,
) -> Vec<Action> {
    let mut actions = Vec::new();
    let deadzone = config.deadzone;

    // Left stick: move the mouse cursor around, e.g. to position it before
    // starting a selection or to place the text caret.
    if state.left_x.abs() > deadzone || state.left_y.abs() > deadzone {
        actions.push(Action::MoveCursor {
            dx: state.left_x * CURSOR_SPEED,
            // Screen Y grows downward; invert so pushing up moves the cursor up.
            dy: -state.left_y * CURSOR_SPEED,
        });
    }

    // Right stick: click-and-drag to select text. Moving the stick begins
    // (or continues) the drag; releasing it back to center ends the drag.
    if state.right_x.abs() > deadzone || state.right_y.abs() > deadzone {
        *right_drag_active = true;
        actions.push(Action::SelectText {
            dx: state.right_x * CURSOR_SPEED,
            dy: -state.right_y * CURSOR_SPEED,
            active: true,
        });
    } else if *right_drag_active {
        *right_drag_active = false;
        actions.push(Action::SelectText {
            dx: 0.0,
            dy: 0.0,
            active: false,
        });
    }

    actions
}
