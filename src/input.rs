use anyhow::Result;
use log::info;
use std::cell::Cell;
use std::process::Command;
use crate::action::{Action, HighlightColor, PageDirection, ScrollDirection, ZoomDirection};

pub struct InputInjector {
    /// Tracks whether the left mouse button is currently held down for a
    /// right-stick drag-select, so we only send `mousedown`/`mouseup` once
    /// per drag rather than on every polling tick.
    dragging: Cell<bool>,
}

impl InputInjector {
    pub fn new() -> Result<Self> {
        // Verify xdotool is available
        Command::new("which")
            .arg("xdotool")
            .output()?;

        Ok(InputInjector {
            dragging: Cell::new(false),
        })
    }

    pub fn execute_action(&self, action: &Action) -> Result<()> {
        match action {
            Action::PressKeys(keys) => {
                self.press_key_sequence(keys)?;
            }
            Action::NavigatePage(direction) => {
                self.navigate_page(direction)?;
            }
            Action::Scroll(direction) => {
                self.scroll(direction)?;
            }
            Action::Zoom(direction) => {
                self.zoom(direction)?;
            }
            Action::ToggleFullscreen => {
                self.toggle_fullscreen()?;
            }
            Action::MoveCursor { dx, dy } => {
                self.move_cursor(*dx, *dy)?;
            }
            Action::SelectText { dx, dy, active } => {
                self.select_text(*dx, *dy, *active)?;
            }
            Action::Highlight(color) => {
                self.highlight(color)?;
            }
        }

        Ok(())
    }

    fn press_key_sequence(&self, keys: &[String]) -> Result<()> {
        let key_str = keys.join("+").to_lowercase();
        info!("Pressing keys: {}", key_str);

        Command::new("xdotool")
            .arg("key")
            .arg(&key_str)
            .output()?;

        Ok(())
    }

    fn navigate_page(&self, direction: &PageDirection) -> Result<()> {
        let key = match direction {
            PageDirection::Next => "Right",
            PageDirection::Previous => "Left",
        };

        info!("Navigate page: {:?}", direction);
        Command::new("xdotool")
            .arg("key")
            .arg(key)
            .output()?;

        Ok(())
    }

    fn scroll(&self, direction: &ScrollDirection) -> Result<()> {
        let (key, clicks) = match direction {
            ScrollDirection::Up => ("Up", 3),
            ScrollDirection::Down => ("Down", 3),
            ScrollDirection::Left => ("Left", 3),
            ScrollDirection::Right => ("Right", 3),
        };

        info!("Scroll: {:?}", direction);

        for _ in 0..clicks {
            Command::new("xdotool")
                .arg("key")
                .arg(key)
                .output()?;
        }

        Ok(())
    }

    fn zoom(&self, direction: &ZoomDirection) -> Result<()> {
        let keys = match direction {
            ZoomDirection::In => vec!["ctrl".to_string(), "plus".to_string()],
            ZoomDirection::Out => vec!["ctrl".to_string(), "minus".to_string()],
            ZoomDirection::Reset => vec!["ctrl".to_string(), "0".to_string()],
        };

        info!("Zoom: {:?}", direction);

        let key_str = keys.join("+");
        Command::new("xdotool")
            .arg("key")
            .arg(&key_str)
            .output()?;

        Ok(())
    }

    fn toggle_fullscreen(&self) -> Result<()> {
        info!("Toggle fullscreen");

        Command::new("xdotool")
            .arg("key")
            .arg("F11")
            .output()?;

        Ok(())
    }

    /// Left stick: move the mouse cursor without pressing any button, so the
    /// user can freely reposition it (e.g. before starting a selection).
    fn move_cursor(&self, dx: f32, dy: f32) -> Result<()> {
        Command::new("xdotool")
            .arg("mousemove_relative")
            .arg("--")
            .arg((dx.round() as i32).to_string())
            .arg((dy.round() as i32).to_string())
            .output()?;

        Ok(())
    }

    /// Right stick: click-and-drag to select text. `active` is true while the
    /// stick is deflected (drag continues) and false once it returns to
    /// center (drag ends / mouse button released).
    fn select_text(&self, dx: f32, dy: f32, active: bool) -> Result<()> {
        if active {
            if !self.dragging.get() {
                info!("Starting text selection drag");
                Command::new("xdotool").arg("mousedown").arg("1").output()?;
                self.dragging.set(true);
            }

            Command::new("xdotool")
                .arg("mousemove_relative")
                .arg("--")
                .arg((dx.round() as i32).to_string())
                .arg((dy.round() as i32).to_string())
                .output()?;
        } else if self.dragging.get() {
            info!("Ending text selection drag");
            Command::new("xdotool").arg("mouseup").arg("1").output()?;
            self.dragging.set(false);
        }

        Ok(())
    }

    /// Y button: apply a highlight annotation to the current text selection.
    /// In Okular, Ctrl+6 activates the yellow Highlighter annotation tool;
    /// with text already selected via the drag above, this applies the
    /// highlight directly to that selection.
    fn highlight(&self, color: &HighlightColor) -> Result<()> {
        match color {
            HighlightColor::Yellow => {
                info!("Highlighting selection: yellow");
                Command::new("xdotool")
                    .arg("key")
                    .arg("ctrl+6")
                    .output()?;
            }
        }

        Ok(())
    }
}
