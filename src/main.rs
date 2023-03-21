// Import necessary modules and libraries
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use rand::Rng;
use rayon::prelude::*;

// Set constants for the number of coins, experiments, and threads
const NUM_COINS: usize = 10000;
const NUM_EXPERIMENTS: usize = 100;
const NUM_THREADS: usize = 100;

fn main() {
    // Create a shared, mutable, and thread-safe HashMap to store counts of each unique state
    let counts = Arc::new(Mutex::new(HashMap::new()));

    // Calculate the chunk size for dividing experiments among threads
    let chunk_size = NUM_EXPERIMENTS / NUM_THREADS;
    // Create a vector of chunk sizes for each thread
    let experiments = vec![chunk_size; NUM_THREADS];

    // Use rayon's parallel iterator to process each chunk of experiments concurrently
    experiments.par_iter().for_each(|&num_experiments| {
        // Create a local HashMap to store counts for the current thread
        let mut local_counts = HashMap::new();
        // Create a random number generator for the current thread
        let mut rng = rand::thread_rng();

        // Perform experiments within the current thread
        for _ in 0..num_experiments {
            let mut state = String::new();

            // Initialize all coins as tails
            for _ in 0..NUM_COINS {
                state.push('T');
            }

            // Randomly flip coins
            for _ in 0..NUM_COINS {
                let coin: u8 = rng.gen_range(0..=1);
                if coin == 0 {
                    // If the generated random number is 0, flip a coin from tails to heads
                    let index = rng.gen_range(0..NUM_COINS);
                    state.replace_range(index..index + 1, "H");
                }
            }

            // Update the local_counts HashMap with the new state or increment the count if it already exists
            let counter = local_counts.entry(state).or_insert(0);
            *counter += 1;
        }

        // Merge local_counts from the current thread into the global counts HashMap
        let mut counts = counts.lock().unwrap();
        for (state, count) in local_counts {
            *counts.entry(state).or_insert(0) += count;
        }
    });

    // Calculate the total number of experiments
    let total_experiments = experiments.iter().sum();
    // Calculate the entropy using the global counts HashMap and total number of experiments
    let entropy = calculate_entropy(&counts.lock().unwrap(), total_experiments);
    // Print the calculated entropy
    println!("Entropy: {:.4}", entropy);
}

// Function to calculate the entropy given the state counts and the total number of experiments
fn calculate_entropy(counts: &HashMap<String, usize>, total: usize) -> f64 {
    // Initialize the entropy variable
    let mut entropy = 0.0;

    // Iterate through the counts of each unique state
    for count in counts.values() {
        // Calculate the probability of the current state
        let probability = *count as f64 / total as f64;
        // Update the entropy using the probability and its logarithm in base 2
        entropy -= probability * probability.log2();
    }

    // Return the calculated entropy
    entropy
}