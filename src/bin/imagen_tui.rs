use crossterm::event;
use ratatui;

fn main() -> std::io::Result<()> {
    ratatui::run(|terminal| {
        loop {
            terminal.draw(|frame| frame.render_widget("Hello rammy", frame.area()))?;
            if event::read()?.is_key_press() {
                break Ok(());
            }
        }
    })
}
