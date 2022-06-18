use crate::app::App;
use tui::layout::Rect;
use tui::style::{Color, Modifier};
use tui::widgets::{Paragraph, Wrap};
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::Style,
    text::{Span, Spans},
    widgets::{Block, Borders},
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

pub fn draw_status<B>(f: &mut Frame<B>, _app: &App, layout_rect: Rect)
where
    B: Backend,
{
    let block = Block::default().borders(Borders::ALL).title("tappie");
    f.render_widget(block, layout_rect);
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

pub fn draw_content_base<B>(f: &mut Frame<B>, _app: &App, layout_rect: Rect)
where
    B: Backend,
{
    let block = Block::default().borders(Borders::ALL);
    f.render_widget(block, layout_rect);
}

pub fn draw_input_block<B>(f: &mut Frame<B>, layout_rect: Rect)
where
    B: Backend,
{
    let block = Block::default().borders(Borders::ALL);
    f.render_widget(block, layout_rect);
}
