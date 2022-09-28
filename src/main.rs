mod graphics;
mod tiler;

use std::path::Path;

use graphics::*;
use tiler::*;
const DIRECTIONS: [CardinalDirections; 4] = [North, East, South, West];

fn main() {
    let width = 40;
    let height = 40;

    let mut tile_set: TileSetKeys<CardinalDirections> = TileSetKeys::new(24 * 4, 0);

    //Skog = 0, Väg = 1, Stad = 2

    let mut keys: [[u8; 4]; 24] = [
        [0, 0, 1, 0],
        [0, 0, 0, 0],
        [2, 2, 2, 2],
        [2, 1, 0, 1],
        [2, 0, 0, 0],
        [0, 2, 0, 2],
        [0, 2, 0, 2],
        [0, 2, 0, 2],
        [2, 2, 0, 0],
        [2, 1, 1, 0],
        [2, 0, 1, 1],
        [2, 1, 1, 1],
        [2, 2, 0, 0],
        [2, 2, 0, 0],
        [2, 1, 1, 2],
        [2, 1, 1, 2],
        [2, 2, 0, 2],
        [2, 2, 0, 2],
        [2, 2, 1, 2],
        [2, 2, 1, 2],
        [1, 0, 1, 0],
        [0, 0, 1, 1],
        [0, 1, 1, 1],
        [1, 1, 1, 1],
    ];

    let mut vec = Vec::with_capacity(24 * 4 * 4);

    for mut tile in keys {
        for _r in 0..4 {
            vec.append(&mut tile.to_vec());
            tile.rotate_right(1);
        }
    }

    let mut chances = Vec::with_capacity(24 * 4);

    for tile in keys {
        let mut product = 1.0;

        for i in tile {
            product *= match i {
                0 => 2.0,
                1 => 1.0,
                2 => 0.5,
                _ => unreachable!(),
            };
        }
        chances.push(product);
        chances.push(product);
        chances.push(product);
        chances.push(product);
    }

    for i in 0..8 {
        chances[i] = 0.2;
    }

    tile_set.probability_weights = chances;
    tile_set.keys = vec;

    //tile_set.set_allow(x, y, dir, val)

    let coordinate_system = CoordinateSystemGrid::new(width, height);

    let mut tiler = TilerPossibilities::new(coordinate_system, tile_set);

    tiler.tile().unwrap();

    let tiles = tiler.tiling(24 * 4 + 1, 24 * 4 + 1);

    let textures = load_textures(Path::new("resources/rotated_texture.jpg"), 24 * 4 + 1);

    create_image(width, height, tiles, textures)
        .save(Path::new("tiling.png"))
        .unwrap();
}

fn main2() {
    let mut tile_set: TileSetKeys<CardinalDirections> = TileSetKeys::new(16, 0);

    for i in 0..16 {
        if i % 2 == 0 {
            tile_set.set_key(i, North, 1);
        }

        if (i / 2) % 2 == 0 {
            tile_set.set_key(i, East, 1);
        }

        if (i / 4) % 2 == 0 {
            tile_set.set_key(i, South, 1);
        }

        if (i / 8) % 2 == 0 {
            tile_set.set_key(i, West, 1);
        }
    }

    let width = 20;
    let height = 20;

    //tile_set.set_allow(x, y, dir, val)

    let coordinate_system = CoordinateSystemGrid::new(width, height);

    let mut tiler = TilerPossibilities::new(coordinate_system, tile_set);

    tiler.tile().unwrap();

    let tiles = tiler.tiling(18, 17);

    let grid = tiler.tiling(3, 4);

    let textures = load_textures(Path::new("resources/squares2.png"), 16);

    create_image(width, height, tiles, textures)
        .save(Path::new("tiling.png"))
        .unwrap();
}

