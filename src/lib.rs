use std::collections::HashMap;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

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

pub struct Distribution {
    /// map of distribution
    map: HashMap<char, f64>,
}

impl Distribution {
    pub fn new(frequencies: Frequencies) -> Self {
        let n = frequencies.map.values()
            .sum::<u128>() as f64;

        let map = frequencies.map.iter()
            .map(|(&k,&v)| {
                (k, v as f64 / n)
            })
            .collect();

        Self { map }
    }
}

mod optimal_alignments {
    #[derive(Debug, Clone, Eq, PartialEq)]
    enum Element {
        Character(char),
        Padding,
    }

    #[derive(Debug, Default, Eq, PartialEq)]
    pub struct OptimalAlignments {
        presented: Vec<Element>,
        transcribed: Vec<Element>,
    }

    impl OptimalAlignments {
        pub fn new<P, T>(presented: P, transcribed: T) -> Self
            where P: Into<&'static str>, T: Into<&'static str>
        {
            let (presented, transcribed) = (presented.into(), transcribed.into());

            let mut slf = Self::default();

            let mut d = Self::msd(presented, transcribed);

            let (presented, transcribed): (Vec<char>, Vec<char>) = (
                presented.to_string().chars().collect(),
                transcribed.to_string().chars().collect()
            );

            let (x, y) = (presented.len(), transcribed.len());

            slf.alignments(
                &presented,
                &transcribed,
                &mut d, x, y,
                Vec::new(),
                Vec::new(),
            );

            slf
        }

        fn msd(presented: &str, transcribed: &str) -> Vec<Vec<u128>> {
            fn r(x: char, y: char) -> u128 {
                if x == y { 0 } else { 1 }
            }

            let mut d = std::iter::repeat(
                std::iter::repeat(0u128)
                    .take(transcribed.chars().count() + 1)
                    .collect::<Vec<_>>()
            )
                .take(presented.chars().count() + 1)
                .collect::<Vec<_>>();

            for i in 0..=presented.chars().count() {
                d[i][0] = i as u128;
            }

            for j in 0..=transcribed.chars().count() {
                d[0][j] = j as u128;
            }

            for i in 1..=presented.chars().count() {
                for j in 1..=transcribed.chars().count() {
                    let mut candidates = [
                        d[i - 1][j] + 1,
                        d[i][j - 1] + 1,
                        d[i - 1][j - 1] + r(
                            presented.chars().skip(i - 1).next().unwrap(),
                            transcribed.chars().skip(j - 1).next().unwrap(),
                        )
                    ];
                    candidates.sort();
                    d[i][j] = candidates[0];
                }
            }

            d
        }

        fn alignments(
            &mut self,
            presented: &Vec<char>,
            transcribed: &Vec<char>,
            d: &mut Vec<Vec<u128>>,
            x: usize,
            y: usize,
            p_aligned: Vec<Element>,
            t_aligned: Vec<Element>,
        )
        {
            if x == 0 && y == 0 {
                self.presented = p_aligned;
                self.transcribed = t_aligned;

                return;
            }

            if x > 0 && y > 0 {
                if d[x][y] == d[x - 1][y - 1] && presented[x - 1] == transcribed[y - 1] {
                    let (mut p_aligned, mut t_aligned) = (p_aligned.clone(), t_aligned.clone());
                    p_aligned.insert(0, Element::Character(presented[x - 1]));
                    t_aligned.insert(0, Element::Character(transcribed[y - 1]));

                    // recursive call
                    self.alignments(presented, transcribed, d, x - 1, y - 1, p_aligned, t_aligned);
                }

                if d[x][y] == d[x - 1][y - 1] + 1 {
                    let (mut p_aligned, mut t_aligned) = (p_aligned.clone(), t_aligned.clone());
                    p_aligned.insert(0, Element::Character(presented[x - 1]));
                    t_aligned.insert(0, Element::Character(transcribed[y - 1]));

                    // recursive call
                    self.alignments(presented, transcribed, d, x - 1, y - 1, p_aligned, t_aligned);
                }
            }

            if x > 0 && d[x][y] == d[x - 1][y] + 1 {
                let (mut p_aligned, mut t_aligned) = (p_aligned.clone(), t_aligned.clone());
                p_aligned.insert(0, Element::Character(presented[x - 1]));
                t_aligned.insert(0, Element::Padding);

                // recursive call
                self.alignments(presented, transcribed, d, x - 1, y, p_aligned, t_aligned);
            }

            if y > 0 && d[x][y] == d[x][y - 1] + 1 {
                let (mut p_aligned, mut t_aligned) = (p_aligned.clone(), t_aligned.clone());
                p_aligned.insert(0, Element::Padding);
                t_aligned.insert(0, Element::Character(transcribed[y - 1]));

                // recursive call
                self.alignments(presented, transcribed, d, x, y - 1, p_aligned, t_aligned);
            }

            return;
        }
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn msd_test() {
            let d = OptimalAlignments::msd("abcd", "acbd");
            let answer = vec![
                vec![0, 1, 2, 3, 4],
                vec![1, 0, 1, 2, 3],
                vec![2, 1, 1, 1, 2],
                vec![3, 2, 1, 2, 2],
                vec![4, 3, 2, 2, 2],
            ];

            assert_eq!(d, answer);

            let d = OptimalAlignments::msd("quickly", "qucehkly");
            assert_eq!(d[7][8], 3)
        }

        #[test]
        fn alignment_test() {
            let presented = "quickly";
            let transcribed = "qucehkly";

            let optimal_alignment = OptimalAlignments::new(presented, transcribed);
            let answer = OptimalAlignments {
                presented: vec![
                    Element::Character('q'),
                    Element::Character('u'),
                    Element::Character('i'),
                    Element::Character('c'),
                    Element::Padding,
                    Element::Padding,
                    Element::Character('k'),
                    Element::Character('l'),
                    Element::Character('y'),
                ],
                transcribed: vec![
                    Element::Character('q'),
                    Element::Character('u'),
                    Element::Padding,
                    Element::Character('c'),
                    Element::Character('e'),
                    Element::Character('h'),
                    Element::Character('k'),
                    Element::Character('l'),
                    Element::Character('y'),
                ],
            };

            assert_eq!(optimal_alignment, answer);
        }
    }
}