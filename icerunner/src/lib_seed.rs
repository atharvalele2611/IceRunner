use std::fmt;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use puzzle::Puzzle;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Object {
    ICE,
    WALL,
    START,
    END,
}
impl Object {
    // need additional methods
}
impl Display for Object {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str(match self {
            Object::ICE => ".",
            Object::WALL => "*",
            Object::START => "S",
            Object::END => "E",
        })
    }
}

mod pos {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
    pub struct Pos {
        x: usize,
        y: usize,
    }
    impl Pos {
        pub(super) fn new(x: usize, y: usize) -> Self {
            if x >= 5 {
                panic!("Pos::new x (is {}) should be less than 5", x)
            }
            if y >= 5 {
                panic!("Pos::new y (is {}) should be less than 5", y)
            }
            Pos { x, y }
        }
        pub(super) fn xy(&self) -> (usize, usize) {
            (self.x, self.y)
        }
    }
}
use self::pos::Pos;

impl Pos {
    fn at_edge(&self) -> bool {
        let (x, y) = self.xy();
        y == 0 || y == 4
    }
    fn is_raised(&self) -> bool {
        let (x, y) = self.xy();
        self.at_edge() || ((x == 0 || x == 4) && y == 2) || (x == 2 && (y == 0 || y == 4))
    }
    /// Return `Some(pos)` if `pos` is the position on the gameboard that is one
    /// step from `self` in the direction `dir`.  Returns `None` if there is no
    /// position on the gameboard that is one step from `self` in the direction
    /// `dir` (i.e., would move off the edge of the gameboard).
    fn step(&self, dir: Direction) -> Option<Self> {
        let (x, y) = self.xy();
        let (x, y) = match dir {
            Direction::North => {
                if y == 0 {
                    return None;
                }
                (x, y - 1)
            }
            Direction::South => {
                if y == 4 {
                    return None;
                }
                (x, y + 1)
            }
            Direction::West => {
                if x == 0 {
                    return None;
                }
                (x - 1, y)
            }
            Direction::East => {
                if x == 4 {
                    return None;
                }
                (x + 1, y)
            }
        };
        Some(Pos::new(x, y))
    }
    /// An iterator over all positions of the gameboard.
    pub fn values() -> impl Iterator<Item = Self> {
        (0..5).flat_map(|y| (0..5).map(move |x| Pos::new(x, y)))
    }
}
impl Display for Pos {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let (x, y) = self.xy();
        write!(f, "({},{})", x, y)
    }
}

/// The `Direction` type represents the cardinal directions, in which objects
/// may be moved on the gameboard.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Direction {
    North,
    South,
    West,
    East,
}
impl Direction {
    fn rev(&self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
            Direction::East => Direction::West,
        }
    }
    /// An iterator over all directions.
    fn values() -> impl Iterator<Item = Self> {
        [
            Direction::North,
            Direction::South,
            Direction::West,
            Direction::East,
        ]
        .into_iter()
    }
}
impl Display for Direction {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str(match self {
            Direction::North => "↑",
            Direction::South => "↓",
            Direction::West => "←",
            Direction::East => "→",
        })
    }
}

mod icerunner {
    use super::Object;
    use super::Pos;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct IceRunner {
        maze: [[Option<Object>; 5]; 5],
        // new fields here
    }
    impl IceRunner {
        pub(super) fn new() -> Self {
            IceRunner {
                maze: [[None; 5]; 5],
                // add new fields here.
            }
        }
        /// Returns a reference to the gameboard at position `pos`.
        pub(super) fn get(&self, pos: Pos) -> &Option<Object> {
            let (x, y) = pos.xy();
            &self.maze[x][y]
        }
        /// Returns a mutable reference to the gameboard at position `pos`.
        pub(super) fn get_mut(&mut self, pos: Pos) -> &mut Option<Object> {
            let (x, y) = pos.xy();
            &mut self.maze[x][y]
        }
        // may need additional methods
    }
}
pub use self::icerunner::IceRunner;

impl Display for IceRunner {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        for pos in Pos::values() {
            match self.get(pos) {
                None => {}
                Some(obj) => obj.fmt(f)?,
            }
            if pos.xy().0 == 4 {
                f.write_str("\n")?
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IceMazeParseError;
impl FromStr for IceRunner {
    type Err = IceMazeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        unimplemented!()
    }
}

impl Puzzle for IceRunner {
    type Move = (Object, Direction);

    fn is_goal(&self) -> bool {
        // iterate through all positions
        unimplemented!()
    }

    fn next(&self) -> Vec<(Self::Move, Self)> {
        let mut next = Vec::new();

        // iterate through all positions
        for pos in Pos::values() {
            // iterate through all directions
            for dir in Direction::values() {
                if let Some((obj, icerunner)) = self.move_yourself(pos, dir) {
                    next.push(((obj, dir), icerunner))
                }
            }
        }
        next
    }
}

impl IceRunner {
    fn move_yourself(&self, pos: Pos, dir: Direction) -> Option<(Object, Self)> {
        // Your code here
        unimplemented!();
    }
}

// #[cfg(test)]
// mod tests;
