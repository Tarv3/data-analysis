use std::{ops::{Div, Add}, cmp::Ordering};

pub fn reduce<U, T, F>(
    values: impl Iterator<Item = T>, 
    init: U, 
    mut f: F
) -> U 
where
    F: FnMut(&mut U, T)
{
    let mut output = init;

    for value in values {
        f(&mut output, value);
    }

    output
}

pub fn middle<T, O, U>(values: &[T], combine: U) -> Option<O> 
where
    U: FnOnce(&T, Option<&T>) -> O,
{
    let len = values.len();
    if len == 0 {
        None
    }
    else if len % 2 == 1 {
        let middle = len / 2;
        let output = combine(&values[middle], None);

        Some(output)
    }
    else {
        let middle1 = len / 2;
        let middle2 = middle1 - 1;
        let output = combine(&values[middle1], Some(&values[middle2]));

        Some(output)
    }
}

pub fn quartiles<T, O, F, U>(values: &mut [T], compare: F, mut combine: U) -> Option<[O; 3]> 
where
    T: std::fmt::Debug,
    F: FnMut(&T, &T) -> Ordering,
    U: FnMut(&T, Option<&T>) -> O,
{
    values.sort_by(compare);
    let median = middle(values, &mut combine)?;

    let len = values.len();

    if len % 2 == 1 {
        let left = &values[0..=(len / 2)];
        let right = &values[(len / 2)..];
        let left_med = middle(left, &mut combine)?;
        let right_med = middle(right, &mut combine)?;

        Some([left_med, median, right_med])
    }
    else {
        let left = &values[0..(len / 2)];
        let right = &values[(len / 2)..];
        let left_med = middle(left, &mut combine)?;
        let right_med = middle(right, &mut combine)?;

        Some([left_med, median, right_med])
    }
}