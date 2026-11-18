use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

use ratatui::crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::layout::{Constraint, Layout};
use ratatui::style::{Modifier, Style};
use ratatui::widgets::{Block, List, ListState, Paragraph};
use ratatui::{DefaultTerminal, Frame};

const TICK_RATE: Duration = Duration::from_millis(500);

struct Game {
    name: &'static str,
    command: &'static str,
    args: &'static [&'static str],
}

struct App {
    running: bool,
    ticks: u64, 
    key_presses: u64, /** some remaining of the code i was using to test if the app worked, you can completly remove key_presses 
                          and ticks (you would have to do some debug) (i'm not completly shure if removing it will break something or not,
                          thats why those function are still heare).
                          and yes my english suck, hope you understand anyways :)
                          **/
    games: Vec<Game>,
    list_state: ListState,
    message: String,
}

fn main() -> std::io::Result<()> {
    ratatui::run(|terminal| App::new().run(terminal))
}

impl App {
    fn new() -> Self {
        Self {
            running: true,
            ticks: 0,
            key_presses: 0,
            //-----GAME LIST-----:
            // modify it with your games name and directory 
            games: vec![
                Game {
                    name: "Pokopia - switch 2 emu",
                    command: "nvidia-offload",
                    args: &["/home/leo/Downloads/lichii-linux-x86_64-pgo.AppImage"],
                },
                Game {
                    name: "Zelda - Ocarina of time",
                    command: "nvidia-offload",
                    args: &["/home/leo/Games/Zelda/SoH-Dewey-Bravo.appimage"],
                },
                Game {
                    name: "Deadlock",
                    command: "steam",
                    args: &["steam://rungameid/1422450"],
                },
            ],
            //-------------------
            list_state: ListState::default().with_selected(Some(0)),
            message: String::new(),
        }
    }

    fn run(&mut self, terminal: &mut DefaultTerminal) -> std::io::Result<()> {
        let mut last_tick = Instant::now();

        while self.running {
            terminal.draw(|frame| self.render(frame))?;

            let timeout = TICK_RATE.saturating_sub(last_tick.elapsed());

            if event::poll(timeout)? {
                if let Event::Key(key) = event::read()? {
                    self.handle_key(key);
                }
            }

            if last_tick.elapsed() >= TICK_RATE {
                self.on_tick();
                last_tick = Instant::now();
            }
        }

        Ok(())
    }

    fn handle_key(&mut self, key: KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }
        self.key_presses += 1;

        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => self.running = false,
            KeyCode::Down | KeyCode::Char('j') => self.list_state.select_next(),
            KeyCode::Up | KeyCode::Char('k') => self.list_state.select_previous(),
            KeyCode::Enter => self.launch_game(),
            _ => {}
        }
    }

    fn launch_game(&mut self) {
        let Some(i) = self.list_state.selected() else {
            return;
        };
        let game = &self.games[i];

        let result = Command::new(game.command)
            .args(game.args)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn();

        self.message = match result {
            Ok(_) => format!("avviato {}", game.name),
            Err(e) => format!("impossibile avviare {}: {e}", game.command),
        };
    }

    fn on_tick(&mut self) {
        self.ticks += 1;
    }

    fn render(&mut self, frame: &mut Frame) {
        let [games_area, options] =
            Layout::vertical([Constraint::Percentage(90), Constraint::Percentage(10)])
                .areas(frame.area());

        let list = List::new(self.games.iter().map(|g| g.name))
            .block(Block::bordered().title("Games"))
            .highlight_style(Style::new().add_modifier(Modifier::REVERSED))
            .highlight_symbol("> ");

        frame.render_stateful_widget(list, games_area, &mut self.list_state);
        frame.render_widget(
            Paragraph::new(format!(
                "(↑) move up | (↓) move down | enter to run | q to quit\n{}",
                self.message
            )),
            options,
        );
    }
}
