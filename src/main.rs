mod create_piece;
mod erreur;
mod fen;
mod notation;
mod partie;
mod plateau;
mod user_interface;
use crate::partie::lancer_partie;
use crate::plateau::Plateau;
use crate::plateau::creation_du_plateau;

fn main() {
    let fen: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";
    let mut plateau: Plateau = match creation_du_plateau(&fen) {
        Ok(plateau) => plateau,
        Err(e) => return println!("Erreur : {:?}", e),
    };
    lancer_partie(&mut plateau);
}
