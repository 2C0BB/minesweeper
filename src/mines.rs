use rand::Rng;
use std::cmp;

#[derive(Debug, Clone, PartialEq)]
enum TileType {
    Mine,
    Empty(u8),
}

#[derive(Debug, Clone)]
struct TileData {
    tile_type: TileType,
    opened: bool,
    flagged: bool
}

impl TileData {
    fn new(tile_type: TileType, opened: bool, flagged: bool) -> TileData {
        TileData {tile_type, opened, flagged}
    }

    fn to_char(&self) -> char {
        if self.opened {
            match self.tile_type {
                TileType::Mine => '*',
                TileType::Empty(n) => {
                    if n == 0 {
                        ' '
                    } else {
                        (n + 48) as char
                    }
                },
            }
        } else {
            if self.flagged {
                'F'
            } else {
                '#'
            }
        }
    }
}

fn get_neighbours(x: usize, y: usize) -> Vec<[usize; 2]>  {

    let mut out: Vec<[usize; 2]> = Vec::new();

    for r in cmp::max(y, 1)-1..=cmp::min(y, 8)+1 {
        for c in cmp::max(x, 1)-1..=cmp::min(x, 8)+1 {
            if !(r == y && c == x) {
                out.push([c, r]);
            }
        }
    }

    return out;
}

fn vec_at_pos<'a, T>(arr: &'a Vec<Vec<T>>, pos: &[usize; 2]) -> &'a T {
    return &arr[pos[1]][pos[0]];
}

fn vec_at_pos_mut<'a, T>(arr: &'a mut Vec<Vec<T>>, pos: &[usize; 2]) -> &'a mut T {
    return &mut arr[pos[1]][pos[0]];
}

fn get_random_positions(min: usize, max: usize, amount: usize) -> Vec<[usize; 2]> {
    let mut rng = rand::thread_rng();
    let mut out: Vec<[usize; 2]> = Vec::new();

    for _ in 0..amount {
        let mut val: [usize; 2];

        loop {
            val = [rng.gen_range(min..=max), rng.gen_range(min..=max)];

            if !out.contains(&val) {
                out.push(val);
                break;
            }
        }
    }

    return out;
}

pub struct Map {
    tiles: Vec<Vec<TileData>>,
}

impl Map {
    pub fn new() -> Map {

        let mut tiles: Vec<Vec<TileData>> = vec![
            vec![TileData::new(TileType::Empty(0), false, false); 10];
        10];

        for mine_pos in get_random_positions(0, 9, 10) {
            tiles[mine_pos[1]][mine_pos[0]] = TileData::new(TileType::Mine, false, false);
        }

        let checking_tiles = tiles.clone();

        for (y, line) in tiles.iter_mut().enumerate() {
            for (x, tile) in line.iter_mut().enumerate() {

                if tile.tile_type == TileType::Mine {
                    continue;
                }

                let neighbours: usize = get_neighbours(x, y)
                    .iter()
                    .map(|n| vec_at_pos(&checking_tiles, &[n[0], n[1]]))
                    .filter(|n| n.tile_type == TileType::Mine)
                    .count();

                tile.tile_type = TileType::Empty(neighbours as u8);
            }
        }


        Map {
            tiles,
        }
    }

    pub fn dig(&mut self, x: usize, y: usize) -> bool {

        // return is current value is mine
        {
            let temp_tile: &mut TileData = vec_at_pos_mut(&mut self.tiles, &[x, y]);

            if temp_tile.flagged {
                return false;
            }

            if temp_tile.tile_type == TileType::Mine {
                temp_tile.opened = true;
                return true;
            }
        }

        let mut to_check: Vec<[usize; 2]> = Vec::new();
        to_check.push([x, y]);

        while !to_check.is_empty() {
            let checking_pos: [usize; 2] = to_check.pop().unwrap();
            let checking_type: TileType;
            {
                let checking: &mut TileData = vec_at_pos_mut(&mut self.tiles, &checking_pos);
                checking.opened = true;

                checking_type = checking.tile_type.clone();
            }

            match checking_type {
                TileType::Empty(n) => {
                    if n == 0 {
                        let mut unopened_neighbours: Vec<[usize; 2]> = get_neighbours(checking_pos[0], checking_pos[1])
                            .into_iter()
                            .filter(|neigh| vec_at_pos(&self.tiles, neigh).opened == false)
                            .collect();

                        to_check.append(&mut unopened_neighbours);
                    }
                },
                _ => unreachable!(),
            }
        }

        false
    }

    pub fn flag(&mut self, x: usize, y: usize) {
        let tile: &mut TileData = vec_at_pos_mut(&mut self.tiles, &[x, y]);
        tile.flagged = !tile.flagged;
    }

    pub fn is_done(&self) -> bool {
        let non_mines: Vec<&TileData> = self.tiles
            .iter()
            .flatten()
            .filter(|x| x.tile_type != TileType::Mine)
            .collect();

        return non_mines.iter().all(|x| x.opened);
    }

    pub fn to_string(&self) -> String {
        self.tiles.iter()
            .map(|line| line.iter().map(|x| x.to_char()).collect::<String>())
            .collect::<Vec<String>>()
            .join("\n")
    }
}
