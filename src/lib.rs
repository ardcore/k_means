extern crate num_traits;
use num_traits::sign::Signed;
use num_traits::{FromPrimitive, ToPrimitive};
use rand::distributions::{IndependentSample, Range};
use std::cmp::{Ordering, PartialOrd};
use std::marker::Copy;
use std::ops::{Add, Sub};

pub trait NumberLike: PartialOrd + ToPrimitive + Add + Copy {}
impl<T: PartialOrd + ToPrimitive + Add + Copy> NumberLike for T {}

#[derive(Debug)]
pub struct Cluster<T: NumberLike> {
    center: f32,
    items: Vec<T>,
}

impl<T: NumberLike> Cluster<T> {
    fn add(&mut self, item: T) {
        self.items.push(item);
    }
    fn set_center(&mut self, center: f32) {
        self.center = center;
    }
    fn reset(&mut self) {
        self.items.clear();
    }
}

fn centroid<'a, T: 'a>(numbers: &'a [T]) -> Option<f32>
where
    T: NumberLike,
{
    match numbers.len() {
        0 => None,
        len => {
            let mut sum: f32 = 0.0;
            for n in numbers {
                sum = sum + T::to_f32(n)?;
            }
            let length = f32::from_usize(len).unwrap_or(1.0);
            Some(sum / length)
        }
    }
}

fn distance<T: Sub<Output = T> + Signed>(a: T, b: T) -> T {
    (a - b).abs()
}

fn closest<'a, T: NumberLike>(clusters: &'a mut Vec<Cluster<T>>, target: &T) -> &'a mut Cluster<T> {
    let target = match T::to_f32(&target) {
        Some(n) => n,
        None => panic!("Can't unwrap value")
    };

    let closest = match clusters
        .iter_mut()
        .min_by(|a, b| {
            let da = distance(a.center, target);
            let db = distance(b.center, target);
            da.partial_cmp(&db).unwrap_or(Ordering::Equal)
        }) {
            Some(n) => n,
            None => panic!("Cant unwrap distance")
        };

    closest
}

fn qualify<T: NumberLike>(el: T, mut clusters: &mut Vec<Cluster<T>>) {
    let closest_cluster = closest(&mut clusters, &el);
    closest_cluster.add(el);
}

fn update_centroid<T: NumberLike>(cluster: &mut Cluster<T>) {
    if !cluster.items.is_empty() {
        let centroid = match centroid(&cluster.items) {
            Some(v) => v,
            None => panic!("Cant unwrap centroid")
        };

        cluster.set_center(centroid);
    }
}

pub fn solve<T: NumberLike>(data: &Vec<T>, clusters_count: i32, epochs: i32) -> Vec<Cluster<T>> {
    let max = data
        .iter()
        .max_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal))
        .unwrap();
    let min = data
        .iter()
        .min_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal))
        .unwrap();

    let min = T::to_f32(&min).unwrap();
    let max = T::to_f32(&max).unwrap();

    let range = Range::new(min, max);
    let mut rng = rand::thread_rng();

    let mut clusters: Vec<Cluster<T>> = Vec::new();

    for _ in 0..clusters_count {
        let center = range.ind_sample(&mut rng);
        let cluster = Cluster {
            center,
            items: Vec::new(),
        };
        clusters.push(cluster);
    }

    for _ in 0..epochs {
        clusters.iter_mut().for_each(|cluster| cluster.reset());
        data.iter().for_each(|el| qualify(*el, &mut clusters));
        clusters
            .iter_mut()
            .for_each(|mut cluster| update_centroid(&mut cluster));
    }

    clusters
}
