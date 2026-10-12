mod create_piece;
mod erreur;
mod fen;
mod notation;
mod plateau;
mod user_interface;
use crate::notation::{get_position_piece, get_position_plateau};
use crate::plateau::creation_du_plateau;
use crate::user_interface::{afficher_plateau, saisie};

fn main() {
    let fen: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";
    let plateau = match creation_du_plateau(&fen) {
        Ok(plateau) => plateau,
        Err(e) => return println!("Erreur : {:?}", e),
    };

    afficher_plateau(&plateau);

    let (depart, arrivee) = saisie::demander_coup();

    println!("{:?}", get_position_piece(&plateau, &depart));
    println!("{:?}", get_position_plateau(&arrivee));
}
