#![allow(dead_code)] // Nie les variables, struct, enums... non utiliser
#![allow(unused_imports)] // Nie les imports non utiliser
mod create_piece;
mod plateau;
pub mod fen;

use crate::create_piece::Piece;
use crate::fen::Fen;

fn main(){
    //Fen::who_is_type_of_fen("rnbqkbn/RNBQKBN");
    //let roi = Piece::construction_de_piece(&'K');
    //dbg!(roi);
}