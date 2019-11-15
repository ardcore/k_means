extern crate num_traits;
use rand::distributions::{IndependentSample, Range};
use std::cmp::{Ordering, PartialOrd};
use std::ops::{Sub, Add};
use std::iter::Sum;
use num_traits::sign::Signed;
use num_traits::{FromPrimitive, ToPrimitive};
use std::marker::Copy;

pub trait NumberLike: PartialOrd + ToPrimitive + Sum + Add + Copy {}
impl<T: PartialOrd + ToPrimitive + Sum + Add + Copy> NumberLike for T {}

#[derive(Debug)]
pub struct Group<T: NumberLike> {
    center: f64,
    items: Vec<T>,
}

impl<T: NumberLike> Group<T> {
    fn add(&mut self, item: T) {
        self.items.push(item);
    }
    fn set_center(&mut self, center: f64) {
        self.center = center;
    }
    fn reset(&mut self) {
        self.items.clear();
    }
}

fn centroid<'a, T: 'a>(numbers: &'a [T]) -> Option<f64>
where
    T: NumberLike,
{
    match numbers.len() {
        0 => None,
        len => {
            let mut sum: f64 = 0.0;
            for n in numbers {
                sum = sum + T::to_f64(n).unwrap();
            }
            let length = f64::from_usize(len)?;
            Some(sum / length)
        }
    }
}

fn distance<T: Sub<Output = T> + Signed>(a: T, b: T) -> T {
    (a - b).abs()
}

fn closest<'a, T: NumberLike>(groups: &'a mut Vec<Group<T>>, target: &T) -> &'a mut Group<T> {
    let closest = groups
        .iter_mut()
        .min_by(|a, b| {
            let da = distance(a.center, T::to_f64(&target).unwrap());
            let db = distance(b.center, T::to_f64(&target).unwrap());
            da.partial_cmp(&db).unwrap_or(Ordering::Equal)
        })
        .unwrap();
    closest
}

fn qualify<T: NumberLike>(el: T, mut groups: &mut Vec<Group<T>>) {
    let closest_group = closest(&mut groups, &el);
    closest_group.add(el);
}

fn update_centroid<T: NumberLike>(group: &mut Group<T>) {
    if group.items.len() > 0 {
        let centroid = centroid(&group.items).unwrap();
        group.set_center(centroid);
    }
}

pub fn solve<T: NumberLike>(data: &Vec<T>, groups_count: i32, epochs: i32) -> Vec<Group<T>> {
    let max = data
        .iter()
        .max_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal))
        .unwrap();
    let min = data
        .iter()
        .min_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal))
        .unwrap();

    let min = T::to_f64(&min).unwrap();
    let max = T::to_f64(&max).unwrap();

    let range = Range::new(min, max);
    let mut rng = rand::thread_rng();

    let mut groups: Vec<Group<T>> = Vec::new();

    for _ in 0..groups_count {
        let center = range.ind_sample(&mut rng);
        let group = Group {
            center,
            items: Vec::new(),
        };
        groups.push(group);
    }

    for _ in 0..epochs {
        groups.iter_mut().for_each(|group| group.reset());
        data.iter()
            .for_each(|el| qualify(*el, &mut groups));
        groups
            .iter_mut()
            .for_each(|mut group| update_centroid(&mut group));
    }

    groups
}
