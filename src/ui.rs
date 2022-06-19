use crate::app::App;
use crate::text_reader::TestFileList;
use tui::layout::Rect;
use tui::style::{Color, Modifier};
use tui::widgets::{Paragraph, Wrap};
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::Style,
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem},
    Frame,
};

pub fn draw<B>(f: &mut Frame<B>, app: &App)
where
    B: Backend,
{
    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(4),
                Constraint::Min(5),
                Constraint::Max(8),
            ]
            .as_ref(),
        )
        .split(f.size());

    draw_top_bar(f, app, main_layout[0]);
    draw_content_base(f, app, main_layout[1]);
    draw_input_block(f, main_layout[2]);
}

pub fn draw_top_bar<B>(f: &mut Frame<B>, app: &App, layout_rect: Rect)
where
    B: Backend,
{
    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(80), Constraint::Percentage(20)].as_ref())
        .split(layout_rect);

    draw_status(f, app, layout[0]);
    draw_help(f, layout[1]);
}

pub fn draw_status<B>(f: &mut Frame<B>, app: &App, layout_rect: Rect)
where
    B: Backend,
{
    let text: Vec<Spans> = vec![Spans::from(vec![
        Span::from("Current: "),
        Span::from(app.current_state.get_title()),
    ])];
    let block = Block::default().borders(Borders::ALL).title("tappie");
    let paragraph = Paragraph::new(text).block(block);
    f.render_widget(paragraph, layout_rect);
}

pub fn draw_help<B>(f: &mut Frame<B>, layout_rect: Rect)
where
    B: Backend,
{
    let text: Vec<Spans> = vec![
        Spans::from(vec![
            Span::styled(
                "CTRL + H",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::from(": Help"),
        ]),
        Spans::from(vec![
            Span::styled(
                "CTRL + Q",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::from(": Quit"),
        ]),
    ];
    let block = Block::default().borders(Borders::ALL);
    let paragraph = Paragraph::new(text).block(block).wrap(Wrap { trim: true });
    f.render_widget(paragraph, layout_rect);
}

pub fn draw_content_base<B>(f: &mut Frame<B>, app: &App, layout_rect: Rect)
where
    B: Backend,
{
    draw_test_list(f, app, layout_rect);
}

pub fn draw_input_block<B>(f: &mut Frame<B>, layout_rect: Rect)
where
    B: Backend,
{
    let block = Block::default().borders(Borders::ALL);
    f.render_widget(block, layout_rect);
}

pub fn draw_test_list<B>(f: &mut Frame<B>, app: &App, layout_rect: Rect)
where
    B: Backend,
{
    let mut list_items: Vec<ListItem> = Vec::new();
    for (key, _) in &app.test_list {
        let item_key = key.clone();
        let list_item = ListItem::new(item_key);
        list_items.push(list_item);
    }
    let block = Block::default().borders(Borders::ALL).title("Typing test");
    let list = List::new(list_items).block(block).highlight_symbol(">> ");
    f.render_widget(list, layout_rect);
}
