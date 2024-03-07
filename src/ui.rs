use ratatui::{prelude::*, widgets::*};
use crate::file_manager::*;


/// Takes care of the UI and acts as the interface between User and File Manager
pub struct UIManager {
    file_manager: Manager,
    selected_entries: Vec<Entry>,
    selection_index: usize,
}

impl UIManager {
    pub fn new(manager: Manager) -> UIManager {
        UIManager { file_manager: manager, selected_entries: Vec::new(), selection_index: 0 }
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
    
        let items = self.file_manager.filenames();
        let list = List::new(items)
            .block(Block::default().title(" Current directory ").borders(Borders::ALL))
            .style(Style::default())
            .highlight_style(Style::default().add_modifier(Modifier::BOLD))
            .repeat_highlight_symbol(true)
            .direction(ListDirection::TopToBottom);
    
        frame.render_stateful_widget(list, main_layout[1], &mut state);
    }
}