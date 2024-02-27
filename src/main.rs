pub mod mines;

use std::io::{self, Write};
use std::time::Duration;

use crossterm::{style::{Color, Stylize}, queue, execute, event::{poll, read, Event, KeyCode}};

fn update_screen(map: &mines::Map, curs_x: usize, curs_y: usize) {

    queue!(io::stdout(), 
           crossterm::terminal::Clear(crossterm::terminal::ClearType::All),
           crossterm::cursor::MoveTo(0, 0)).unwrap();

    for (row, line) in map.to_string().split("\n").enumerate() {

        for (col, ch) in line.chars().enumerate() {
            let mut styled = crossterm::style::StyledContent::new(
                    crossterm::style::ContentStyle::new(),
                    ch,
            );

            if row == curs_y && col == curs_x {
                styled = styled.on(Color::Blue);
            }

            queue!(io::stdout(), 
                   crossterm::style::PrintStyledContent(styled)).unwrap();
        }

        queue!(io::stdout(), 
               crossterm::cursor::MoveDown(0),
               crossterm::cursor::MoveToColumn(0),
        ).unwrap();
    }
}

fn main() {
    let mut map: mines::Map = mines::Map::new();

    crossterm::terminal::enable_raw_mode().unwrap();
    queue!(io::stdout(), 
           crossterm::cursor::Hide,
           crossterm::terminal::Clear(crossterm::terminal::ClearType::All)
    ).unwrap();

    let mut curs_x: usize = 0;
    let mut curs_y: usize = 0;

    let mut should_update: bool = true;
    let mut should_finish: bool = false;
    loop {
        if poll(Duration::ZERO).unwrap() {
            match read().unwrap() {
                Event::Key(event) => {
                    if event.kind == crossterm::event::KeyEventKind::Press {
                        match event.code {
                            KeyCode::Esc => {
                                should_finish = true;
                            }

                            KeyCode::Up => {
                                if curs_y > 0 {
                                    curs_y -= 1;
                                    should_update = true;
                                }
                            }
                            KeyCode::Down => {
                                if curs_y < 9 {
                                    curs_y += 1;
                                    should_update = true;
                                }
                            }
                            KeyCode::Left => {
                                if curs_x > 0 {
                                    curs_x -= 1;
                                    should_update = true;
                                }
                            }
                            KeyCode::Right => {
                                if curs_x < 9 {
                                    curs_x += 1;
                                    should_update = true;
                                }
                            }

                            KeyCode::Char(ch) => {
                                match ch {
                                    'd' => {
                                        if map.dig(curs_x, curs_y) {
                                            should_finish = true;
                                        }
                                        should_update = true;
                                    },
                                    'f' => {
                                        map.flag(curs_x, curs_y);
                                        should_update = true;
                                    }
                                    _ => {}
                                }
                            }

                            _ => {}
                        }
                    }
                },
                _ => {}
            };
        }

        if should_update {
            update_screen(&map, curs_x, curs_y);
            should_update = false;
        }

        io::stdout().flush().unwrap();

        if map.is_done() {
            println!("win!!");
            return;
        }

        if should_finish {
            break;
        }
    }

    execute!(io::stdout(), crossterm::cursor::Show).unwrap();
}
