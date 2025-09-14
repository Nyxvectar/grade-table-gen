/**
 * Author:  Raye Lattice
 * Repo:    table-gen
 * Created: 09/14/2025
 */

pub fn data_calc(arg: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    if arg.is_empty() {
        vec![]
    } else {
        let vec_amount = arg.len();
        let vec_length = arg[0].len();
        let mut lattice: Vec<Vec<i32>> = vec![vec![0; vec_amount]; vec_length];
        for i in 0..vec_length {
            for k in 0..vec_amount {
                if i < arg[k].len() {
                    lattice[i][k] = arg[k][i];
                }
            }
        }
        for i in 1..vec_length {
            let temp = lattice[i][0];
            lattice[i].insert(0, temp);
            lattice[i].insert(0, temp);
            for k in 0..vec_amount-3 {
                lattice[i][k] = calc_average(lattice[i][k],lattice[i][k+1],lattice[i][k+2])
            }
            lattice[i].pop();
            lattice[i].pop();
        }
        lattice
    }
}

fn calc_average(a: i32, b: i32, c: i32) -> i32 {
    (a + b + c) / 3
}
