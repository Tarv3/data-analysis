use crate::average::*;
use std::cmp::Ordering;

pub struct Summary<T> {
    pub avg: T,
    pub min: Option<T>,
    pub max: Option<T>,
    pub quartiles: Option<[T; 3]>
}

impl<T> Summary<T> 
where
    T: Average + Clone,
{
    pub fn from_iter<O>(values: impl Iterator<Item = T>, mut ordering: O) -> Summary<T>
    where
        O: FnMut(&T, &T) -> Ordering,
    {
        let mut values: Vec<T> = values.collect();

        let avg = T::average(values.iter().cloned());
        let min = values.iter().cloned().min_by(&mut ordering);
        let max = values.iter().cloned().max_by(&mut ordering);  
        
        let quartiles = crate::math::quartiles(&mut values, &mut ordering, |a, b| {
            let b = b.iter().map(|value| (*value).clone());
            let a = a.clone();
            
            let iter = std::iter::once(a).chain(b);
            T::average(iter)
        });

        Summary {
            avg,
            min,
            max,
            quartiles
        }
    }
}