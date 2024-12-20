use std::io::Write;

use binpack_reader::reader::training_data_reader::CompressedTrainingDataEntryReader;

fn main() {
    let mut reader = CompressedTrainingDataEntryReader::new(
        // "/mnt/g/stockfish-data/test80-2024/test80-2024-06-jun-2tb7p.min-v2.v6.binpack",
        "/mnt/g/stockfish-data/dual-nnue/hse-v1/leela96-filt-v2.min.high-simple-eval-1k.min-v2.binpack",
        // "/mnt/g/stockfish-data/dual-nnue/hse-v1/test60-2019-2tb7p.min.high-simple-eval-1k.min-v2.binpack",
        // "/mnt/g/stockfish-data/ep1.binpack",
    )
    .unwrap();

    let mut count: u64 = 0;
    let mut stats: u64 = 0;

    debug_assert!(false);

    let t0 = std::time::Instant::now();

    while reader.has_next() {
        let entry = reader.next();

        count += 1;

        stats += entry.ply as u64;

        // println!("entry:");
        // println!("{}", entry.pos.fen());
        // println!("{:?}", entry.mv.to_uci());
        // println!("score {}", entry.score);
        // println!("ply {}", entry.ply);
        // println!("result {}", entry.result);
        // println!("\n");

        // println!("count: {}", count);

        if count % 10000000 == 0 {
            let t1 = std::time::Instant::now();
            let elapsed = t1.duration_since(t0).as_millis() + 1;
            print!(
                "count: {} elapsed: {} entries/s: {}\r",
                count,
                elapsed,
                (count * 1000) as u128 / elapsed
            );
            let _ = std::io::stdout().flush();
        }
    }

    println!("count: {}", count);
    println!("stats: {}", stats);
}
