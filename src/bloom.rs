use std::hash::{BuildHasher, Hash, Hasher, RandomState};

#[derive(Debug)]
pub struct BloomFilter {
    hasher_states: Vec<RandomState>,
    store: Vec<bool>,
}

impl<'a> BloomFilter {
    pub fn new(m: usize, k: usize) -> Self {
        Self {
            hasher_states: (0..k).map(|_| RandomState::new()).collect(),
            store: vec![false; m],
        }
    }

    pub fn insert<T: Hash>(&mut self, item: T) {
        let m = self.store.len();
        for index in &mut self
            .hasher_states
            .iter_mut()
            .map(|state| hash_item(&item, state, m))
        {
            self.store[index] = true;
        }
    }

    pub fn has<T: Hash>(&mut self, item: T) -> bool {
        let m = self.store.len();
        self.hasher_states
            .iter_mut()
            .map(|state| hash_item(&item, state, m))
            .fold(true, |acc, index| match acc {
                false => false,
                _ => self.store[index],
            })
    }
}

fn hash_item<T: Hash>(item: T, state: &mut RandomState, bound: usize) -> usize {
    let mut hasher = state.build_hasher();
    item.hash(&mut hasher);
    (hasher.finish() as usize).rem_euclid(bound)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut bloom_filter = BloomFilter::new(10000, 2);
        bloom_filter.insert("foo");
        assert!(bloom_filter.has("foo"));
        assert!(!bloom_filter.has("bar"));
        bloom_filter.insert("foo");
        assert!(bloom_filter.has("foo"));
    }
}
