use anyhow::Result;
use log::info;
use std::process::Command;
use crate::action::{Action, PageDirection, ScrollDirection, ZoomDirection};

pub struct InputInjector;

impl InputInjector {
    pub fn new() -> Result<Self> {
        // Verify xdotool is available
        Command::new("which")
            .arg("xdotool")
            .output()?;

        Ok(InputInjector)
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
}
