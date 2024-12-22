# Binpack Reader

Rust port of the Stockfish binpack reader from the [C++ version](https://github.com/official-stockfish/Stockfish/blob/tools/src/extra/nnue_data_binpack_format.h).

## Compile

If your machine has the fast BMI2 instruction set (Zen 3+), you should enable the feature flag.

```bash
RUSTFLAGS="-C target-feature=+bmi2" cargo build --release
```

## Usage

```rust
use binpack_reader::reader::training_data_reader::CompressedTrainingDataEntryReader;

fn main() {
    let mut reader = CompressedTrainingDataEntryReader::new(
        "test60-2019-2tb7p.min.high-simple-eval-1k.min-v2.binpack", // path to file
    )
    .unwrap();

    while reader.has_next() {
        let entry = reader.next();

        println!("entry:");
        println!("fen {}", entry.pos.fen());
        println!("uci {:?}", entry.mv.as_uci());
        println!("score {}", entry.score);
        println!("ply {}", entry.ply);
        println!("result {}", entry.result);
        println!("\n");

        // progress percentage
        // let percentage = reader.read_bytes() as f64 / reader.file_size() as f64 * 100.0;
    }
}
```

*If you are doing some counting keep in mind to use a `u64` type for the counter.*

## Performance Comparison

Slightly faster when compiled with bmi2 because of _pdep_u64 trick which is missing in the upstream version.
