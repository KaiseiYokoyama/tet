use std::collections::HashMap;

pub struct Frequencies {
    /// map of frequencies
    map: HashMap<char, u128>,
}

impl Frequencies {
    /// record an appearance of char
    pub fn record(&mut self, c: char) {
        if let Some(record) = self.map.get_mut(&c) {
            *record += 1;
        } else {
            self.map.insert(c, 1);
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Distribution {
    /// map of distribution
    map: HashMap<char, f64>,
}

impl Distribution {
    pub fn new(frequencies: Frequencies) -> Self {
        let n = frequencies.map.values()
            .sum::<u128>() as f64;

        let map = frequencies.map.iter()
            .map(|(&k, &v)| {
                (k, v as f64 / n)
            })
            .collect();

        Self { map }
    }

    pub fn p(&self, c: &char) -> Option<&f64> {
        self.map.get(c)
    }

    /// H(X): entropy
    pub fn hx(&self) -> f64 {
        -self.map.iter()
            .map(|(_, &pi)| {
                pi * pi.log2()
            })
            .sum::<f64>()
    }
}

mod optimal_alignments;