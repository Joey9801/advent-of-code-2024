use super::{Dir, Vec2};

pub trait Map2dExt<Tile> {
    fn size(&self) -> Vec2;
    fn get(&self, pos: Vec2) -> Option<Tile>
    where
        Tile: Copy;
    fn get_mut(&mut self, pos: Vec2) -> Option<&mut Tile>;

    fn debug_print(&self, f: impl Fn(Tile) -> char)
    where
        Tile: Copy,
    {
        for y in 0..self.size().y {
            for x in 0..self.size().x {
                let pos = Vec2::new(x, y);
                let tile = self.get(pos).unwrap();
                print!("{}", f(tile));
            }
            println!();
        }
    }
}

#[derive(Clone)]
pub struct Map2d<Tile> {
    pub size: Vec2,
    pub data: Vec<Tile>,
}

impl<Tile> Map2d<Tile> {
    pub fn new_default(size: Vec2, default: Tile) -> Self
    where
        Tile: Clone,
    {
        let data = vec![default; (size.x * size.y) as usize];
        Self { size, data }
    }

    pub fn parse_grid(s: &str, f: impl Fn(char) -> Tile) -> Self {
        let size_x = s.lines().next().unwrap().len();
        let size_y = s.lines().count();
        let size = Vec2::new(size_x as i64, size_y as i64);

        let data = s.chars().filter(|&c| c != '\n').map(f).collect::<Vec<_>>();

        Self { size, data }
    }

    pub fn index_of(&self, pos: Vec2) -> Option<usize> {
        if pos.x < 0 || pos.y < 0 || pos.x >= self.size.x || pos.y >= self.size.y {
            None
        } else {
            Some((pos.x + pos.y * self.size.x) as usize)
        }
    }

    pub fn pos_of(&self, index: usize) -> Vec2 {
        let x = index as i64 % self.size.x;
        let y = index as i64 / self.size.x;
        Vec2::new(x, y)
    }

    pub fn get_row(&self, y: i64) -> &[Tile] {
        let start = self.index_of(Vec2::new(0, y)).unwrap();
        let end = self.index_of(Vec2::new(self.size.x - 1, y)).unwrap();
        &self.data[start..=end]
    }

    pub fn find(&self, predicate: impl Fn(&Tile) -> bool) -> Option<Vec2> {
        self.data.iter().position(predicate).map(|i| self.pos_of(i))
    }
}

impl<Tile> Map2dExt<Tile> for Map2d<Tile> {
    fn size(&self) -> Vec2 {
        self.size
    }

    fn get(&self, pos: Vec2) -> Option<Tile>
    where
        Tile: Copy,
    {
        self.index_of(pos).map(|i| self.data[i])
    }

    fn get_mut(&mut self, pos: Vec2) -> Option<&mut Tile> {
        self.index_of(pos).map(move |i| &mut self.data[i])
    }
}

impl<Tile> AsRef<Map2d<Tile>> for Map2d<Tile> {
    fn as_ref(&self) -> &Map2d<Tile> {
        self
    }
}

/// A view of the original map, rotated about the center of the map
pub struct RotatedMap2d<'a, Tile> {
    pub map: &'a mut Map2d<Tile>,

    /// The original 'up' direction of the source map in this rotated view
    pub up: Dir,
}

impl<'a, Tile> RotatedMap2d<'a, Tile> {
    fn source_pos(&self, pos: Vec2) -> Vec2 {
        match self.up {
            Dir::Up => pos,
            Dir::Down => Vec2::new(self.map.size.x - pos.x - 1, self.map.size.y - pos.y - 1),
            Dir::Left => Vec2::new(self.map.size.x - pos.y - 1, pos.x),
            Dir::Right => Vec2::new(pos.y, self.map.size.y - pos.x - 1),
        }
    }
}

impl<'a, Tile> Map2dExt<Tile> for RotatedMap2d<'a, Tile> {
    fn size(&self) -> Vec2 {
        match self.up {
            Dir::Up | Dir::Down => self.map.size,
            Dir::Left | Dir::Right => Vec2::new(self.map.size.y, self.map.size.x),
        }
    }

    fn get(&self, pos: Vec2) -> Option<Tile>
    where
        Tile: Copy,
    {
        self.map.get(self.source_pos(pos))
    }

    fn get_mut(&mut self, pos: Vec2) -> Option<&mut Tile> {
        self.map.get_mut(self.source_pos(pos))
    }
}