fn main_old() {
    let mut tile_set: TileSetBits<CardinalDirections> = TileSetBits::new(12);

    let width = 150;
    let height = 500;

    let coordinate_system = CoordinateSystemGrid::new(width, height);
    //,TILES ( )

    /*
    for dir in [North, South] {
        tile_set.set(0, 0, dir, true);
        tile_set.set(0, 1, dir, true);
        tile_set.set(1, 1, dir, true);
    }

    tile_set.set(0, 1, West, true);
    tile_set.set(0, 1, East, true);
    */

    /*
    for dir in DIRECTIONS {
        tile_set.set(0, 0, dir, true);
        //tile_set.set(0, 1, dir, true);
        tile_set.set(1, 1, dir, true);
    }
    */

    /*
    tile_set.set(0, 0, North, true);
    tile_set.set(0, 0, East, true);

    tile_set.set(1, 1, North, true);
    tile_set.set(1, 1, East, true);
    */

    /*
    for dir in [North, East, South, West] {
        tile_set.set(0, 1, dir, true)
    }
    */

    /*
    tile_set.set(0, 0, North, true);
    tile_set.set(0, 0, East, true);
    for dir in DIRECTIONS {
        tile_set.set(0, 1, dir, true)
    }
    */

    for dir in DIRECTIONS {
        tile_set.set_allow(0, 1, dir, true);
        tile_set.set_allow(1, 2, dir, true);
        tile_set.set_allow(2, 3, dir, true);
        tile_set.set_allow(3, 4, dir, true);
    }
    tile_set.set_allow(0, 0, North, true);
    tile_set.set_allow(0, 0, East, true);
    tile_set.set_allow(1, 1, North, true);
    tile_set.set_allow(1, 1, East, true);
    tile_set.set_allow(2, 2, North, true);
    tile_set.set_allow(2, 2, East, true);
    tile_set.set_allow(3, 3, North, true);
    tile_set.set_allow(3, 3, East, true);
    tile_set.set_allow(4, 4, North, true);
    tile_set.set_allow(4, 4, East, true);

    tile_set.set_allow(5, 6, South, true);
    tile_set.set_allow(6, 11, West, true);
    tile_set.set_allow(6, 7, East, true);
    tile_set.set_allow(6, 8, South, true);
    tile_set.set_allow(7, 9, South, true);
    tile_set.set_allow(11, 10, South, true);
    tile_set.set_allow(8, 9, East, true);
    tile_set.set_allow(8, 10, West, true);

    for i in 0..5 {
        tile_set.set_allow(5, i, East, true);
        tile_set.set_allow(5, i, North, true);
        tile_set.set_allow(5, i, West, true);

        tile_set.set_allow(7, i, North, true);
        tile_set.set_allow(7, i, East, true);

        tile_set.set_allow(9, i, East, true);
        tile_set.set_allow(9, i, South, true);

        tile_set.set_allow(8, i, South, true);

        tile_set.set_allow(10, i, South, true);
        tile_set.set_allow(10, i, West, true);

        tile_set.set_allow(11, i, West, true);
        tile_set.set_allow(11, i, North, true);
    }

    for tile in 5..12 {
        tile_set.set_probability_weight(tile, 0.01)
    }

    /*
       5
    11 6 7
    10 8 9
     _
    / \
    |_|
    */

    /*
    for dir in DIR {
        tile_set.set(0, 0, dir, true);
        tile_set.set(1, 1, dir, true);
        //tile_set.set(2, 2, dir, true);
    }
    tile_set.set(0, 1, South, true);
    //tile_set.set(0, 1, East, true);
    //tile_set.set(1, 2, South, true);
    //tile_set.set(1, 2, East, true);
    */

    let mut tiler = TilerPossibilities::new(coordinate_system, tile_set);

    tiler.tile().unwrap();

    let grid = tiler.tiling(3, 4);

    let mut output = String::with_capacity(width * (height + 1));
    let mut iter = grid
        .iter()
        .map(|t| [' ', '.', '-', 'o', 'O', '_', ' ', '\\', '_', '|', '|', '/'][*t as usize]);
    for _ in 0..height {
        for _ in 0..width {
            output.push(iter.next().or(Some('X')).unwrap());
        }
        output.push('\n')
    }

    print!("{}", output);
}
