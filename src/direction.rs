#[derive(Copy, Clone, Debug)]
pub enum AbsolouteDirection{
    North = 0,
    East = 1,
    South = 2,
    West = 3,
    Up = 4, // if pictured on a map, points towards the viewer of the map
    Down = 5 // if pictured on a map, points away from the viewer
}

impl Into<i32> for AbsolouteDirection{
    fn into(self) -> i32 {
        match self{
            Self::North => 0,
            Self::East => 1,
            Self::South => 2,
            Self::West => 3,
            Self::Up => 4,
            Self::Down => 5
        }
    }
}