use std::collections::VecDeque;

pub trait Average: Sized {
    fn zero() -> Self;
    fn add(&mut self, b: Self);
    fn divide(&mut self, count: u32);

    fn average(values: impl Iterator<Item = Self>) -> Self {
        let mut initial = Self::zero();

        let mut count = 0;

        for value in values {
            initial.add(value);
            count += 1;
        }

        initial.divide(count);

        initial
    }
}

impl Average for f32 {
    fn zero() -> f32 {
        0.0
    }

    fn add(&mut self, b: Self)  {
        *self += b;
    }

    fn divide(&mut self, count: u32) {
        *self /= count as f32;
    }
}

impl Average for f64 {
    fn zero() -> f64 {
        0.0
    }

    fn add(&mut self, b: Self)  {
        *self += b;
    }

    fn divide(&mut self, count: u32) {
        *self /= count as f64;
    }
}

impl Average for i64 {
    fn zero() -> i64 {
        0
    }

    fn add(&mut self, b: Self)  {
        *self += b;
    }

    fn divide(&mut self, count: u32) {
        *self /= count as i64;
    }
}

pub fn moving_average<T: Average + Clone>(width: usize, mut values: impl Iterator<Item = T>) -> Option<impl Iterator<Item = T>> {
    let mut queue = VecDeque::with_capacity(width);

    for _ in 1..width {
        match values.next() {
            Some(value) => queue.push_back(value),
            None => return None
        }
    }

    Some(values.map(move |item| {
        queue.push_back(item);

        let value = T::average(queue.iter().cloned());
        queue.pop_front();

        value
    }))
}