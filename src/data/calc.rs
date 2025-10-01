/**
 * Author:  Raye Lattice
 * Repo:    table-gen
 * Created: 09/14/2025
 */

pub fn data_calc(input: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    if input.is_empty() {
        return vec![];
    }

    let row_count = input.len();
    let col_count = input[0].len();
    let mut lattice: Vec<Vec<i32>> = (0..col_count)
        .map(|col_idx| {
            (0..row_count)
                .map(|row_idx| {
                    input
                        .get(row_idx)
                        .and_then(|row| row.get(col_idx))
                        .copied()
                        .unwrap_or(0)
                })
                .collect()
        })
        .collect();

    for row in &mut lattice[1..] {
        let original = row.clone();
        let elements_to_process = row_count - 2;
        for k in 0..elements_to_process {
            let a = if k < 2 { original[0] } else { original[k - 2] };
            let b = if k < 1 { original[0] } else { original[k - 1] };
            let c = original[k];
            row[k] = calc_average(a, b, c);
        }
    }
    lattice
}
fn calc_average(a: i32, b: i32, c: i32) -> i32 {
    (a + b + c) / 3
}
