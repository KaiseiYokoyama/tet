#[cfg(feature = "serde1")]
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

/// frequency of characters
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
#[derive(Debug, PartialEq, Clone)]
pub struct Frequencies {
    /// map of frequencies
    map: HashMap<char, u128>,
}

impl Frequencies {
    pub fn new() -> Self {
        Frequencies {
            map: HashMap::new()
        }
    }

    pub fn with_map(map: HashMap<char, u128>) -> Self {
        Self { map }
    }

    /// record an appearance of char
    pub fn record(&mut self, c: char) {
        if let Some(record) = self.map.get_mut(&c) {
            *record += 1;
        } else {
            self.map.insert(c, 1);
        }
    }

    pub fn n(&self) -> u128 {
        self.map.values().sum::<u128>()
    }

    pub fn retain<F: Fn(&char) -> bool>(&mut self, func: F) {
        self.map.retain(|c, _| func(c))
    }

    pub fn entry_char(&mut self, c: char) {
        if !self.map.contains_key(&c) {
            self.map.insert(c, 0);
        } else {}
    }
}

/// distribution of characters
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
#[derive(Debug, PartialEq)]
pub struct Distribution {
    /// map of distribution
    pub(crate) map: HashMap<char, f64>,
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

    pub fn with_map(map: HashMap<char, f64>) -> Self {
        Self { map }
    }

    pub(crate) fn p(&self, c: &char) -> Option<&f64> {
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

