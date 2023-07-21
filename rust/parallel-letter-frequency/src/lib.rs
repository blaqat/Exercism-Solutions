/*
test bench_large_parallel   ... bench:     309,023 ns/iter (+/- 52,760)
test bench_large_sequential ... bench:     239,355 ns/iter (+/- 50,222)
test bench_small_parallel   ... bench:       6,751 ns/iter (+/- 811)
test bench_small_sequential ... bench:       8,282 ns/iter (+/- 653)
test bench_tiny_parallel    ... bench:         117 ns/iter (+/- 19)
test bench_tiny_sequential  ... bench:          71 ns/iter (+/- 8)
 */

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

// Function to calculate frequency of characters in a given input array of strings.
// It uses multi-threading for processing if the length of the input is greater than 500.
pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let mut freq_map = HashMap::new();

    // If input is empty, return an empty frequency map
    if input.is_empty() {
        return freq_map;
    }

    // Convert input to a vector of owned strings
    let input = input.iter().map(|s| s.to_string()).collect::<Vec<String>>();

    match input.len() {
        0 => freq_map, // If the input is still empty after conversion, return the empty frequency map
        len if len < 500 => {
            // If the length of the input is less than 500, process it using the single-threaded method
            frequency_simple(&input, &mut freq_map);
            freq_map
        }
        _ => {
            // If the length of the input is 500 or more, process it using multiple threads.
            let freq_map = Arc::new(Mutex::new(HashMap::new())); // Create a new thread-safe frequency map
            let mut threads = Vec::with_capacity(worker_count); // Preallocate space for the worker threads

            // Divide the input into chunks and spawn a thread for each chunk.
            input.chunks(1 + input.len() / worker_count).for_each(|c| {
                let m = Arc::clone(&freq_map); // Clone the Arc to the frequency map
                let c = c.to_vec(); // Clone the current chunk
                let handle = thread::spawn(move || {
                    let mut locked_map = m.lock().unwrap(); // Lock the frequency map
                    frequency_simple(&c, &mut locked_map); // Process the chunk
                });
                threads.push(handle); // Store the handle to the thread
            });

            // Wait for all threads to finish
            for t in threads {
                t.join().unwrap();
            }

            // Lock the frequency map and clone its contents.
            let x = freq_map.lock().unwrap().to_owned();
            x
        }
    }
}

// Function to calculate frequency of characters in a given string slice.
// It directly modifies a given frequency map.
pub fn frequency_simple(string: &[String], map: &mut HashMap<char, usize>) {
    string.iter().for_each(|s| {
        s.chars()
            // Filter alphabetic characters and convert them to lower case.
            .filter_map(|c| {
                if c.is_alphabetic() {
                    Some(c.to_ascii_lowercase())
                } else {
                    None
                }
            })
            // For each character, increment its count in the map. If it doesn't exist, add it with a count of 1.
            .for_each(|c| {
                map.entry(c)
                    .and_modify(|count| {
                        *count += 1;
                    })
                    .or_insert(1);
            })
    })
}
