use crate::text_reader::{get_test_list, TestFileList};
use crate::ui;
use anyhow::Result;
use crossterm::{
    event::{self, poll, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::collections::HashMap;
use std::error::Error;
use std::io;
use std::time::{Duration, Instant};
use tui::{backend::CrosstermBackend, Terminal};

pub enum AppState {
    Help,
    MainMenu,
    TypingTest,
}

impl AppState {
    pub fn get_title(&self) -> &str {
        match self {
            Self::MainMenu => "Main menu",
            Self::TypingTest => "Typing test",
            Self::Help => "Help",
        }
    }
}

pub struct App<'a> {
    pub current_state: AppState,
    pub test_list: TestFileList,
    pub title: &'a str,
    pub should_quit: bool,
}

impl<'a> App<'a> {
    pub fn new(title: &'a str) -> App<'a> {
        App {
            current_state: AppState::MainMenu,
            test_list: HashMap::new(),
            title,
            should_quit: false,
        }
    }
}

pub fn run() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    terminal.hide_cursor()?;

    let mut app = App::new("tappie");
    let test_list = get_test_list();
    app.test_list = test_list;

    let tick_rate = Duration::from_millis(100);
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|mut f| {
            ui::draw(&mut f, &app);
        })?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.modifiers == KeyModifiers::CONTROL {
                    match key.code {
                        KeyCode::Char('q') => app.should_quit = true,
                        KeyCode::Char('h') => app.current_state = AppState::Help,
                        _ => {}
                    }
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }

        if app.should_quit {
            break;
        }
    }

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
