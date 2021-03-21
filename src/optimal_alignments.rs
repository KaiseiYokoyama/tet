use crate::distribution::Distribution;

#[derive(Debug, Clone, Eq, PartialEq)]
enum Element {
    Character(char),
    Null,
}

impl Element {
    pub fn is_null(&self) -> bool {
        self == &Element::Null
    }
}

#[derive(Debug, PartialEq)]
pub struct OptimalAlignments<'a> {
    distribution: &'a Distribution,
    presented: Vec<Element>,
    transcribed: Vec<Element>,
    p_null: f64,
    len: usize,
}

impl<'a> OptimalAlignments<'a> {
    pub fn new<P, T>(presented: P, transcribed: T, distribution: &'a Distribution) -> Self
        where P: Into<&'static str>, T: Into<&'static str>
    {
        let (presented, transcribed) = (presented.into(), transcribed.into());

        let mut slf = Self {
            distribution,
            presented: Vec::new(),
            transcribed: Vec::new(),
            p_null: 0.0,
            len: 0,
        };

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

        if slf.presented.len() != slf.transcribed.len() {
            panic!("Something went wrong :sob:");
        } else {
            slf.len = slf.presented.len();
        }

        slf.p_null = slf.p_null();

        slf
    }

    /// ref. https://dl.acm.org/doi/10.1145/572020.572056
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

    /// ref. https://dl.acm.org/doi/fullHtml/10.1145/3290605.3300866
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
            t_aligned.insert(0, Element::Null);

            // recursive call
            self.alignments(presented, transcribed, d, x - 1, y, p_aligned, t_aligned);
        }

        if y > 0 && d[x][y] == d[x][y - 1] + 1 {
            let (mut p_aligned, mut t_aligned) = (p_aligned.clone(), t_aligned.clone());
            p_aligned.insert(0, Element::Null);
            t_aligned.insert(0, Element::Character(transcribed[y - 1]));

            // recursive call
            self.alignments(presented, transcribed, d, x, y - 1, p_aligned, t_aligned);
        }

        return;
    }

    /// N(presented -> entry)
    fn n<F: Fn(&Element, &Element) -> bool>(&self, f: F) -> usize {
        let mut counter = 0usize;

        self.presented.iter()
            .zip(
                self.transcribed.iter()
            )
            .for_each(|(p, t)| if f(p, t) {
                counter += 1;
            });

        counter
    }

    /// p(i)
    fn p(&self, c: &Element) -> Option<f64> {
        match c {
            Element::Null => Some(self.p_null),
            Element::Character(c) => self.distribution.p(c).cloned()
        }
    }

    /// p(NULL) = p'(NULL)
    fn p_null(&self) -> f64 {
        self.n(|p, _| p == &Element::Null) as f64
            / self.len() as f64
    }

    /// p'(c)
    fn p_dash(&self, c: &Element) -> Option<f64> {
        match c {
            Element::Null => Some(self.p_null),
            c => {
                self.p(c)
                    .map(|p_c| {
                        p_c * (1f64 - self.p_null)
                    })
            }
        }
    }

    /// p_i(j)
    fn p_i_j(&self, i: &Element, j: &Element) -> f64 {
        // insertion error
        match (i, j) {
            (Element::Null, Element::Character(_)) => {
                self.insertion_probability()
                    / self.distribution.map.keys().count() as f64
            }
            (Element::Character(_), Element::Null) => {
                self.omission_probability()
            }
            (Element::Character(p), Element::Character(e)) => {
                if p != e {
                    self.substitution_probability()
                        / (self.distribution.map.keys().count() - 1) as f64
                } else {
                    self.probability_of_correct_entries()
                }
            }
            _ => {
                dbg!(&(i, j));
                unreachable!()
            }
        }
    }

    /// p(i,j)
    fn pij(&self, i: &Element, j: &Element) -> Option<f64> {
        self.p_dash(i)
            .map(|p_dash_i| {
                p_dash_i * self.p_i_j(i, j)
            })
    }

    /// p_j(i)
    fn p_j_i(&self, i: &Element, j: &Element) -> Option<f64> {
        // let extend = vec![Element::Null];
        Some(
            self.pij(i, j)?
                / self.distribution.map.keys()
                .cloned()
                .map(Element::Character)
                // .chain(extend)
                // .filter(|i| !i.is_null() || !j.is_null())
                .map(|i| self.pij(&i, j))
                .fold(Some(0.0), |acc, p| {
                    if acc.is_none() || p.is_none() {
                        None
                    } else {
                        Some(acc.unwrap() + p.unwrap())
                    }
                })?
        )
    }

    /// H_Y(X)
    fn hyx(&self, distribution: &Distribution) -> Option<f64> {
        let elements = distribution.map.keys()
            .cloned()
            .map(Element::Character);

        let is = elements.clone();

        let mut acc = 0.0;

        for i in is {
            let extend = vec![Element::Null];
            let js = elements.clone().chain(extend);

            for j in js {
                if i.is_null() && j.is_null() {
                    continue;
                }
                // dbg!((&i, &j));
                acc += self.pij(&i, &j)?
                    * self.p_j_i(&i, &j)?.log2();
                // dbg!(&acc);
            }
        }

        Some(-acc)
    }

    /// I(X,Y): bits/character
    pub fn ixy(&self, distribution: &Distribution) -> Option<f64> {
        self.hyx(distribution)
            .map(|hyx| distribution.hx() - hyx)
    }

    /// \sum_{i,j} N(i -> j)
    fn len(&self) -> usize {
        self.len
    }
}

