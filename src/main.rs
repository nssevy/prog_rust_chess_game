mod create_piece;
mod erreur;
mod fen;
mod plateau;
use crate::plateau::{afficher_plateau, creation_du_plateau};

fn main() {
    let plateau = creation_du_plateau("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR");

    match plateau {
        Ok(plateau) => afficher_plateau(&plateau),
        Err(e) => println!("Erreur: {:?}", e),
    }
}
