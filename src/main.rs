use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use rand::Rng;
use rayon::prelude::*;

const NUM_COINS: usize = 100;
const NUM_EXPERIMENTS: usize = 1000;
const NUM_THREADS: usize = 4;

fn main() {
    let counts = Arc::new(Mutex::new(HashMap::new()));

    let chunk_size = NUM_EXPERIMENTS / NUM_THREADS;
    let experiments = vec![chunk_size; NUM_THREADS];

    experiments.par_iter().for_each(|&num_experiments| {
        let mut local_counts = HashMap::new();
        let mut rng = rand::thread_rng();

        for _ in 0..num_experiments {
            let mut state = String::new();

            // Initialize all coins as tails
            for _ in 0..NUM_COINS {
                state.push('T');
            }

            for _ in 0..NUM_COINS {
                let coin: u8 = rng.gen_range(0..=1);
                if coin == 0 {
                    // Flip a coin from tails to heads
                    let index = rng.gen_range(0..NUM_COINS);
                    state.replace_range(index..index + 1, "H");
                }
            }

            let counter = local_counts.entry(state).or_insert(0);
            *counter += 1;
        }

        // Merge local_counts into the global counts
        let mut counts = counts.lock().unwrap();
        for (state, count) in local_counts {
            *counts.entry(state).or_insert(0) += count;
        }
    });

    let total_experiments = experiments.iter().sum();
    let entropy = calculate_entropy(&counts.lock().unwrap(), total_experiments);
    println!("Entropy: {:.4}", entropy);
}

fn calculate_entropy(counts: &HashMap<String, usize>, total: usize) -> f64 {
    let mut entropy = 0.0;

    for count in counts.values() {
        let probability = *count as f64 / total as f64;
        entropy -= probability * probability.log2();
    }

    entropy
}