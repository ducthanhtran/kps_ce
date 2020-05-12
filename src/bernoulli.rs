use std::mem;

use ndarray::{Array1, Array2, Axis};

use rand::distributions::{Bernoulli, Distribution};
use rand::rngs::StdRng;


#[derive(Debug)]
pub struct BernoulliProduct {
    parameters: Array1<f64>,
    distributions: Vec<Bernoulli>
}

#[allow(dead_code)]
impl BernoulliProduct {
    pub fn new(dimension: usize) -> BernoulliProduct {
        let parameters: Array1<f64> = Array1::from(vec![0.5; dimension]);
        let distributions: Vec<Bernoulli> = vec![Bernoulli::new(0.5).unwrap(); dimension];
        BernoulliProduct { parameters, distributions}
    }

    fn sample(&self, rng: &mut StdRng) -> Vec<f64> {
        self.distributions.iter().map(|d| u8::from(d.sample(rng)))
                                 .map(|v| f64::from(v))
                                 .collect()
    }

    fn dim(&self) -> usize {
        self.distributions.len()
    }

    pub fn sample_matrix(&self, sample_size: usize, rng: &mut StdRng) -> Array2<f64> {
        let mut samples: Vec<f64> = Vec::new();
        for _ in 0..sample_size {
            samples.extend(self.sample(rng).iter());
        }
        Array2::from_shape_vec((sample_size, self.dim()), samples).unwrap()
    }

    /// best_samples has dimensionality (#best_samples, self.dim())
    pub fn update(&mut self, best_samples: Array2<f64>) {
        let mean = best_samples.mean_axis(Axis(0)).unwrap();
        for i in 0..self.dim() {
            self.parameters[i] = mean[i];
            mem::replace(&mut self.distributions[i], Bernoulli::new(mean[i]).unwrap());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Array1, BernoulliProduct, StdRng};
    use rand::SeedableRng;

    #[test]
    fn test_bernoulli_product_dist_creation() {
        let dim = 3;
        let dist = BernoulliProduct::new(dim);
        assert_eq!(dist.distributions.len(), dim);
        assert!(dist.parameters == Array1::from(vec![0.5; dim]));
    }

    #[test]
    fn test_bernoulli_product_dist_sample() {
        let dim = 5;
        let dist = BernoulliProduct::new(dim);
        let mut rng = StdRng::seed_from_u64(22);
        let sampled = dist.sample(&mut rng);
        assert_eq!(sampled.len(), dim);
        for &v in sampled.iter() {
            assert!(0f64 <= v && v <= 1f64);
        }
    }

    #[test]
    fn test_bernoulli_product_dist_sample_matrix() {
        let dim = 5;
        let dist = BernoulliProduct::new(dim);
        let mut rng = StdRng::seed_from_u64(32);
        let sample_size = 25;
        let sample_matrix = dist.sample_matrix(sample_size, &mut rng);
        assert_eq!(sample_matrix.shape(), &[sample_size, dim]);
        for &v in sample_matrix.iter() {
            assert!(0f64 <= v && v <= 1f64);
        }
    }

    #[test]
    fn test_bernoulli_product_dist_update() {
        let dim = 5;
        let mut dist = BernoulliProduct::new(dim);

        let best_samples = ndarray::arr2(&[[1., 0., 1., 1., 0.],
                                                                           [0., 1., 1., 0., 0.],
                                                                           [1., 1., 1., 0., 0.]]);
        dist.update(best_samples);
        assert_eq!(dist.parameters, ndarray::arr1(&[2./3., 2./3., 1., 1./3., 0.]));

        let mut rng = StdRng::seed_from_u64(32);
        for _ in 0..10 {
            assert_eq!(dist.sample(&mut rng)[2], 1.);
            assert_eq!(dist.sample(&mut rng)[4], 0.);
        }
    }
}