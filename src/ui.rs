use crate::file_manager::*;
use ratatui::{prelude::*, widgets::*};

/// Describes events that can happen in the UI
pub enum UIEvent {
    Up,           // Up key
    Down,         // Down key
    In,           // Right key = move into directory
    Out,          // Left key = move out of directory
    ToggleHidden, // H key = toggle hidden files
    None,         // When nothing happens
    Quit,         // Q key = quit app
}

/// Takes care of the UI and acts as the interface between User and File Manager
pub struct UIManager {
    pub should_quit: bool,
    file_manager: Manager,
    current_entries: Vec<Entry>,
    selected_entries: Vec<Entry>,
    selection_index: usize,
    show_hidden: bool,
}

impl UIManager {
    pub fn new(manager: Manager) -> UIManager {
        UIManager {
            should_quit: false,
            file_manager: manager,
            current_entries: Vec::new(),
            selected_entries: Vec::new(),
            selection_index: 0,
            show_hidden: false,
        }
    }

    pub fn update(&mut self) -> () {
        self.current_entries = self.file_manager.current_directory_entries.clone();
    }

    pub fn ui(&self, frame: &mut Frame) -> () {
        let main_layout = Layout::new(
            Direction::Vertical,
            [Constraint::Length(1), Constraint::Min(0)],
        )
        .split(frame.size());

        frame.render_widget(
            Block::new().borders(Borders::TOP).title(" Rusty Files "),
            main_layout[0],
        );
        
        let items = self.get_filenames_to_show();
        let mut state = ListState::default().with_selected(Some(self.selection_index));

        let list = List::new(items)
            .block(
                Block::default()
                    .title(" Current directory ")
                    .borders(Borders::ALL),
            )
            .style(Style::default())
            .highlight_style(Style::default().add_modifier(Modifier::BOLD))
            .repeat_highlight_symbol(true)
            .direction(ListDirection::TopToBottom);

        frame.render_stateful_widget(list, main_layout[1], &mut state);
    }

    fn toggle_show_hidden(&mut self) -> () {
        self.show_hidden = !self.show_hidden;
    }

    fn get_filenames_to_show(&self) -> Vec<String> {
        let filenames: Vec<String> = self.current_entries.iter().map(|entry| entry.filename()).collect();

        if self.show_hidden {
            filenames
        }
        else {
            filenames
                .into_iter()
                .filter(|f| !f.starts_with('.'))
                .collect()
        }
    }

    fn get_number_of_showed_entries(&self) -> usize {
        if self.show_hidden {
            self.current_entries.len()
        }
        else {
            self.current_entries
                .iter()
                .filter(|entry| !entry.is_hidden())
                .count()
        }
    }

    fn increase_index(&mut self) -> () {
        self.selection_index += 1;
        self.selection_index = self.selection_index % self.get_number_of_showed_entries();
    }

    fn decrease_index(&mut self) -> () {
        if self.selection_index > 0 {
            self.selection_index -= 1;
        }
        else {
            self.selection_index = self.get_number_of_showed_entries() - 1;
        }
    }

    pub fn proccess_ui_event(&mut self, event: UIEvent) -> () {
        match event {
            UIEvent::Quit => self.should_quit = true,
            UIEvent::ToggleHidden => self.toggle_show_hidden(),
            UIEvent::Up => self.decrease_index(),
            UIEvent::Down => self.increase_index(),
            _ => {}
        }
    }
}
