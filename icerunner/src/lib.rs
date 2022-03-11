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
    pub fn is_ice(&self) -> bool {
        matches!(self, Object::ICE)
    }
    pub fn is_obstacle(&self) -> bool {
        matches!(self, Object::WALL)
    }
    pub fn is_start(&self) -> bool {
        matches!(self, Object::START)
    }
    pub fn is_end(&self) -> bool {
        matches!(self, Object::END)
    }
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
        end_point: Pos,
    }
    impl IceRunner {
        pub(super) fn new() -> Self {
            IceRunner {
                maze: [[None; 5]; 5],
                end_point: Pos::new(0, 0),
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
        pub(super) fn end_point_mut(&mut self) -> &mut Pos {
            &mut self.end_point
        }
        pub(super) fn end_point(&self) -> Pos {
            self.end_point
        }
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
        let mut icemaze = IceRunner::new();
        let mut cs = s.chars();
        let mut start = false;
        let mut end = false;
        for y in 0..5 {
            for x in 0..5 {
                let pos = Pos::new(x, y);
                let mut put_obj = |icemaze: &mut IceRunner, obj| {
                    if obj == Object::START {
                        if start {
                            println!("only one start point allowed");
                            return Err(IceMazeParseError);
                        }
                        start = true;
                    }
                    if obj == Object::END {
                        if end {
                            println!("only one end point allowed");
                            return Err(IceMazeParseError);
                        }
                        end = true;
                        *icemaze.end_point_mut() = pos;
                    }
                    *icemaze.get_mut(pos) = Some(obj);
                    Ok(())
                };
                match cs.next() {
                    None => return Err(IceMazeParseError),
                    Some(c) => match c {
                        '.' => put_obj(&mut icemaze, Object::ICE)?,
                        '*' => put_obj(&mut icemaze, Object::WALL)?,
                        'S' => put_obj(&mut icemaze, Object::START)?,
                        'E' => put_obj(&mut icemaze, Object::END)?,
                        _ => return Err(IceMazeParseError),
                    },
                }
            }
            match cs.next() {
                Some('\n') => (),
                _ => {
                    return Err(IceMazeParseError);
                }
            }
        }
        if start == false {
            return Err(IceMazeParseError);
        }
        if end == false {
            return Err(IceMazeParseError);
        }
        if cs.next().is_some() {
            return Err(IceMazeParseError);
        }

        Ok(icemaze)
    }
}

impl Puzzle for IceRunner {
    type Move = (Object, Direction);

    fn is_goal(&self) -> bool {
        // iterate through all positions
        for pos in Pos::values() {
            if let Some(obj) = self.get(pos) {
                if obj.is_start() && pos == self.end_point() {
                    return true;
                }
            }
        }
        false
    }

    fn next(&self) -> Vec<(Self::Move, Self)> {
        let mut next = Vec::new();

        // iterate through all positions
        for pos in Pos::values() {
            // iterate through all directions
            for dir in Direction::values() {
                if let Some((obj, icerunner)) = self.move_yourself(pos, dir) {
                    println!("obj {} pos {} dir {} ", obj, pos, dir);
                    println!("{}", icerunner);
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
        let self_copy = &mut self.clone();
        let obj_p = match self.get(pos) {
            Some(o) => o,
            None => return None,
        };
        // check if object is a rabbit object only then proceed
        if !obj_p.is_start() {
            return None;
        }

        let mut p = pos;
        p = match get_next_loc(self, p, dir) {
            Some(p) => p,
            None => return None,
        };
        if p == pos {
            return None;
        }
        *self_copy.get_mut(p) = Some(*obj_p);
        *self_copy.get_mut(pos) = Some(Object::ICE);
        return Some((*obj_p, *self_copy));
    }
}
fn get_next_loc(board: &IceRunner, mut p: Pos, dir: Direction) -> Option<Pos> {
    loop {
        match p.step(dir) {
            Some(new_pos) => match board.get(new_pos) {
                Some(ob) => {
                    println!("pos {:?} dir {}", new_pos, dir);

                    if !ob.is_obstacle() {
                        p = new_pos;
                    } else {
                        break;
                    }
                }
                None => return None,
            },
            None => {
                return Some(p);
            }
        }
    }
    Some(p)
}

// #[cfg(test)]
// mod tests;
