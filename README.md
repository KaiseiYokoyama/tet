# Text Entry Throughput
Text entry throughput [introduced by Minguri et al.](https://dl.acm.org/doi/fullHtml/10.1145/3290605.3300866)
is a text entry method-independent throughput metric based on Shannon information theory.

This crate is a third-party implementation of TET.

## TL;DR
```rust
use tet::TextEntryThroughput;

// A preset for English alphabet is provided.
// An explanation for other language texts is wrote on later.
let tet = TextEntryThroughput::alphabet_letter_distribution();

let presented_text = "my watch fell in the waterprevailing wind from the east";
let transcribed_text = "my wacch fell in waterpreviling wind on the east";
let s = std::time::Duration::from_secs(12); // 4 characters per second

let throughput = tet.calc(presented_text, transcribed_text, s).unwrap();
assert!((throughput - 12.954965333409255).abs() < 0.0001);
```

## Usage
### Get distribution
First, prepare a distribution of characters to get entropy `H(X)` from source.
```rust
use tet::{Frequencies, Distribution};

let mut frequency = Frequencies::new();

// get frequency of each character
let source = "large and appropriate text is recommended";
source.chars()
    .for_each(|c| {
        frequency.record(c.clone());
    });

// normalize frequency to get distribution
let distribution = Distribution::new(frequency);
```

### Compute TET
```rust
// now you can calculate TET! :+1:
let tet = TextEntryThroughput::new(distribution);

// Of course, you can use also multibyte characters
// ref. https://doc.rust-lang.org/std/primitive.char.html
let (presented, transcribed) = ("うまぴょい", "うまぽい");
let s = std::time::Duration::from_secs(2); // 4 characters per minute

// Text Entry Throughput (bits/second)
let throughput = tet.calc(presented, transcribed, s).unwrap();
```