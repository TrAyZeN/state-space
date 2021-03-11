use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Clone)]
pub struct MinPrioriyQueue<T> {
    heap: BinaryHeap<InvertedPriority<T>>,
}

impl<T: Eq> MinPrioriyQueue<T> {
    /// Creates a new empty `MinPrioriyQueue<P, E>`.
    #[inline]
    pub fn new() -> Self {
        Self {
            heap: BinaryHeap::new(),
        }
    }

    #[inline]
    pub fn enqueue(&mut self, priority: f32, element: T) {
        self.heap
            .push(InvertedPriority::new(priority, element).expect("Priority should not be NaN"));
    }

    #[inline]
    pub fn dequeue(&mut self) -> Option<T> {
        self.heap.pop().map(|e| e.element)
    }

    #[inline]
    pub fn contains(&self, element: &T) -> bool {
        self.heap.iter().any(|e| &e.element == element)
    }
}

impl<T> MinPrioriyQueue<T> {
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }
}

impl<T> From<MinPrioriyQueue<T>> for Vec<T> {
    #[inline]
    fn from(queue: MinPrioriyQueue<T>) -> Self {
        queue.heap.into_iter().map(|e| e.element).collect()
    }
}

impl<T: Eq> Default for MinPrioriyQueue<T> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(transparent)]
struct NotNan(f32);

impl NotNan {
    // `is_nan` is not const yet
    #[allow(clippy::missing_const_for_fn)]
    #[inline]
    fn new(value: f32) -> Option<Self> {
        match value {
            value if value.is_nan() => None,
            value => Some(Self(value)),
        }
    }
}

impl Eq for NotNan {}

impl Ord for NotNan {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).expect("Value should not be NaN")
    }
}

impl PartialOrd for NotNan {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

#[derive(Clone, Eq, PartialEq)]
struct InvertedPriority<T> {
    priority: NotNan,
    element: T,
}

impl<T> InvertedPriority<T> {
    #[inline]
    fn new(priority: f32, element: T) -> Option<Self> {
        let priority = NotNan::new(priority)?;

        Some(Self { priority, element })
    }
}

impl<T: Eq> Ord for InvertedPriority<T> {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        other.priority.cmp(&self.priority)
    }
}

impl<T: PartialEq> PartialOrd for InvertedPriority<T> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.priority.partial_cmp(&self.priority)
    }
}
