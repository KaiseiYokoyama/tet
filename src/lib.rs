use std::collections::HashMap;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub struct Frequencies<C: std::hash::Hash + std::cmp::Eq> {
    /// map of frequencies
    map: HashMap<C, u128>,
}

impl<C: std::hash::Hash + std::cmp::Eq> Frequencies<C> {
    /// record an appearance of C
    pub fn record(&mut self, c: C) {
        if let Some(record) = self.map.get_mut(&c) {
            *record += 1;
        } else {
            self.map.insert(c, 1);
        }
    }
}

pub struct Distribution<'a, C: std::hash::Hash + std::cmp::Eq> {
    /// map of distribution
    map: HashMap<&'a C, f64>,
}

impl<'a, C: std::hash::Hash + std::cmp::Eq> Distribution<'a, C> {
    pub fn new(frequencies: &'a Frequencies<C>) -> Self {
        let n = frequencies.map.values()
            .sum::<u128>() as f64;

        let map = frequencies.map.iter()
            .map(|(k,&v)| {
                (k, v as f64 / n)
            })
            .collect();

        Self { map }
    }
}

