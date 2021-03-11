use state_space::{CostStateSpace, HeuristicStateSpace, StateSpace};

fn main() {
    let maze = Maze::from_string(
        r#"XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX
X   X                     X                                           X
X X    XXXX X    XX X  X  XXXXX    X  X    X XX XXX X   XX        XXX X
X   X     X   X X       X               X                       X     X
X X X X X   X X X XXXXX  XXX    X X XX   X XXXX X XX   XXX X    X X   X
X   X     X X X X           X X X X X                 X       X X X   X
X X X   X X        XX  XXXX X   X    XX          XX X   XXXXX   X X X X
X       X     X X           X     X       X       X   X X   X X X     X
X  X  XXXX X    X       X X XXXX XX X     XXX X   X X X X X X X X X X X
X               X                             X   X       X     X     X
X X X  XXX  X X XXX  XXXXX XX   XXX X  X XX   XXX     XX X XX X  X  XXX
X                   X             X X       X   X X                   X
X XXX X   X   X   X   XXXXXX    X X XXX   XXX   X XXX X  XX     X X X X
X   X     X                 X X X   X                 X               X
X X   X   X X XXX X  XXX  X XX    X X X X  XX XX    X X   XX  X   XXX X
X       X X         X   X                           X X       X       X
X X   X X   XXX X       X  X X    X XXX   X XXXXXX XXXX   XXXXX XX  X X
X X X   X       X   X X   X     X         X   X         X           X X
X     XX  X   X X     XXXX XXX  X X X XX XXXX X XX  XXX  XXX X XXXX X X
X   X           X     X           X           X X                     X
X    XX    XX X   X X X  X   XX X X XXXXX   X    XX XX XX X X X   X   X
X X   X   X     X                 X         X           X X           X
X XX  X X XXX X   X X XXX X   XXXXXXX X X    XX X  XXX  X     XXX X XXX
X       X                 X               X       X   X           X   X
X XX X  X X   XX  X XXXX XX XXXXX XXXXX X   X     X X X     XXX X  X  X
X   X   X     X   X   X   X           X     X X X X         X     X   X
X X  X    XX   XX         X XXXX X    X XXX   X X X  XXXX X X  XXXX  XX
X X       X     X                           X     X     X   X X       X
X  X  X X X X X   X X XXXXX XX   XX XXXX  X XXX X X XXXXXX XX  XX XX  X
X             X X         X         X   X X     X             X       X
X X  X  XXX  X  X X   XXX X XXX X X X X X    XX   X X  X    X X   XXX X
X   X   X     X     X         X       X   X             X X   X   X   X
XXX X   X XXX   XX  X X XX  X X     XXX X   X XXXXX  XXX  XX  X X X XXX
X       X   X         X     X     X X   X X X X                       X
XXX X   X X   XXX  XX XX  X  XXXX  X     XX    XX  XXX    X X X X   X X
X           X         X   X     X           X     X     X   X         X
X XXX  XXX XX  XXX    X XXX       X X XXX    X XX   XX   XX X X   XX  X
X                             X                                 X     X
X  X    XXXX   XX X X XXX X     XXX   X X XXX      X  XX   XX  X     XX
X     X         X X         X X               X             X         X
X XXX X X X  XX     X  X    X  XX  X  XX    X X X X X X  X  X    XXXX X
X   X X   X       X             X   X           X   X   X X         X X
X XXX X XXX XXXXXXX  XXX XXX      X   X XX   XXXX X X XXX XXXXX X X   X
X         X         X         X       X     X     X X           X     X
XXX X X X XXX X X XX X  X XXX  XXX  X   X  XX     X X X X XX X X  XX  X
X   X X   X       X               X       X   X     X       X       X X
X X  XX X X X   X X  X  XX     X  XXXXXXXXXXX XXX XXX  XXX  X  XXXX   X
X   X   X X X     X               X       X       X   X         X     X
XXX   XXX X X X XX XXXXX X XX   X   XX  X   X  XX X XXX       X X XX XX
X     X     X                                   X                     X
XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX"#
            .into(),
    );

    let initial = (1, 43);
    let goal = (21, 47);
    let path = maze.a_star(initial, goal);

    println!("{}", maze.draw_maze_path(&path, &[]));
}

#[derive(Debug)]
pub struct Maze {
    width: usize,
    height: usize,
    cells: Vec<CellState>,
}

impl Maze {
    #[inline]
    #[must_use]
    pub fn new(width: usize, height: usize, cells: Vec<CellState>) -> Self {
        assert!(width > 0 && height > 0);

        Self {
            width,
            height,
            cells,
        }
    }

    #[must_use]
    pub fn from_string(serialized_maze: String) -> Maze {
        let width = serialized_maze
            .split('\n')
            .next()
            .expect("Input maze should have at least one line")
            .len();

        let height = serialized_maze.split('\n').count();

        let cells: Vec<CellState> = serialized_maze
            .split('\n')
            .map(|line| {
                line.chars().map(|chr| {
                    if chr == 'X' {
                        CellState::Wall
                    } else {
                        CellState::Ground
                    }
                })
            })
            .flatten()
            .collect();

        Self::new(width, height, cells)
    }

