mod create_piece;
mod erreur;
mod fen;
mod notation;
mod plateau;
mod user_interface;
use crate::notation::analyser_coup;
use crate::plateau::creation_du_plateau;
use crate::user_interface::{afficher_plateau, saisie};

fn main() {
    let fen: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";
    let plateau = match creation_du_plateau(&fen) {
        Ok(plateau) => plateau,
        Err(e) => return println!("Erreur : {:?}", e),
    };

    afficher_plateau(&plateau);

    /*let (depart, arrivee) = saisie::demander_coup();

    println!("{:?}", get_position_piece(&plateau, &depart));
    println!("{:?}", get_position_plateau(&arrivee));*/

    let (depart, arrivee) = saisie::demander_coup();

    match analyser_coup(&plateau, &depart, &arrivee) {
        Ok(_) => {
            println!("Deplacer piece"); // déplacer la pièce
        }
        Err(e) => println!("Coup invalide : {:?}", e),
    }
}
