mod app;
mod event;
mod game;
mod utils;

use std::{error::Error, io, time::Duration};
use termion::{event::Key, input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::{
    backend::TermionBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color},
    widgets::{
        canvas::{Canvas, Line, Points},
        Block, Borders,
    },
    Terminal,
};

use app::App;
use event::{Config, Event, Events};
use game::Command;

fn main() -> Result<(), Box<dyn Error>> {
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let config = Config {
        tick_rate: Duration::from_millis(250),
        ..Default::default()
    };
    let events = Events::with_config(config);

    let mut app = App::new();

    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(80), Constraint::Percentage(20)].as_ref())
                .split(f.size());
            // params
            let board_size = app.get_size();
            let panel_size = board_size + (board_size / 3.0);
            let half_box_size = app.box_size / 2.0;
            let font_width = 2.0;
            // Game board
            let canvas = Canvas::default()
                .block(Block::default().borders(Borders::ALL).title("2048-@wander"))
                .paint(|ctx| {
                    let grid = app.get_grid();
                    for (row, list) in grid.iter().enumerate() {
                        for (col, _) in list.iter().enumerate() {
                            // 盒子参数
                            let score = grid[row][col];
                            let s = if score == 0 {
                                String::from("").into_boxed_str()
                            } else {
                                pad_str(score.to_owned().to_string(), 6).into_boxed_str()
                            };
                            let x_box = (col as f64) * app.box_size;
                            let y_box = (row as f64) * app.box_size;
                            ctx.print(
                                ((col + 1) as f64) * app.box_size - half_box_size - font_width,
                                ((4 - row) as f64) * app.box_size
                                    - half_box_size
                                    - font_width * 2.0,
                                Box::leak(s),
                                score_to_color(score),
                            );
                            ctx.draw(&Line {
                                x1: x_box,
                                y1: y_box,
                                x2: x_box + app.box_size,
                                y2: y_box,
                                color: Color::Green,
                            });
                            ctx.draw(&Line {
                                x1: x_box,
                                y1: y_box,
                                x2: x_box,
                                y2: y_box + app.box_size,
                                color: Color::Green,
                            });
                            ctx.draw(&Line {
                                x1: x_box + app.box_size,
                                y1: y_box,
                                x2: x_box + app.box_size,
                                y2: y_box + app.box_size,
                                color: Color::Green,
                            });
                            ctx.draw(&Line {
                                x1: x_box,
                                y1: y_box + app.box_size,
                                x2: x_box + app.box_size,
                                y2: y_box + app.box_size,
                                color: Color::Green,
                            });
                        }
                    }

                    if !app.is_alive() {

                        ctx.draw(&Points {
                            coords: &app.get_game_over_modal(),
                            color: Color::Green
                        });

                        ctx.print(
                            app.box_size * 1.5,
                            app.box_size * 2.0,
                            " GAME OVER! ",
                            Color::Blue,
                        );

                        ctx.print(
                            app.box_size * 1.3,
                            app.box_size * 1.8,
                            " Restart[R] Quit[Q] ",
                            Color::Blue,
                        );
                    }
                })
                .x_bounds([0.0, board_size])
                .y_bounds([0.0, board_size]);
            f.render_widget(canvas, chunks[0]);
            // Informantions
            let canvas = Canvas::default()
                .block(Block::default().borders(Borders::ALL).title("Panel"))
                .paint(|ctx| {
                    ctx.print(board_size, board_size, "> Relax <", Color::Blue);

                    let score = app.get_score().to_owned().to_string().into_boxed_str();
                    ctx.print(board_size, board_size - 30.0, "Score:", Color::Green);
                    ctx.print(
                        board_size,
                        board_size - 40.0,
                        Box::leak(score),
                        Color::Green,
                    );

                    ctx.print(board_size, 0.0, "Quit[Q]", Color::Blue);
                })
                .x_bounds([board_size, panel_size])
                .y_bounds([0.0, board_size]);
            f.render_widget(canvas, chunks[1]);
        })?;

        // Events
        match events.next()? {
            Event::Input(input) => match input {
                Key::Char('q') => {
                    break;
                }
                Key::Char('r') => {
                    app.restart();
                }
                // left up right down
                Key::Down => {
                    app.add_command(Command::Down);
                }
                Key::Up => {
                    app.add_command(Command::Up);
                }
                Key::Right => {
                    app.add_command(Command::Right);
                }
                Key::Left => {
                    app.add_command(Command::Left);
                }
                // h k l j   vim keys support
                Key::Char('h') => {
                    app.add_command(Command::Left);
                }
                Key::Char('k') => {
                    app.add_command(Command::Up);
                }
                Key::Char('l') => {
                    app.add_command(Command::Right);
                }
                Key::Char('j') => {
                    app.add_command(Command::Down);
                }
                _ => {
                    app.add_command(Command::Nil);
                }
            },
            Event::Tick => {
                app.next()
            }
        }
    }

    Ok(())
}

/// make different strings as same length
fn pad_str(s: String, length: usize) -> String {
    let mut s = s.clone();
    loop {
        if s.len() < length {
            s.push_str(" ");
        } else {
            break;
        }
    }

    s
}

/// render different color for different score
fn score_to_color(score: i32) -> Color {
    if score < 64 {
        Color::Green
    } else if score < 256 {
        Color::Magenta
    } else if score < 1024 {
        Color::Cyan
    } else if score < 4096 {
        Color::LightRed
    } else {
        Color::Red
    }
}
