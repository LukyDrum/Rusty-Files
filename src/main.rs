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
use ratatui::{prelude::*, widgets::*};

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    let manager = Manager::new();
    let ui_manager: UIManager = UIManager::new(manager);

    let mut should_quit = false;
    while !should_quit {
        terminal.draw(|frame| ui_manager.ui(frame))?;
        
        let ui_event = handle_events()?;
    }

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