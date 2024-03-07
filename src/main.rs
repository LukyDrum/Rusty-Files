mod file_manager;
mod ui;

use crate::ui::{UIManager, UIEvent};

use std::io::{self, stdout};

use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use file_manager::Manager;
use ratatui::prelude::*;

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    // Actual code starts

    let manager = Manager::new();
    let mut ui_manager: UIManager = UIManager::new(manager);

    while !ui_manager.should_quit {
        ui_manager.update();

        terminal.draw(|frame| ui_manager.ui(frame))?;
        
        let ui_event = handle_events()?;
        ui_manager.proccess_ui_event(ui_event);
    }

    // Actual code ends

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

fn handle_events() -> Result<UIEvent, std::io::Error>{
    if event::poll(std::time::Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                return match key.code {
                    KeyCode::Char('q') => Ok(UIEvent::Quit),
                    KeyCode::Up => Ok(UIEvent::Up),
                    KeyCode::Down => Ok(UIEvent::Down),
                    KeyCode::Left => Ok(UIEvent::Out),
                    KeyCode::Right => Ok(UIEvent::In),
                    KeyCode::Char('h') => Ok(UIEvent::ToggleHidden),
                    _ => Ok(UIEvent::None) // For all other cases
                }
            }
        }
    }

    Ok(UIEvent::None)
}