impl<'a> OptimalAlignments<'a> {
    /// p(I)
    pub fn insertion_probability(&self) -> f64 {
        let closure = |p: &Element, e: &Element| -> bool {
            p.is_null() && !e.is_null()
        };

        self.n(closure) as f64
            / self.len as f64
    }

    /// p(M)
    pub fn omission_probability(&self) -> f64 {
        let closure = |p: &Element, e: &Element| -> bool {
            !p.is_null() && e.is_null()
        };

        self.n(closure) as f64
            / self.n(|p, _| !p.is_null()) as f64
            * (1f64 - self.insertion_probability())
    }

    /// p(S)
    pub fn substitution_probability(&self) -> f64 {
        let closure = |p: &Element, e: &Element| -> bool {
            !p.is_null() && !e.is_null() && p != e
        };

        self.n(closure) as f64
            / self.n(|p, _| !p.is_null()) as f64
            * (1f64 - self.insertion_probability())
    }

    /// p(C)
    pub fn probability_of_correct_entries(&self) -> f64 {
        let closure = |p: &Element, e: &Element| -> bool {
            !p.is_null() && !e.is_null() && p == e
        };

        self.n(closure) as f64
            / self.n(|p, _| !p.is_null()) as f64
            * (1f64 - self.insertion_probability())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::HashMap;

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

        let distribution = alphabet_distribution();
        let optimal_alignment = OptimalAlignments::new(presented, transcribed, &distribution);
        let answer = OptimalAlignments {
            distribution: &distribution,
            presented: vec![
                Element::Character('q'),
                Element::Character('u'),
                Element::Character('i'),
                Element::Character('c'),
                Element::Null,
                Element::Null,
                Element::Character('k'),
                Element::Character('l'),
                Element::Character('y'),
            ],
            transcribed: vec![
                Element::Character('q'),
                Element::Character('u'),
                Element::Null,
                Element::Character('c'),
                Element::Character('e'),
                Element::Character('h'),
                Element::Character('k'),
                Element::Character('l'),
                Element::Character('y'),
            ],
            p_null: 0.0,
            len: 9,
        };

        assert_eq!(optimal_alignment, answer);
    }

    fn sample_alignments<'a>(distribution: &'a Distribution) -> OptimalAlignments<'a> {
        let presented = "my watch fell in the waterprevailing wind from the east";
        let transcribed = "my wacch fell in waterpreviling wind on the east";

        OptimalAlignments::new(presented, transcribed, distribution)
    }

    #[test]
    fn probabilities_test() {
        let distribution = alphabet_distribution();
        let alignments = sample_alignments(&distribution);

        assert_eq!(alignments.insertion_probability(), 0.0);
        assert_eq!(alignments.omission_probability(), 0.12727272727272726);
        assert_eq!(alignments.substitution_probability(), 0.03636363636363636);
        assert_eq!(alignments.probability_of_correct_entries(), 0.8363636363636363);
    }

    fn alphabet_distribution() -> Distribution {
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

        Distribution { map }
    }

    #[test]
    fn ixy_test() {
        let distribution = alphabet_distribution();

        let alignments = sample_alignments(&distribution);

        // insertion probability
        assert_eq!(alignments.insertion_probability(), 0.0);
        // omission probability
        assert_eq!(alignments.omission_probability(), 0.12727272727272726);
        // substitution probability
        assert_eq!(alignments.substitution_probability(), 0.03636363636363636);
        // correct probability
        assert_eq!(alignments.probability_of_correct_entries(), 0.8363636363636363);

        // H(X)
        assert!((distribution.hx() - 4.090309047790043).abs() < 0.00000000001);

        // H_Y(X)
        assert!(alignments.hyx(&distribution).unwrap() - 0.8515677144377292 < 0.00000000001);

        // I(X,Y): bits/character
        assert!(alignments.ixy(&distribution).unwrap() - 3.238741333352314 < 0.00000000001);
    }
}
