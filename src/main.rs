mod config;
mod controller;
mod input;
mod action;

use anyhow::Result;
use log::{info, error};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    info!("Starting decker - Xbox Controller → Okular Control");

    // Load configuration
    let config = config::load_config()?;
    info!("Configuration loaded from {}", config::config_path()?.display());

    // Initialize controller
    let mut controller = controller::ControllerManager::new(config.clone())?;
    info!("Xbox controller initialized");

    // Initialize input injector
    let input_injector = input::InputInjector::new()?;
    info!("Input injector ready");

    // Main event loop
    loop {
        match controller.poll_event() {
            Ok(Some(event)) => {
                match action::process_event(&event, &config) {
                    Ok(Some(action)) => {
                        info!("Action triggered: {:?}", action);
                        if let Err(e) = input_injector.execute_action(&action) {
                            error!("Failed to execute action: {}", e);
                        }
                    }
                    Ok(None) => {
                        // Event mapped but no action
                    }
                    Err(e) => {
                        error!("Error processing event: {}", e);
                    }
                }
            }
            Ok(None) => {
                // No event, sleep briefly to avoid busy-waiting
                tokio::time::sleep(Duration::from_millis(10)).await;
            }
            Err(e) => {
                error!("Controller error: {}", e);
                // Attempt to reinitialize
                match controller::ControllerManager::new(config.clone()) {
                    Ok(new_controller) => {
                        controller = new_controller;
                        info!("Controller reinitialized");
                    }
                    Err(e) => {
                        error!("Failed to reinitialize controller: {}", e);
                        tokio::time::sleep(Duration::from_secs(2)).await;
                    }
                }
            }
        }
    }
}
