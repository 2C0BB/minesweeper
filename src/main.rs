pub mod mines;

use std::io::{self, Write};
use std::time::{Duration, Instant};
use std::env;

use crossterm::{style::{Color, Stylize}, queue, execute, event::{poll, read, Event, KeyCode}};

fn update_screen(map: &mines::Map, curs_x: usize, curs_y: usize, time: Duration) {

    queue!(io::stdout(), 
           crossterm::terminal::Clear(crossterm::terminal::ClearType::All),
           crossterm::cursor::MoveTo(0, 0)).unwrap();

    queue!(io::stdout(),
        crossterm::style::Print(format!("Time: {:?} seconds.", time.as_secs())),
        crossterm::cursor::MoveDown(1),
        crossterm::cursor::MoveToColumn(0),
    ).unwrap();

    for (row, line) in map.to_string().split("\n").enumerate() {

        for (col, ch) in line.chars().enumerate() {
            let mut styled = crossterm::style::StyledContent::new(
                    crossterm::style::ContentStyle::new(),
                    ch,
            );

            styled = match ch {
                '\u{2691}' => styled.red(),
                '1' => styled.blue(),
                '2' => styled.green(),
                '3' => styled.red(),
                '4' => styled.dark_blue(),
                '5' => styled.dark_red(),
                '6' => styled.cyan(),
                '7' => styled.dark_magenta(),
                '8' => styled.grey(),

                _ => styled,
            };

            if row == curs_y && col == curs_x {
                styled = styled.on(Color::Magenta);
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

    let start = Instant::now();

    let argv: Vec<String> = env::args().collect();
    let argc: usize = argv.len();


    let map_size: usize;
    let mines_num: usize;

    match argc {
        3 => {
            let num_args: Vec<usize> = argv.iter()
                .skip(1)
                .map(|x| x.parse::<usize>().unwrap())
                .collect();
           
            map_size = num_args[0];
            mines_num = num_args[1];

            if mines_num > map_size * map_size {
                println!("too many mines for map size");
                return;
            }
        }

        _ => {
            map_size = 10;
            mines_num = 10;
        }
    }

    let mut map: mines::Map = mines::Map::new(map_size, mines_num);

    crossterm::terminal::enable_raw_mode().unwrap();
    queue!(io::stdout(), 
           crossterm::cursor::Hide,
           crossterm::terminal::Clear(crossterm::terminal::ClearType::All)
    ).unwrap();

    let mut curs_x: usize = 0;
    let mut curs_y: usize = 0;

    let mut should_update: bool = true;
    let mut should_finish: bool = false;

    let mut last_time: Duration = start.elapsed();

    loop {

        let current_time: Duration = start.elapsed();

        if current_time.as_secs() > last_time.as_secs() {
            should_update = true;
        }
        last_time = current_time;
        
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
                                if curs_y < map_size - 1 {
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
                                if curs_x < map_size - 1 {
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
            update_screen(&map, curs_x, curs_y, current_time);
            should_update = false;
        }

        io::stdout().flush().unwrap();

        if map.is_done() {
            println!("you win!");

            should_finish = true;
        }

        if should_finish {
            println!("Game complete in: {:?} seconds.", current_time.as_secs());
            break;
        }
    }

    execute!(io::stdout(), crossterm::cursor::Show).unwrap();
}
