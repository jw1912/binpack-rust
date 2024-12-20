# Binpack Reader

Rust port of the Stockfish binpack reader from the [C++ version](https://github.com/official-stockfish/Stockfish/blob/tools/src/extra/nnue_data_binpack_format.h).

## Compile

If your machine has the fast BMI2 instruction set (Zen 3+), you should enable the feature flag.

```bash
RUSTFLAGS="-C target-feature=+bmi2" cargo build --release
```

## Performance Comparison

Slightly faster when compiled with bmi2 because of _pdep_u64 trick which is missing in the upstream version.
