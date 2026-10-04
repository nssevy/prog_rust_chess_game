mod create_piece;
mod erreur;
mod fen;
mod plateau;
use crate::plateau::{afficher_plateau, creation_du_plateau};

fn main() {
    let fen: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";
    let plateau = match creation_du_plateau(&fen) {
        Ok(plateau) => plateau,
        Err(e) => return println!("Erreur : {:?}", e),
    };

    afficher_plateau(&plateau);
    match plateau.get_piece(plateau.position_piece(0, 1)) {
        Ok(a) => println!("{:?}", a),
        Err(e) => return println!("{:?}", e),
    };
}
