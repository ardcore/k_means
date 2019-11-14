extern crate num_traits;
use rand::distributions::{IndependentSample, Range};
use std::cmp::Ordering;
use std::ops::{Sub};
use std::iter::Sum;
use num_traits::sign::Signed;
use num_traits::{FromPrimitive, ToPrimitive};


#[derive(Debug)]
pub struct Group {
    center: f64,
    items: Vec<f32>,
}

impl Group {
    fn add(&mut self, item: f32) {
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
    T: ToPrimitive + Sum<&'a T>,
{
    match numbers.len() {
        0 => None,
        len => {
            let sum = numbers.iter().sum::<T>();
            let length = f64::from_usize(len)?;

            T::to_f64(&sum).map(|sum| sum / length)
        }
    }
}

fn distance<T: Sub<Output = T> + Signed>(a: T, b: T) -> T {
    (a - b).abs()
}

fn closest<'a>(groups: &'a mut Vec<Group>, target: f32) -> &'a mut Group {
    let closest = groups
        .iter_mut()
        .min_by(|a, b| {
            let da = distance(a.center, target as f64);
            let db = distance(b.center, target as f64);
            da.partial_cmp(&db).unwrap_or(Ordering::Equal)
        })
        .unwrap();
    closest
}

fn qualify(el: f32, mut groups: &mut Vec<Group>) {
    let closest_group = closest(&mut groups, el);
    closest_group.add(el);
}

fn update_centroid(group: &mut Group) {
    if group.items.len() > 0 {
        let centroid = centroid(&group.items).unwrap();
        group.set_center(centroid);
    }
}

pub fn solve(data: &Vec<f32>, groups_count: i32, epochs: i32) -> Vec<Group> {
    let max = data
        .iter()
        .max_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));
    let min = data
        .iter()
        .min_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));

    let range = Range::new(*min.unwrap() as u32, *max.unwrap() as u32);
    let mut rng = rand::thread_rng();

    let mut groups: Vec<Group> = Vec::new();

    for _ in 0..groups_count {
        let center = range.ind_sample(&mut rng);
        let group = Group {
            center: center as f64,
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
