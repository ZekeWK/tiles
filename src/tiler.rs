use std::{iter::repeat, marker::PhantomData};
use bitvec::prelude::*;
use keyed_priority_queue::KeyedPriorityQueue;
use nohash_hasher::BuildNoHashHasher;
use rand::{distributions::WeightedIndex, prelude::Distribution};
use custom_error::custom_error;


///Each value represents a type of tile.
pub type TileType = u8;

///Represents all ways two tiles can interact. 
pub trait Directions : Clone + Copy + 'static { //TODO Make the id and directions better work together.
    ///Returns a slice containing all directions in any order.
    const DIRECTIONS : &'static [Self];

    //TODO Add better description
    ///Returns the directions id. These need to start at 0 and enumerate upwards, and if seen as a loop, needs to be once rotationaly symmetric if opposites are the same.
    fn id(self) -> usize;

    ///Returns the opposite direction, such that if x is dir of y, then y is dir of x.  
    fn opposite(self) -> Self;

    ///Returns the number of directions.
    const LEN : usize;
}

///Represents the coordinate system in which the tiling occurs, as well as its dimensions. Each tile must be represented by a single usize, enumerated from 0.
pub trait CoordinateSystem<Dir> where Dir : Directions {
    ///Moves pos once by dir. Returns None if this would take it out of the coordinate system.
    fn move_direction(&self, pos : usize, dir : Dir) -> Option<usize> where Dir : Directions;

    ///The total amount of tiles.
    fn len(&self) -> usize;
}

pub type Probability = f32;

///Represents a set of tiles (represented as TileType) and how they interact.
pub trait TileSet<Dir> where Dir : Directions {
    ///Returns whether x allows y to be dir of it. The result is equal to if y allows x to be dir.opposite() of it.
    fn allows(&self, x : TileType, y : TileType, dir : Dir) -> bool;

    ///Returns the amount of tile types represented in the TileSet.
    fn len(&self) -> TileType;

    ///When deciding the tiletype of an undetermined tile, the weights are used to decide at which rate. They do however not necessarily correspond to the actual frequency.
    fn probabilty_weight(&self, tile : TileType) -> Probability;
}

///Represents a way to tile the tiles from the tile set into a tiling.
pub trait Tiler<CS, TS, Dir> where CS : CoordinateSystem<Dir>, TS : TileSet<Dir>, Dir : Directions {
    ///Finishes the tiling proccess.
    fn tile(&mut self) -> Result<(), TilingError>;

    ///Creates a vector with the tiling
    fn tiling(&self, undetermined : TileType, impossible : TileType) -> Vec<TileType>;
}

custom_error! {pub TilingError
    Failed = "This tiling attempt failed.",
    Untileable = "There is no possible tiling."

}




pub use CardinalDirections::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CardinalDirections {
    North = 0,
    East = 1,
    South = 2,
    West = 3,
}

impl Directions for CardinalDirections {
    const DIRECTIONS : &'static [Self] = &[North, East, South, West];

    fn id(self) -> usize {
        match self {
            North => 0,
            East => 1,
            South => 2,
            West => 3,
        }
    }

    fn opposite(self) -> Self {
        match self {
            North => South,
            East => West,
            South => North,
            West => East
        }        
    }

    const LEN : usize = 4;
}

pub struct CoordinateSystemGrid<Dir> where Dir : Directions {
    pub width : usize,
    pub height : usize,
    pub directions : PhantomData<Dir>,
}

impl CoordinateSystem<CardinalDirections> for CoordinateSystemGrid<CardinalDirections> {
    fn move_direction(&self, pos : usize, dir : CardinalDirections) -> Option<usize> {
        let (mut r, mut c) = (pos / self.width, pos % self.width);
    
        match dir {
            North => {
                if r == 0 {return None}

                r -= 1;
            },
            South => {
                if r == self.height - 1 {return None}
                r += 1;
            },
            West => {
                if c == 0 {return None}
                c -= 1;
            }
            East => {
                if c == self.width - 1 {return None}
                c += 1;
            }
        };
        
        Some(r*self.width + c)
    }

    fn len(&self) -> usize {
        self.height * self.width
    }
}
 
impl <Dir> CoordinateSystemGrid<Dir> where Dir : Directions{
    pub fn new(width : usize, height : usize) -> Self {
        Self { width, height, directions: Default::default()}
    }
}

