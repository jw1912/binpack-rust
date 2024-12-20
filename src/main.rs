use binpack_reader::reader::training_data_reader::{
    CompressedTrainingDataEntryReader, TrainingDataEntry,
};

fn main() {
    let mut reader = CompressedTrainingDataEntryReader::new(
        // "/mnt/g/stockfish-data/test80-2024/test80-2024-06-jun-2tb7p.min-v2.v6.binpack",
        "/mnt/g/stockfish-data/dual-nnue/hse-v1/leela96-filt-v2.min.high-simple-eval-1k.min-v2.binpack",
        // "/mnt/g/stockfish-data/dual-nnue/hse-v1/test60-2019-2tb7p.min.high-simple-eval-1k.min-v2.binpack",
        // "/mnt/g/stockfish-data/ep1.binpack",
    )
    .unwrap();

    let mut count: u64 = 0;

    debug_assert!(false);

    while reader.has_next() {
        let _entry = reader.next();

        count += 1;

        // println!("entry:");
        // println!("{}", entry.pos.fen());
        // println!("{:?}", entry.mv.to_uci());
        // println!("score {}", entry.score);
        // println!("ply {}", entry.ply);
        // println!("result {}", entry.result);
        // println!("\n");

        // println!("count: {}", count);

        if count % 100000 == 0 {
            // println!("count: {}", count);
            print!("count: {}\r", count);
        }
    }
}
