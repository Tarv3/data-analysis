use crate::average::*;
use std::{ cmp::Ordering, io::Write };

pub fn write_summary_headers(prefix: &str, mut writer: impl Write) -> std::io::Result<()> {
    write!(writer, "{0}avg,{0}min,{0}max,{0}low,{0}med,{0}up,", prefix)
}

pub struct Summary<T> {
    pub avg: T,
    pub variance: T,
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
        let variance = T::variance(&avg, values.iter().cloned());
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
            variance,
            min,
            max,
            quartiles
        }
    }

    pub fn to_writer(&self, mut writer: impl Write) -> std::io::Result<()> 
    where
        T: std::fmt::Display,
    {
        let min = self.min.clone().unwrap_or(T::zero());
        let max = self.max.clone().unwrap_or(T::zero());
        let [low, med, up] = self.quartiles.clone().unwrap_or([T::zero(), T::zero(), T::zero()]);

        write!(writer, "{},{},{},{},{},{}", self.avg, min, max, low, med, up)
    }
}
