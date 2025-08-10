use crate::{config::Strategy, load_balancer::strategies::random::RandomStrategy};

pub mod strategies;

pub struct LoadBalancer(Strategy);

impl LoadBalancer {
    pub fn new(s: Strategy) -> Self {
        LoadBalancer(s)
    }

    pub fn select_instance<T: Clone>(&self, instances: &[T]) -> Option<T> {
        let mut strategy = self.0.build();
        strategy.select_instance(instances)
    }
}

impl Strategy {
    pub fn build<T: Clone>(&self) -> Box<dyn LoadBalancingStrategy<T>> {
        match self {
            Strategy::Random => Box::new(RandomStrategy),
        }
    }
}

pub trait LoadBalancingStrategy<T: Clone> {
    fn select_instance(&mut self, instances: &[T]) -> Option<T>;
}
