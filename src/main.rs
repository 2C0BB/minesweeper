pub mod mines;

use std::io::{self, Write};
use crossterm::{style::Print, queue};

fn update_screen(map: mines::Map) {

    queue!(io::stdout(), 
           crossterm::terminal::Clear(crossterm::terminal::ClearType::All),
           crossterm::cursor::MoveTo(0,0)).unwrap();

    for line in map.to_string().split("\n") {
        queue!(io::stdout(), 
               Print(line),
               crossterm::cursor::MoveDown(0),
               crossterm::cursor::MoveToColumn(0),
        ).unwrap();
    }
}

fn main() {
    let map: mines::Map = mines::Map::new();
    println!("{}", map.to_string());

    queue!(io::stdout(), crossterm::terminal::Clear(crossterm::terminal::ClearType::All)).unwrap();

    update_screen(map);

    io::stdout().flush().unwrap();
}
