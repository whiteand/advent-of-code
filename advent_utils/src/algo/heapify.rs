use std::cmp::Ordering;

pub fn heap_pop<T: Ord>(heap: &mut Vec<T>) -> Option<T> {
    heap_pop_by(heap, Ord::cmp)
}

pub fn heap_pop_by<T>(heap: &mut Vec<T>, mut cmp: impl FnMut(&T, &T) -> Ordering) -> Option<T> {
    if heap.is_empty() {
        None
    } else {
        let n = heap.len() - 1;
        heap.swap(0, n);
        let x = heap.pop().expect("should be present");
        heap_lower_by(heap, &mut cmp, 0);
        Some(x)
    }
}

/// Transforms vector into max binary heap
pub fn heapify<T>(arr: &mut [T], mut cmp: impl FnMut(&T, &T) -> Ordering) {
    for i in 0..arr.len() {
        heap_raise_by(arr, &mut cmp, i)
    }
}
pub fn heapify_by<T>(arr: &mut [T], mut cmp: impl FnMut(&T, &T) -> Ordering) {
    for i in 0..arr.len() {
        heap_raise_by(arr, &mut cmp, i)
    }
}

/// Checks if parent is less than current index,
/// if yes, it swaps parent and child
pub fn heap_raise_by<T>(arr: &mut [T], mut cmp: impl FnMut(&T, &T) -> Ordering, index: usize) {
    let mut i = index;
    while let Some(parent_index) = get_parent(i) {
        let parent = &arr[parent_index];
        let ordering = cmp(parent, &arr[i]);
        if ordering.is_lt() {
            arr.swap(parent_index, i);
            i = parent_index;
        } else {
            return;
        }
    }
}

// Pushes the element down
pub fn heap_lower_by<T>(arr: &mut [T], cmp: &mut impl FnMut(&T, &T) -> Ordering, index: usize) {
    let mut i = index;
    while i < arr.len() {
        let left_child_index = get_left_child(i);
        let right_child_index = left_child_index + 1;
        let left_child = arr.get(left_child_index);
        let right_child = arr.get(right_child_index);
        match (left_child, right_child) {
            (None, None) => return,
            (None, _) => unreachable!(),
            (Some(c), None) => {
                if cmp(&arr[i], c).is_lt() {
                    arr.swap(i, left_child_index);
                }
                i = left_child_index;
            }
            (Some(left), Some(right)) => {
                let mut swap_with = i;
                if cmp(&arr[swap_with], left).is_lt() {
                    swap_with = left_child_index;
                }
                if cmp(&arr[swap_with], right).is_lt() {
                    swap_with = right_child_index;
                }
                if i == swap_with {
                    return;
                }
                arr.swap(i, swap_with);
                i = swap_with;
            }
        }
    }
}

pub const fn get_parent(index: usize) -> Option<usize> {
    if index > 0 {
        Some((index - 1) >> 1)
    } else {
        None
    }
}

pub const fn get_left_child(index: usize) -> usize {
    index * 2 + 1
}
