use std::{collections::VecDeque, cmp::Ordering};

// Returns an iterator over (peak value, peak width)
pub fn peaks_by<T: Clone, F>(width: usize, mut values: impl Iterator<Item = T>, mut f: F) -> Option<impl Iterator<Item = (T, usize)>>
where
    F: FnMut(&T, &T) -> Ordering
{
    let width = width * 2 + 1;
    let middle = width / 2;
    let mut queue = VecDeque::with_capacity(width);

    // Returns Some(value) if middle value is greater than or equal to all other values
    // value is how many values after middle value are the same as middle value
    let mut test_peak = move |queue: &VecDeque<T>| -> Option<usize> {
        let middle_value = queue.get(middle)?;
        let mut continuous = true; 
        let mut len = 0;

        for i in (0..middle).rev() {
            match f(middle_value, &queue[i]) {
                Ordering::Less => return None,
                Ordering::Equal => if !continuous {
                    return None
                }
                Ordering::Greater => continuous = false,
            }
        }

        continuous = true; 

        for i in (middle + 1)..queue.len() {
            match f(middle_value, &queue[i]) {
                Ordering::Equal => if continuous {
                    len += 1;
                }
                else {
                    return None;
                },
                Ordering::Less => return None,
                Ordering::Greater => continuous = false,
            }
        }

        Some(len)
    };

    for _ in 1..width {
        match values.next() {
            Some(value) => {
                queue.push_back(value)
            },
            None => return None
        }
    }

    let mut to_skip = 0;

    Some(values.map(move |item| {
        queue.push_back(item.clone());

        if to_skip > 0 {
            to_skip -= 1;
            queue.pop_front();
            return None; 
        }

        let result = test_peak(&queue).map(|in_row| {
            to_skip = in_row;

            (queue[middle].clone(), in_row + 1)
        });

        queue.pop_front();

        result
    }).filter(|value| value.is_some()).map(|value| value.unwrap()))
}

pub fn peaks<T: Clone + std::cmp::Ord>(width: usize, values: impl Iterator<Item = T>) -> Option<impl Iterator<Item = (T, usize)>> {
    peaks_by(width, values, |a, b| a.cmp(b))
}