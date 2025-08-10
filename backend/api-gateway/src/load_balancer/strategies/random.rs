use rand::seq::IndexedRandom;

use crate::load_balancer::LoadBalancingStrategy;

pub struct RandomStrategy;

impl<T: Clone> LoadBalancingStrategy<T> for RandomStrategy {
    fn select_instance(&mut self, instances: &[T]) -> Option<T> {
        let mut rng = rand::rng();
        instances.choose(&mut rng).cloned()
    }
}
