use ansi_to_tui::IntoText;
use color_eyre::Result;
use crossterm::event::KeyCode;
use ratatui::{
    self, Frame,
    crossterm::event::{self, Event, KeyEventKind},
    layout::{Constraint, Layout, Rect},
    text::ToText,
    widgets::{Block, BorderType, Borders, Paragraph, Widget},
};

use imagen::{cli, image_loader, my_image};

#[derive(Clone, Debug)]
struct AppState {
    ascii_str: String,
}

fn main() -> Result<()> {
    let args = cli::parse();
    let img = image_loader::loader(&args);
    let text = my_image::draw(
        img,
        my_image::DrawArgs {
            color: args.color,
            edge: args.edge,
        },
    );

    let app_state = AppState { ascii_str: text };

    let mut terminal = ratatui::init();
    let result = run(&mut terminal, &app_state);
    ratatui::restore();
    result
}

fn run(terminal: &mut ratatui::DefaultTerminal, app_state: &AppState) -> Result<()> {
    loop {
        // draw terminal ui
        terminal.draw(|f| render(f, app_state))?;

        // break if returns true
        if handle_events()? {
            break Ok(());
        };
    }
}

fn render(frame: &mut ratatui::Frame, app_state: &AppState) {
    Paragraph::new(app_state.ascii_str.into_text().unwrap_or_default())
        .render(frame.area(), frame.buffer_mut())

    // let [border_area] = Layout::vertical([Constraint::Fill(1)])
    //     .margin(1)
    //     .areas(frame.area());
    //
    // Block::bordered()
    //     .border_type(BorderType::Rounded)
    //     .render(border_area, frame.buffer_mut());
}

// Handle keypress events, return true to quit
fn handle_events() -> Result<bool> {
    match event::read()? {
        Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
            KeyCode::Char('q') => return Ok(true),
            _ => {}
        },
        _ => {}
    }

    Ok(false)
}