#[derive(Debug, Clone)]
pub struct TileSetBits<Dir> where Dir : Directions { 
    pub rules : BitVec,
    pub len : TileType,
    pub probability_weights : Vec<Probability>,
    pub directions : PhantomData<Dir>,
}

impl <Dir> TileSet<Dir> for TileSetBits<Dir> where Dir : Directions {
    fn allows(&self, mut x : TileType, mut y : TileType, mut dir : Dir) -> bool {
        if dir.id() > dir.opposite().id() {
            (x, y) = (y, x);
            dir = dir.opposite();
        }

        let len = self.len;
        
        self.rules[(x as usize * len as usize + y as usize) * Dir::LEN/2 + dir.id() as usize] 
    }

    fn len(&self) -> TileType {
        self.len
    }

    fn probabilty_weight(&self, tile : TileType) -> Probability {
        self.probability_weights[tile as usize]
    }
}

impl <Dir> TileSetBits<Dir> where Dir : Directions {
    pub fn new(len : TileType) -> Self {
        TileSetBits { rules: bitvec![0; len as usize * len as usize * Dir::LEN/2], len, probability_weights : vec![1.0; len as usize], directions : Default::default()} //Double check that this works as expected.
    }

    pub fn set_probability_weight(&mut self, tile : TileType, weight : Probability) {
        self.probability_weights[tile as usize] = weight;
    }

    pub fn set_allow(&mut self, mut x : TileType, mut y : TileType, mut dir : Dir, val : bool) {
        if dir.id() > dir.opposite().id() {
            (x, y) = (y, x);
            dir = dir.opposite();
        }

        let len = self.len;
        
        self.rules.set((x as usize * len as usize + y as usize) * Dir::LEN/2 + dir.id() as usize, val)
    }
}

pub struct TileSetKeys<Dir> where Dir : Directions {
    pub keys : Vec<TileType>,
    pub probability_weights : Vec<Probability>,
    pub directions : PhantomData<Dir>,
}

impl <Dir> TileSet<Dir> for TileSetKeys<Dir> where Dir : Directions {
    fn allows(&self, x : TileType, y : TileType, dir : Dir) -> bool {
        self.keys[Dir::LEN * x as usize + dir.id()] == self.keys[Dir::LEN * y as usize + dir.opposite().id()]
    }

    fn len(&self) -> TileType {
        (self.keys.len() / Dir::LEN) as u8
    }

    fn probabilty_weight(&self, tile : TileType) -> Probability {
        self.probability_weights[tile as usize]
    }
}

impl <Dir> TileSetKeys<Dir> where Dir : Directions {
    pub fn new(len : usize, standard_key : TileType) -> Self {
        TileSetKeys { keys: vec![standard_key; len * Dir::LEN], probability_weights: vec![1.0; len], directions: Default::default() }
    }

    pub fn set_key(&mut self, tile : TileType, dir : Dir, key : TileType) {
        self.keys[Dir::LEN * tile as usize + dir.id()] = key;
    }

    /*
    pub fn set_keys(&mut self, tile : TileType, keys : Vec<TileType>) {
        assert_eq!(keys.len() as TileType, self.len());
        
        for (dir, key) in Dir::DIRECTIONS.into_iter().zip(keys.into_iter()) {
            self.set_key(tile, dir, key);
        }
    }
    */

    pub fn set_probability_weight(&mut self, tile : TileType, weight : Probability) {
        self.probability_weights[tile as usize] = weight;
    }
}


pub struct TilerPossibilities<CS, TS, Dir> where TS : TileSet<Dir>, CS : CoordinateSystem<Dir>, Dir : Directions {
    pub possible_tiles : BitVec,
    pub queue : KeyedPriorityQueue<usize, u8, BuildNoHashHasher<usize>>, // KeyedPriorityQueue<usize, u8, RandomState>, 
    pub coordinate_system : CS,
    pub tile_set : TS,
    pub directions : PhantomData<Dir>
}

