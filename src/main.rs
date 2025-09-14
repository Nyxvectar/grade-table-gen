/**
 * Author:  Raye Lattice
 * Repo:    table-gen
 * Created: 09/06/2025
 */
mod args;
mod data;
mod render;

use crate::data::data::data_reader;
use crate::data::calc::data_calc;

fn main() {
    let mut table: Vec<Vec<i32>> = vec![];
    match data_reader() { 
        Ok(msg) => table= data_calc(msg),
        Err(err) => println!("{:?}", err)
    }
    println!("{:?}", table)
}
