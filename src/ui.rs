use ratatui::{prelude::*, widgets::*};

pub fn ui(frame: &mut Frame) {
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

    let mut state = ListState::default().with_selected(Some(1));

    let items = ["Item 1", "Item 2", "Item 3"];
    let list = List::new(items)
        .block(Block::default().title(" Current directory ").borders(Borders::ALL))
        .style(Style::default())
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .repeat_highlight_symbol(true)
        .direction(ListDirection::TopToBottom);

    frame.render_stateful_widget(list, main_layout[1], &mut state);
}