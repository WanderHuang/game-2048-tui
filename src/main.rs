mod app;
mod canvas_box;
mod event;
mod game;

use std::{error::Error, io, ops::Add, time::Duration};
use termion::{event::Key, input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::{
    backend::TermionBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::Span,
    widgets::{
        canvas::{Canvas, Line, Map, MapResolution, Points, Rectangle},
        Block, Borders,
    },
    Terminal,
};

use app::App;
use canvas_box::render_box;
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
            // 基本参数
            let board_size = app.get_size();
            let panel_size = board_size + (board_size / 3.0);
            let half_box_size = app.box_size / 2.0;
            let font_width = 2.0;
            // 游戏页面
            let canvas = Canvas::default()
                .block(Block::default().borders(Borders::ALL).title("2048"))
                .paint(|ctx| {
                    let grid = app.get_grid();
                    for (row, list) in grid.iter().enumerate() {
                        for (col, _) in list.iter().enumerate() {
                            // 盒子参数
                            let s =
                                pad_str(grid[row][col].to_owned().to_string(), 6).into_boxed_str();
                            let x_box = (col as f64) * app.box_size;
                            let y_box = (row as f64) * app.box_size;
                            ctx.print(
                                ((col + 1) as f64) * app.box_size - half_box_size - font_width,
                                ((4 - row) as f64) * app.box_size
                                    - half_box_size
                                    - font_width * 2.0,
                                Box::leak(s),
                                Color::Yellow,
                            );
                            ctx.draw(&Points {
                                coords: &render_box(x_box, y_box, app.box_size, app.box_size),
                                color: Color::Green,
                            });
                            ctx.draw(&Line {
                                x1: x_box,
                                y1: y_box,
                                x2: x_box + app.box_size,
                                y2: y_box,
                                color: Color::Black,
                            });
                            ctx.draw(&Line {
                                x1: x_box,
                                y1: y_box,
                                x2: x_box,
                                y2: y_box + app.box_size,
                                color: Color::Black,
                            });
                            ctx.draw(&Line {
                                x1: x_box + app.box_size,
                                y1: y_box,
                                x2: x_box + app.box_size,
                                y2: y_box + app.box_size,
                                color: Color::Black,
                            });
                            ctx.draw(&Line {
                                x1: x_box,
                                y1: y_box + app.box_size,
                                x2: x_box + app.box_size,
                                y2: y_box + app.box_size,
                                color: Color::Black,
                            });
                        }
                    }
                })
                .x_bounds([0.0, board_size])
                .y_bounds([0.0, board_size]);
            f.render_widget(canvas, chunks[0]);
            // 统计页面
            let canvas = Canvas::default()
                .block(Block::default().borders(Borders::ALL).title("Panel"))
                .paint(|ctx| {
                    let score = app.get_score().to_owned().to_string().into_boxed_str();
                    ctx.print(board_size, board_size - 5.0, "Score:", Color::Yellow);
                    ctx.print(
                        board_size,
                        board_size - 10.0,
                        Box::leak(score),
                        Color::Yellow,
                    );
                })
                .x_bounds([board_size, panel_size])
                .y_bounds([0.0, board_size]);
            f.render_widget(canvas, chunks[1]);
        })?;
        match events.next()? {
            Event::Input(input) => match input {
                Key::Char('q') => {
                    break;
                }
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
                _ => {
                    app.add_command(Command::Nil);
                }
            },
            Event::Tick => {
                // 一个时钟只取一个命令
                app.next()
            }
        }
    }

    Ok(())
}

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
