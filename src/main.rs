mod create_piece;
mod erreur;
mod plateau;
use crate::plateau::{afficher_plateau, creation_du_plateau};

fn main() {
    let plateau = creation_du_plateau("rnbqkbnr/8/8/8/8/8/8/RNBQKBNR");

    match plateau {
        Ok(plateau) => afficher_plateau(&plateau),
        Err(e) => println!("Erreur: {:?}", e),
    }
}