    #[inline]
    #[must_use]
    pub fn get(&self, x: usize, y: usize) -> Option<&CellState> {
        self.cells.get(x + y * self.width)
    }

    pub fn draw_maze_path(&self, path: &[(usize, usize)], open: &[(usize, usize)]) -> String {
        (0..self.height)
            .map(|y| {
                let mut line = (0..self.width)
                    .map(|x| match self.get(x, y) {
                        Some(CellState::Wall) => "X",
                        _ if path.get(0) == Some(&(x, y)) => "S",
                        _ if path.last() == Some(&(x, y)) => "E",
                        _ if path.contains(&(x, y)) => "\x1b[31mo\x1b[0m",
                        _ if open.contains(&(x, y)) => "\x1b[32m#\x1b[0m",
                        _ => " ",
                    })
                    .collect::<String>();
                line.push('\n');
                line
            })
            .collect::<String>()
    }
}

impl StateSpace for Maze {
    type State = (usize, usize);

    fn neighbours(&self, state: &Self::State) -> Vec<Self::State> {
        let offsets: Vec<(isize, isize)> = vec![(0, -1), (1, 0), (0, 1), (-1, 0)];

        offsets
            .into_iter()
            .filter_map(|(x, y)| {
                let x = checked_add_signed(state.0, x)?;
                let y = checked_add_signed(state.1, y)?;

                if let Some(CellState::Ground) = self.get(x, y) {
                    Some((x, y))
                } else {
                    None
                }
            })
            .collect()
    }

    #[inline]
    fn display_progress(&self, init: &Self::State, goal: &Self::State, open: &[Self::State]) {
        println!("{}", self.draw_maze_path(&[*init, *goal], open));
    }
}

impl CostStateSpace for Maze {
    #[inline]
    fn cost(&self, _current: &Self::State, _next: &Self::State) -> f32 {
        1.
    }
}

impl HeuristicStateSpace for Maze {
    #[inline]
    fn heuristic(&self, state: &Self::State, goal: &Self::State) -> f32 {
        let x_dist = if state.0 > goal.0 {
            state.0 - goal.0
        } else {
            goal.0 - state.0
        };

        let y_dist = if state.1 > goal.1 {
            state.1 - goal.1
        } else {
            goal.1 - state.1
        };

        (x_dist + y_dist) as f32
    }
}

#[derive(Debug)]
#[repr(u8)]
pub enum CellState {
    Wall,
    Ground,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn neighbours_middle() {
        let maze = Maze::new(
            3,
            3,
            vec![
                CellState::Ground,
                CellState::Ground,
                CellState::Ground,
                CellState::Ground,
                CellState::Ground,
                CellState::Ground,
                CellState::Ground,
                CellState::Ground,
                CellState::Ground,
            ],
        );

        assert_eq!(
            maze.neighbours(&(1, 1)),
            vec![(1, 1 - 1), (1 + 1, 1), (1, 1 + 1), (1 - 1, 1),]
        );
    }

    #[test]
    fn neighbours_maze_corner() {
        let maze = Maze::new(
            3,
            3,
            vec![
                CellState::Ground,
                CellState::Ground,
                CellState::Ground,
                CellState::Ground,
                CellState::Ground,
                CellState::Ground,
                CellState::Ground,
                CellState::Ground,
                CellState::Ground,
            ],
        );

        assert_eq!(maze.neighbours(&(0, 0)), vec![(0 + 1, 0), (0, 0 + 1),]);
    }

    #[test]
    fn neighbours_wall_corner() {
        let maze = Maze::new(
            3,
            3,
            vec![
                CellState::Wall,
                CellState::Wall,
                CellState::Wall,
                CellState::Wall,
                CellState::Ground,
                CellState::Ground,
                CellState::Wall,
                CellState::Ground,
                CellState::Ground,
            ],
        );

        assert_eq!(maze.neighbours(&(1, 1)), vec![(1 + 1, 1), (1, 1 + 1),]);
    }
}

#[inline]
pub fn checked_add_signed(lhs: usize, rhs: isize) -> Option<usize> {
    if rhs > 0 {
        lhs.checked_add(rhs.abs() as usize)
    } else {
        lhs.checked_sub(rhs.abs() as usize)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn checked_add_signed_no_overflow() {
        assert_eq!(checked_add_signed(1, 1), Some(2));
    }

    #[test]
    fn checked_add_signed_overflow_negative() {
        assert_eq!(checked_add_signed(1, -2), None);
    }

    #[test]
    fn checked_add_signed_overflow_positive() {
        assert_eq!(
            checked_add_signed(std::isize::MAX as usize + 2, std::isize::MAX),
            None
        );
    }
}
