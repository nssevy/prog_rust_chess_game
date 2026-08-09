#![allow(dead_code)]
mod create_piece;
mod plateau;
pub mod fen;

//use crate::create_piece::Piece;
use crate::fen::Fen;

fn main(){
    //rnbqkbnr
    /*let base_fen: &str = "rnbqkbn/RNBQKBN";

    let fen = fen(base_fen); // ['r', 'n', 'b', 'q', 'k', 'b', 'n', '/', 'R', 'N', 'B', 'Q', 'K', 'B', 'N']

    // Permet de reconnaitre le type de piece dans le tableau (echequier)
    for f in fen.iter(){
        let p = Piece::construction_de_piece(f);
        println!("{:?}", p);
    }*/

    Fen::who_is_type_of_fen("rnbqkbn/RNBQKBN");

}