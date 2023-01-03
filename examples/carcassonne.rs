use std::path::Path;

use tiles::graphics::*;
use tiles::tiler::*;

fn main() {
    let width = 40;
    let height = 40;

    let mut tile_set: TileSetKeys<CardinalDirections> = TileSetKeys::new(24 * 4, 0);

    //Woods = 0, Roads = 1, City = 2
    let keys: [[u8; 4]; 24] = [
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


    let coordinate_system = CoordinateSystemGrid::new(width, height);

    let mut tiler = TilerPossibilities::new(coordinate_system, tile_set);

    tiler.tile().unwrap();

    let tiles = tiler.tiling(24 * 4 + 1, 24 * 4 + 1);

    let textures = load_textures(Path::new("resources/rotated_texture.jpg"), 24 * 4 + 1);

    create_image(width, height, tiles, textures)
        .save(Path::new("tiling.png"))
        .unwrap();
}

