#![allow(dead_code)] // Nie les variables, struct, enums... non utiliser
#![allow(unused_imports)] // Nie les imports non utiliser
mod create_piece;
pub mod fen;
mod plateau;
use crate::plateau::{afficher_plateau, plateau};

use crate::create_piece::Piece;
use crate::fen::Fen;

fn main() {
    let plateau = plateau("rnbqkbn/8");
    println!("{:?}", afficher_plateau(&plateau));
}
