use state_space::StateSpace;

fn main() {
    let p = KnightMove::new((8, 8));
    let initial = (0, 0);
    let goal = (7, 7);

    println!(
        "Steps found from {:?} to {:?} using Random Search:\n{:?}\n",
        initial,
        goal,
        p.random_search(initial, goal)
    );
    println!(
        "Steps found from {:?} to {:?} using Breadth First Search:\n{:?}\n",
        initial,
        goal,
        p.breadth_first_search(initial, goal)
    );
    println!(
        "Steps found from {:?} to {:?} using Depth First Search:\n{:?}",
        initial,
        goal,
        p.depth_first_search(initial, goal)
    );
}

pub struct KnightMove {
    size: (isize, isize),
}

impl KnightMove {
    pub fn new(size: (isize, isize)) -> Self {
        Self { size }
    }

    pub fn is_in_board(&self, state: (isize, isize)) -> bool {
        state.0 >= 0 && state.1 >= 0 && state.0 < self.size.0 && state.1 < self.size.1
    }
}

impl StateSpace for KnightMove {
    type State = (isize, isize);

    fn neighbours(&self, state: &Self::State) -> Vec<Self::State> {
        let offsets = vec![
            (-1, -2),
            (1, -2),
            (2, -1),
            (2, 1),
            (1, 2),
            (-1, 2),
            (-2, 1),
            (-2, -1),
        ];

        offsets
            .into_iter()
            .map(|(i, j)| (state.0.checked_add(i), state.1.checked_add(j)))
            .filter_map(|(i, j)| {
                if let (Some(i), Some(j)) = (i, j) {
                    Some((i, j))
                } else {
                    None
                }
            })
            .filter(|s| self.is_in_board(*s))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn neighbours_middle() {
        let p = KnightMove::new((8, 8));
        assert_eq!(
            p.neighbours(&(4, 4)),
            vec![
                (4 - 1, 4 - 2),
                (4 + 1, 4 - 2),
                (4 + 2, 4 - 1),
                (4 + 2, 4 + 1),
                (4 + 1, 4 + 2),
                (4 - 1, 4 + 2),
                (4 - 2, 4 + 1),
                (4 - 2, 4 - 1),
            ]
        );
    }

    #[test]
    fn neighbours_side() {
        let p = KnightMove::new((8, 8));
        assert_eq!(
            p.neighbours(&(0, 4)),
            vec![
                (0 + 1, 4 - 2),
                (0 + 2, 4 - 1),
                (0 + 2, 4 + 1),
                (0 + 1, 4 + 2),
            ]
        );
    }

    #[test]
    fn neighbours_corner() {
        let p = KnightMove::new((8, 8));
        assert_eq!(p.neighbours(&(7, 7)), vec![(7 - 1, 7 - 2), (7 - 2, 7 - 1),]);
    }
}
