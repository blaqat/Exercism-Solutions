use std::collections::HashMap;
use std::ops::Range;

// Defines the 8 possible directions of neighbors around a mine
const NEIGHBOR_INDECIES: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

// Updates the HashMap for a single mine
fn update_neighbors(
    minefield: &&[&str],
    update_map: &mut HashMap<usize, String>,
    r: usize,
    c: usize,
    r_bounds: Range<usize>,
    c_bounds: Range<usize>,
) {
    let valid_neighbors = NEIGHBOR_INDECIES
        .iter()
        .map(|(nr, nc)| ((*nr + r as isize) as usize, (*nc + c as isize) as usize))
        .filter(|(nr, nc)| r_bounds.contains(nr) && c_bounds.contains(nc));

    for (r, char_index) in valid_neighbors {
        // Extract the row from the map, or the original minefield if not present in the map
        let mut r_val = update_map
            .get(&r)
            .unwrap_or(&minefield[r].to_string())
            .to_string();

        let index_range = char_index..char_index + 1;
        let c_val: &str = r_val.get(index_range.clone()).unwrap();

        // Increment the count if the character is a number, set to 1 if empty, or leave it if it's a mine
        let new_c_val = match c_val.parse::<i32>() {
            Ok(c_val) => (c_val + 1).to_string(),
            Err(_) => if c_val == "*" { c_val } else { "1" }.to_string(),
        };

        // Replace the old value in the row string with the new value
        r_val.replace_range(index_range, new_c_val.as_str());

        update_map.insert(r, r_val.to_owned());
    }
}

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let rows = minefield.len();
    let cols = if rows == 0 { 0 } else { minefield[0].len() };

    let mut update_map: HashMap<usize, String> = HashMap::new();

    for (row, r_val) in minefield.iter().enumerate() {
        r_val
            .as_bytes()
            .iter()
            .enumerate()
            .for_each(|(col, &c_val)| {
                // If it's a mine, update the neighbors
                if c_val == b'*' {
                    update_neighbors(&minefield, &mut update_map, row, col, 0..rows, 0..cols);
                }
            });
    }

    // Update the minefield via the update_map
    minefield
        .iter()
        .enumerate()
        .map(|(r, &row)| update_map.get(&r).unwrap_or(&row.to_string()).to_string())
        .collect()
}
