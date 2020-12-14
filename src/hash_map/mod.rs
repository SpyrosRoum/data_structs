use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::mem;

#[cfg(test)]
mod tests;

pub struct HashMap<K, V> {
    buckets: Vec<Vec<(K, V)>>,
    len: usize,
}

impl<K, V> HashMap<K, V>
where
    K: Eq + Hash,
{
    pub fn new() -> Self {
        Self {
            buckets: Vec::new(),
            len: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    /// Inserts a key-value pair into the map.
    ///
    /// If the map did not have this key present, [`None`] is returned.
    ///
    /// If the map did have this key present, the value is updated, and the old
    /// value is returned.
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        if self.buckets.is_empty() || self.len > 3 * self.buckets.len() / 4 {
            self.resize();
        }

        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        let i = (hasher.finish() % self.buckets.len() as u64) as usize;

        for (bucket_k, bucket_v) in self.buckets[i].iter_mut() {
            if bucket_k == &key {
                return Some(mem::replace(bucket_v, value));
            }
        }

        self.buckets[i].push((key, value));
        self.len += 1;
        None
    }

    /// Removes a key from the map, returning the value at the key if the key was previously in the map.
    pub fn remove(&mut self, key: &K) -> Option<V> {
        if self.len == 0 {
            return None;
        }
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        let i = (hasher.finish() % self.buckets.len() as u64) as usize;

        match self.buckets[i].iter().position(|(k, _)| k == key) {
            Some(pos) => {
                self.len -= 1;
                Some(self.buckets[i].remove(pos).1)
            }
            None => None,
        }
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        if self.len == 0 {
            return None;
        }
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        let i = (hasher.finish() % self.buckets.len() as u64) as usize;

        self.buckets[i]
            .iter()
            .find(|(k, _)| k == key)
            .map(|(_, v)| v)
    }

    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        if self.len == 0 {
            return None;
        }
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        let i = (hasher.finish() % self.buckets.len() as u64) as usize;

        self.buckets[i]
            .iter_mut()
            .find(|(k, _)| k == key)
            .map(|(_, v)| v)
    }

    // Create new buckets and copy everything there
    fn resize(&mut self) {
        let new_size = match self.buckets.len() {
            0 => 2,
            n => n * 2,
        };

        let mut new_buckets = Vec::with_capacity(new_size);
        new_buckets.extend((0..new_size).map(|_| Vec::new()));
        for mut bucket in self.buckets.drain(..) {
            for (k, v) in bucket.drain(..) {
                let mut hasher = DefaultHasher::new();
                k.hash(&mut hasher);
                let i = (hasher.finish() % new_size as u64) as usize;
                new_buckets[i].push((k, v))
            }
        }

        mem::swap(&mut self.buckets, &mut new_buckets)
    }
}
