use tiles::tiler::*;

fn main() {
    let mut tile_set: TileSetBits<CardinalDirections> = TileSetBits::new(12);

    let width = 150;
    let height = 500;

    let coordinate_system = CoordinateSystemGrid::new(width, height);
    for dir in CardinalDirections::DIRECTIONS {
        tile_set.set_allow(0, 1, *dir, true);
        tile_set.set_allow(1, 2, *dir, true);
        tile_set.set_allow(2, 3, *dir, true);
        tile_set.set_allow(3, 4, *dir, true);
    }

    for i in 0..5 {
        tile_set.set_allow(i, i, North, true);
        tile_set.set_allow(i, i, East, true);
    }

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
