//! A library providing searches in state space.
#![warn(
    missing_docs,
    rust_2018_idioms,
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo
)]

use rand::{thread_rng, Rng};
use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::Hash;

use priority_queue::MinPrioriyQueue;

mod priority_queue;

/// A state space which can be searched.
pub trait StateSpace {
    /// The type of the states.
    type State: Clone + Eq + Hash;

    /// Returns the state's neighbours.
    fn neighbours(&self, state: &Self::State) -> Vec<Self::State>;

    /// A callback used to display the progress of the search algorithm.
    /// It can be used to get a nice animation ;)
    #[inline]
    fn display_progress(&self, _init: &Self::State, _goal: &Self::State, _open: &[Self::State]) {}

    /// A search expanding nodes randomly.
    ///
    /// # Properties
    /// - Complete: No
    /// - Optimal: No
    /// - Time complexity: O(b^d)
    /// - Space complexity: O(b^d)
    fn random_search(
        &self,
        init: Self::State,
        goal: Self::State,
    ) -> Vec<Self::State> {
        let mut rng = thread_rng();
        let mut open = Vec::new();
        let mut closed = HashSet::new();
        let mut parent_of_state = HashMap::new();

        open.push(init.clone());

        while !open.is_empty() {
            let current = open.remove(rng.gen_range(0..open.len()));

            if current == goal {
                return reconstruct_path(parent_of_state, current);
            }

            for neighbour in self.neighbours(&current).into_iter() {
                if neighbour != current
                    && !open.contains(&neighbour)
                    && !closed.contains(&neighbour)
                {
                    open.push(neighbour.clone());
                    parent_of_state.insert(neighbour, current.clone());
                }
            }

            closed.insert(current);

            self.display_progress(&init, &goal, &open);
        }

        unreachable!()
    }

    /// A search expanding nodes level by level.
    ///
    /// # Properties
    /// - Complete: Yes
    /// - Optimal: Yes
    /// - Time complexity: O(b^d)
    /// - Space complexity: O(b^d)
    fn breadth_first_search(
        &self,
        init: Self::State,
        goal: Self::State,
    ) -> Vec<Self::State> {
        let mut open = VecDeque::new();
        let mut closed = HashSet::new();
        let mut parent_of_state = HashMap::new();

        open.push_back(init.clone());

        while !open.is_empty() {
            let current = open.pop_front().unwrap();

            if current == goal {
                return reconstruct_path(parent_of_state, current);
            }

            for neighbour in self.neighbours(&current) {
                if neighbour != current
                    && !open.contains(&neighbour)
                    && !closed.contains(&neighbour)
                {
                    open.push_back(neighbour.clone());
                    parent_of_state.insert(neighbour, current.clone());
                }
            }

            closed.insert(current);

            self.display_progress(&init, &goal, open.make_contiguous());
        }

        unreachable!()
    }

    /// A search expanding a path as far as possible.
    ///
    /// # Properties
    /// - Complete: No
    /// - Optimal: No
    /// - Time complexity: O(b^d)
    /// - Space complexity: O(bm)
    fn depth_first_search(
        &self,
        init: Self::State,
        goal: Self::State,
    ) -> Vec<Self::State> {
        let mut open = Vec::new();
        let mut closed = HashSet::new();
        let mut parent_of_state = HashMap::new();

        open.push(init.clone());

        while !open.is_empty() {
            let current = open.pop().unwrap();

            if current == goal {
                return reconstruct_path(parent_of_state, current);
            }

            for neighbour in self.neighbours(&current) {
                if neighbour != current
                    && !open.contains(&neighbour)
                    && !closed.contains(&neighbour)
                {
                    open.push(neighbour.clone());
                    parent_of_state.insert(neighbour, current.clone());
                }
            }

            closed.insert(current);

            self.display_progress(&init, &goal, &open);
        }

        unreachable!()
    }
}

fn reconstruct_path<S: Eq + Hash>(mut parent_of_state: HashMap<S, S>, goal: S) -> Vec<S> {
    let mut path = Vec::new();

    let mut current_state = goal;
    while let Some(next_state) = parent_of_state.remove(&current_state) {
        path.push(current_state);
        current_state = next_state;
    }
    path.push(current_state);

    path.reverse();
    path
}

/// A state space with a cost function.
pub trait CostStateSpace: StateSpace {
    /// Returns the cost of the transition.
    fn cost(&self, current: &Self::State, next: &Self::State) -> f32;

