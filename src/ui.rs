use ratatui::{prelude::*, widgets::*};
use crate::file_manager::*;


/// Describes events that can happen in the UI
pub enum UIEvent {
    Up, // Up key
    Down, // Down key
    In, // Right key = move into directory
    Out, // Left key = move out of directory
    ToggleHidden, // H key = toggle hidden files
    None, // When nothing happens
    Quit, // Q key = quit app
}

/// Takes care of the UI and acts as the interface between User and File Manager
pub struct UIManager {
    pub should_quit: bool,
    file_manager: Manager,
    selected_entries: Vec<Entry>,
    selection_index: usize,
    show_hidden: bool,
}

impl UIManager {
    pub fn new(manager: Manager) -> UIManager {
        UIManager { should_quit: false, file_manager: manager, selected_entries: Vec::new(), selection_index: 0, show_hidden: false }
    }

    pub fn ui(&self, frame: &mut Frame) -> () {
        let main_layout = Layout::new(
            Direction::Vertical,
            [
                Constraint::Length(1),
                Constraint::Min(0),
            ],
        )
        .split(frame.size());
    
        frame.render_widget(
            Block::new().borders(Borders::TOP).title(" Rusty Files "),
            main_layout[0],
        );
    
        let mut state = ListState::default().with_selected(Some(self.selection_index));
    
        let items = self.get_entries_to_show();
        let list = List::new(items)
            .block(Block::default().title(" Current directory ").borders(Borders::ALL))
            .style(Style::default())
            .highlight_style(Style::default().add_modifier(Modifier::BOLD))
            .repeat_highlight_symbol(true)
            .direction(ListDirection::TopToBottom);
    
        frame.render_stateful_widget(list, main_layout[1], &mut state);
    }

    fn toggle_show_hidden(&mut self) -> () {
        self.show_hidden = !self.show_hidden;
    }

    fn get_entries_to_show(&self) -> Vec<String> {
        let filenames = self.file_manager.filenames();

        if !self.show_hidden {
            filenames
        }
        else {
            filenames.into_iter().filter(|f| !f.starts_with('.') ).collect()
        }
    }

    pub fn proccess_ui_event(&mut self, event: UIEvent) -> () {
        match event {
            UIEvent::Quit => self.should_quit = true,
            UIEvent::ToggleHidden => self.toggle_show_hidden(),
            _ => {}
        }
    }
}