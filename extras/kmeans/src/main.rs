#![allow(non_snake_case)]
use ndarray::{stack, Array, Array2, Axis};
use ndarray_rand::RandomExt;
use ndarray_rand::rand_distr::Normal;

// Import KMeans from other file ("lib.rs") by project name
use kmeans::KMeans;

fn get_data(n_samples: usize, n_features: usize) -> Array2<f64> {
    let shape = (n_samples / 2, n_features);
    let X1: Array2<f64> = Array::random(shape, Normal::new(500., 50.0).unwrap());
    let X2: Array2<f64> = Array::random(shape, Normal::new(-500., 50.0).unwrap());
    let X3: Array2<f64> = Array::random(shape, Normal::new(0., 30.0).unwrap());

    stack(Axis(0), &[X1.view(), X2.view(), X3.view()]).unwrap().to_owned()
}

pub fn main() {
    let n_samples = 50000;
    let n_features = 3;
    let n_clusters = 3;

    let X = get_data(n_samples, n_features);

    let mut k_means = KMeans::new(n_clusters);
    k_means.fit(&X);

    println!("The centroids are {:.3}", k_means.centroids.unwrap());
}