impl <TS, CS, Dir> Tiler<CS, TS, Dir> for TilerPossibilities<CS, TS, Dir> where TS : TileSet<Dir>, CS : CoordinateSystem<Dir>, Dir : Directions {
    fn tile(&mut self) -> Result<(), TilingError> {
        let mut rng = rand::prelude::thread_rng(); //TODO Make seedable
        let mut update_queue : Vec<usize> = Vec::new();
        let mut update_queue_contains = bitvec![0; self.coordinate_system.len()];
        let mut probability_weights = vec![0.0 as Probability; self.tile_set.len() as usize]; 

        for _ in 0..self.queue.len() {
            let (pos, disallowed) = self.queue.pop().unwrap();
            
            if disallowed == self.tile_set.len() {return Err(TilingError::Failed)}
            if disallowed == self.tile_set.len() -1 {continue;}
            
            
            for tile in 0..self.tile_set.len() {
                probability_weights[tile as usize] = 
                if !self.possible_tile(pos, tile) {
                    0.0
                }
                else {
                    self.possible_tile_set(pos, tile, false);
                    self.tile_set.probabilty_weight(tile)
                };   
            }

            let dist = WeightedIndex::new(&probability_weights).unwrap();

            let tile = dist.sample(&mut rng) as TileType;
            self.possible_tile_set(pos, tile, true);

            
            for dir in Dir::DIRECTIONS {
                let to_update = match (&self).coordinate_system.move_direction(pos, *dir) {
                    Some(val) => val,
                    None => continue
                };

                if update_queue_contains[to_update] {continue;}
                
                update_queue.push(to_update);
                update_queue_contains.set(to_update, true);
            }

            while let Some(to_update) = update_queue.pop() {
                update_queue_contains.set(to_update, false);

                match self.update(to_update) {
                    Ok(true) => (),
                    Ok(false) => continue,
                    _ => return Err(TilingError::Failed),
                };

                for dir in Dir::DIRECTIONS {
                    let to_update = match (&self).coordinate_system.move_direction(to_update, *dir) {
                        Some(val) => val,
                        None => continue
                    };
                    
                    if update_queue_contains[to_update] {continue;}

                    update_queue.push(to_update);
                    update_queue_contains.set(to_update, true);
                }
            }
        }
        
        Ok(())
    }

    fn tiling(&self, undetermined : TileType, impossible : TileType) -> Vec<TileType> {
        let mut grid : Vec<TileType> = Vec::with_capacity(self.coordinate_system.len());

        for i in 0..self.coordinate_system.len() {
            let mut tile = undetermined;

            for possible_tile in 0..self.tile_set.len() {
                if !self.possible_tile(i, possible_tile) {continue;}

                if tile == undetermined {tile = possible_tile; continue;}
                
                tile = impossible;
                break;
            }

            grid.push(tile);
        }

        grid
    }
}


impl <CS, TS, Dir> TilerPossibilities<CS, TS, Dir> where TS : TileSet<Dir>, CS : CoordinateSystem<Dir>, Dir : Directions { //TODO fix this.
    pub fn new(coordinate_system : CS, tile_set : TS) -> Self {
        let queue : KeyedPriorityQueue<usize, u8, BuildNoHashHasher<usize>> = KeyedPriorityQueue::from_iter((0..coordinate_system.len()).zip(repeat(0))); 
        let possible_tiles = bitvec![1; coordinate_system.len() * tile_set.len() as usize];
        
        TilerPossibilities { possible_tiles, queue, coordinate_system, tile_set, directions : Default::default()}
    }

    pub fn possible_tile(&self, pos : usize, tile : TileType) -> bool {
        self.possible_tiles[pos * self.tile_set.len() as usize + tile as usize]
    }

    pub fn possible_tile_set(&mut self, pos : usize, tile : TileType, value : bool) {
        self.possible_tiles.set(self.tile_set.len() as usize * pos + tile as usize, value)
    }

    pub fn allowed_tile(&self, pos_x : usize, tile_x : TileType, dir : Dir) -> bool {
        let pos_y = match self.coordinate_system.move_direction(pos_x, dir) {
            Some(val) => val,
            None => return true,
        };
        
        for tile_y in 0..self.tile_set.len() {
            if self.possible_tile(pos_y, tile_y) && self.tile_set.allows(tile_x, tile_y, dir) {return true}
        }

        false
    }

    pub fn update(&mut self, pos : usize) -> Result<bool, TilingError> {
        let mut changed = false;
        let mut disallowed = 0u8;


        for tile in 0..self.tile_set.len() {
            if !self.possible_tile(pos, tile) {disallowed += 1; continue;}
        
            for dir in Dir::DIRECTIONS {
                if self.allowed_tile(pos, tile, *dir) {continue;} 
                self.possible_tile_set(pos, tile, false);
                disallowed += 1;
                changed = true;

                break;
            }
        }
        if !changed{return Ok(false);}

        if disallowed == self.tile_set.len() {return Err(TilingError::Failed)}

        self.queue.set_priority(&pos, disallowed).unwrap(); 

        Ok(true)
    }
}
