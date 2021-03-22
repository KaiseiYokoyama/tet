//! # Text Entry Throughput
//! Text entry throughput [introduced by Minguri et al.](https://dl.acm.org/doi/fullHtml/10.1145/3290605.3300866)
//! is a text entry method-independent throughput metric based on Shannon information theory.
//!
//! This crate is a third-party implementation of TET.
//!
//! ## TL;DR (English alphabet only)
//! ```
//! use tet::TextEntryThroughput;
//!
//! let tet = TextEntryThroughput::alphabet_letter_distribution();
//!
//! let presented_text = "my watch fell in the waterprevailing wind from the east";
//! let transcribed_text = "my wacch fell in waterpreviling wind on the east";
//! let s = std::time::Duration::from_secs(12); // 4 characters per second
//!
//! let throughput = tet.calc(presented_text, transcribed_text, s).unwrap();
//! assert!((throughput - 12.954965333409255).abs() < 0.0001);
//! ```
//!
//! ## Usage
//! ### Get distribution
//! First, prepare a distribution of characters to get entropy `H(X)` from source.
//! ```
//! use tet::{Frequencies, Distribution};
//!
//! let mut frequency = Frequencies::new();
//!
//! // get frequency of each character
//! let source = "large and appropriate text is recommended";
//! source.chars()
//!     .for_each(|c| {
//!         frequency.record(c.clone());
//!     });
//!
//! // normalize frequency to get distribution
//! let distribution = Distribution::new(frequency);
//! ```
//!
//! ### Compute TET
//! ```
//! # use tet::*;
//! # use tet::{Frequencies, Distribution};
//! #
//! # let mut frequency = Frequencies::new();
//! #
//! # // get frequency of each character
//! # let source = "large and appropriate text is recommended";
//! # source.chars()
//! #     .for_each(|c| {
//! #         frequency.record(c.clone());
//! #     });
//! #
//! # // normalize frequency to get distribution
//! # let distribution = Distribution::new(frequency);
//! #
//! // now you can calculate TET! :+1:
//! let tet = TextEntryThroughput::new(distribution);
//!
//! let (presented, transcribed) = ("quickly", "qucehkly");
//! let s = std::time::Duration::from_secs(2); // 4 characters per minute
//!
//! // Text Entry Throughput (bits/second)
//! let throughput = tet.calc(presented, transcribed, s).unwrap();
//! ```


pub use crate::distribution::{Distribution, Frequencies};
use std::collections::HashMap;

mod distribution;
mod optimal_alignments;

pub struct TextEntryThroughput {
    distribution: Distribution
}

impl TextEntryThroughput {
    pub fn new(distribution: Distribution) -> Self {
        Self { distribution }
    }

    pub fn with_map(map: HashMap<char, f64>) -> Self {
        Self {
            distribution: Distribution { map }
        }
    }

    pub fn alphabet_letter_distribution() -> Self {
        let alphabets = [
            'a', 'b', 'c', 'd', 'e',
            'f', 'g', 'h', 'i', 'j',
            'k', 'l', 'm', 'n', 'o',
            'p', 'q', 'r', 's', 't',
            'u', 'v', 'w', 'x', 'y',
            'z', ' '
        ];

        let distribution = [
            0.06545420428810268, 0.012614349400134882, 0.022382079660795914, 0.032895839710101495, 0.10287480840814522,
            0.019870906945619955, 0.01628201251975626, 0.0498866519336527, 0.05679944220647908, 0.0009771967640664421,
            0.005621008826086285, 0.03324279082953061, 0.020306796250368523, 0.057236004874678816, 0.061720746945911634,
            0.015073764715016882, 0.0008384527300266635, 0.049980287430261394, 0.05327793252372975, 0.07532249847431097,
            0.022804128240333354, 0.007977317166161044, 0.017073508770571122, 0.0014120607927983009, 0.014305632773116854,
            0.0005138874382474097, 0.18325568938199557];

        let map = alphabets.iter().cloned()
            .zip(distribution.iter().cloned())
            .collect::<HashMap<_, _>>();

        Self::with_map(map)
    }

    /// compute a text entry throughput (bits/s)
    ///
    /// - presented: presented text
    /// - transcribed: transcribed text
    /// - s: time in seconds required for entry transcribed text
    pub fn calc<P, T>(&self, presented: P, transcribed: T, s: std::time::Duration) -> Option<f64>
        where P: Into<&'static str>, T: Into<&'static str>
    {
        use optimal_alignments::OptimalAlignments;

        let transcribed = transcribed.into();
        let characters_per_second = transcribed.chars().count() as f64 / s.as_secs_f64();

        let alignments = OptimalAlignments::new(presented, transcribed, &self.distribution);
        alignments.ixy().map(|ixy| ixy * characters_per_second)
    }
}

#[cfg(test)]
mod test {
    use crate::TextEntryThroughput;

    #[test]
    fn text_entry_throughput_test() {
        let tet = TextEntryThroughput::alphabet_letter_distribution();

        let presented = "my watch fell in the waterprevailing wind from the east";
        let transcribed = "my wacch fell in waterpreviling wind on the east";
        let s = std::time::Duration::from_secs(12);

        let throughput = tet.calc(presented, transcribed, s).unwrap();

        // 3.238741333352314 * 4.0 = 12.954965333409256
        // -> significant digits
        // 3.238 * 4.000 = 12.952 (on the paper)
        // paper: https://dl.acm.org/doi/fullHtml/10.1145/3290605.3300866
        assert!((throughput - 12.954965333409255).abs() < 0.0001);
    }
}