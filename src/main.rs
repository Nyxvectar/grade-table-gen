/**
 * Author:  Raye Lattice
 * Repo:    table-gen
 * Created: 09/06/2025
 */
mod args;
mod data;
mod render;

use crate::data::data::data_reader;

fn main() {
    match data_reader() {
        Ok(msg) => print!("{:?}", msg),
        Err(err) => println!("{:?}", err),
    }
}