    /// A search expanding nodes with minimum costs.
    ///
    /// # Properties
    /// - Complete: Yes
    /// - Optimal: Yes
    /// - Time complexity: O(b^d)
    /// - Space complexity: O(b^d)
    fn dijkstra(
        &self,
        init: Self::State,
        goal: Self::State,
    ) -> Vec<Self::State> {
        let mut queue = MinPrioriyQueue::new();
        let mut distances = HashMap::new();
        let mut parent_of_state = HashMap::new();
        let mut closed = HashSet::new();

        queue.enqueue(0., init.clone());
        distances.insert(init.clone(), 0.);
        while !queue.is_empty() {
            let current = queue.dequeue().unwrap();

            if current == goal {
                return reconstruct_path(parent_of_state, goal);
            }

            let current_dist = *distances.get(&current).unwrap();
            for neighbour in self.neighbours(&current) {
                let d = current_dist + self.cost(&current, &neighbour);

                if !distances.contains_key(&neighbour) {
                    distances.insert(neighbour.clone(), d);
                    queue.enqueue(d, neighbour.clone());
                    parent_of_state.insert(neighbour, current.clone());
                } else {
                    let dp = distances.get_mut(&neighbour).unwrap();
                    if d < *dp {
                        *dp = d;
                        queue.enqueue(d, neighbour.clone());
                        parent_of_state.insert(neighbour, current.clone());
                    }
                }
            }

            closed.insert(current);
            self.display_progress(&init, &goal, &Vec::from(queue.clone()));
        }

        unreachable!()
    }
}

/// A state space with a cost and heuristic function.
pub trait HeuristicStateSpace: CostStateSpace {
    /// Returns a lower bound estimation of the least cost path to the closest
    /// goal state.
    fn heuristic(&self, state: &Self::State, goal: &Self::State) -> f32;

    /// A search expanding nodes with minimum heuristic.
    ///
    /// # Properties
    /// - Complete: No
    /// - Optimal: No
    /// - Time complexity: O(b^d)
    /// - Space complexity: O(b^d)
    fn greedy_search(
        &self,
        init: Self::State,
        goal: Self::State,
    ) -> Vec<Self::State> {
        let mut open = MinPrioriyQueue::new();
        let mut closed = HashSet::new();
        let mut parent_of_state = HashMap::new();

        open.enqueue(self.heuristic(&init, &goal), init.clone());

        while !open.is_empty() {
            let current = open.dequeue().unwrap();

            if current == goal {
                return reconstruct_path(parent_of_state, current);
            }

            for neighbour in self.neighbours(&current) {
                if neighbour != current
                    && !open.contains(&neighbour)
                    && !closed.contains(&neighbour)
                {
                    open.enqueue(self.heuristic(&neighbour, &goal), neighbour.clone());
                    parent_of_state.insert(neighbour, current.clone());
                }
            }

            closed.insert(current);

            self.display_progress(&init, &goal, &Vec::from(open.clone()));
        }

        unreachable!()
    }

    /// A search expanding nodes with minimum *cost + heuristic*.
    ///
    /// # Properties
    /// - Complete: Yes
    /// - Optimal: Yes (if the heuristic is *optimistic*)
    /// - Time complexity: O(min(b^(d+1), b|S|))
    /// - Space complexity: O(min(b^(d+1), b|S|))
    fn a_star(
        &self,
        init: Self::State,
        goal: Self::State,
    ) -> Vec<Self::State> {
        let mut open = MinPrioriyQueue::new();
        let mut distances = HashMap::new();
        let mut parent_of_state = HashMap::new();
        let mut closed = HashSet::new();

        open.enqueue(self.heuristic(&init, &goal), init.clone());
        distances.insert(init.clone(), 0.);

        while !open.is_empty() {
            let current = open.dequeue().unwrap();

            if current == goal {
                return reconstruct_path(parent_of_state, goal);
            }

            let current_dist = *distances.get(&current).unwrap();
            for neighbour in self
                .neighbours(&current)
                .into_iter()
                .filter(|n| !closed.contains(n))
            {
                let neighbour_dist = current_dist + self.cost(&current, &neighbour);

                if !open.contains(&neighbour)
                    || !distances.contains_key(&neighbour)
                    || *distances.get(&neighbour).unwrap() > neighbour_dist
                {
                    distances.insert(neighbour.clone(), neighbour_dist);
                    parent_of_state.insert(neighbour.clone(), current.clone());

                    open.enqueue(
                        neighbour_dist + self.heuristic(&neighbour, &goal),
                        neighbour,
                    );
                }
            }

            closed.insert(current);
            self.display_progress(&init, &goal, &Vec::from(open.clone()));
        }

        unreachable!()
    }
}
