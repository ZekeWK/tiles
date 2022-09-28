use std::path::Path;

use image::{self, GenericImageView, Rgba, ImageBuffer, RgbaImage, GenericImage};

use crate::tiler::TileType;

pub type Image = ImageBuffer<Rgba<u8>, Vec<u8>>;

pub fn load_textures(path : &Path, tile_cnt : usize) -> Vec<Image> {
    let texture = image::open(path).unwrap();

    let picture_dimensions = texture.dimensions();

    let (x, y) = (picture_dimensions.0 / tile_cnt as u32, picture_dimensions.1);
    
    let mut textures = Vec::with_capacity(tile_cnt);

    for i in 0..tile_cnt {
        let view = *texture.view(x * i as u32, 0, x, y);
        let mut buffer = RgbaImage::new(x, y);

        buffer.copy_from(&view, 0, 0).unwrap();
        textures.push(buffer);
    }

    textures
}

pub fn load_texture(path : &Path) -> Image {
    let texture = image::open(path).unwrap();
    texture.to_rgba8()
}

pub fn create_image(width : usize, height : usize, tiles : Vec<TileType>, textures : Vec<Image>) -> Image {
    let (x, y) = textures[0].dimensions();

    let mut output = RgbaImage::new(width as u32 * x, height as u32 * y);

    for r in 0..height {
        for c in 0..width {
            let tile = &textures[tiles[r * width + c] as usize];
            output.copy_from(tile, c as u32*x, r as u32*y).unwrap();
        }
    }

    output
}