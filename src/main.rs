use anyhow::Result;
use ratatui::prelude::*;

#[tokio::main]
async fn main() {
    human_panic::setup_panic!();
    println!("testing...");
}

struct App {
    should_quit: bool,
    steam_path: std::path::PathBuf,
    // ???
}

impl App {
    const FRAMES_PER_SECOND: f32 = 60.0;

    pub async fn run(mut self, mut terminal: ratatui::DefaultTerminal) -> Result<()> {
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs_f32(1.0 / Self::FRAMES_PER_SECOND));
        let mut events = crossterm::event::EventStream::new();

        while !self.should_quit {
            tokio::select! {
                _ = interval.tick() => { terminal.draw(|frame| self.render(frame))?; },
                Some(Ok(event)) = events.next() => self.handle_event(&event),
            }
        }
        Ok(())
    }

    fn render(&self, frame: &mut Frame) {
                let layout = Layout::vertical([Constraint::Length(1), Constraint::Fill(1)]);
        let [title_area, body_area] = frame.area().layout(&layout);
        let title = Line::from("Ratatui async example").centered().bold();
        frame.render_widget(title, title_area);
    }

    fn handle_event(&mut self, event: &crossterm::event::Event){
        use crossterm::event::KeyCode;
        if let Some(key) = event.as_key_event() {
            match key.code {
                KeyCode::Char('q') | KeyCode::Esc => self.should_quit = true,
                _ => {}
            }
        }
    }
}