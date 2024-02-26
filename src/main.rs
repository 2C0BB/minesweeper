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
}

impl TileData {
    fn new(tile_type: TileType, opened: bool) -> TileData {
        TileData {tile_type, opened}
    }

    fn to_char(&self) -> char {
        if self.opened {
            match self.tile_type {
                TileType::Mine => '*',
                TileType::Empty(n) => (n + 48) as char,
            }
        } else {
            '#'
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

struct Map {
    tiles: Vec<Vec<TileData>>,
}

impl Map {
    fn new() -> Map {

        let mut tiles: Vec<Vec<TileData>> = vec![
            vec![TileData::new(TileType::Empty(0), true); 10];
        10];

        for mine_pos in get_random_positions(0, 9, 10) {
            tiles[mine_pos[1]][mine_pos[0]] = TileData::new(TileType::Mine, true);
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

    fn to_string(&self) -> String {
        self.tiles.iter()
            .map(|line| line.iter().map(|x| x.to_char()).collect::<String>())
            .collect::<Vec<String>>()
            .join("\n")
    }
}

fn main() {
    let map: Map = Map::new();
    println!("{}", map.to_string());

    let rand_pos: Vec<[usize; 2]> = get_random_positions(0, 10, 5);
    println!("{:?}", rand_pos);
}
