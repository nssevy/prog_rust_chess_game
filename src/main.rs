mod create_piece;
mod erreur;
mod fen;
mod plateau;
mod user_interface;
use crate::plateau::creation_du_plateau;
use crate::user_interface::{afficher_plateau, demander_saisie, get_position_piece};

fn main() {
    let fen: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";
    let plateau = match creation_du_plateau(&fen) {
        Ok(plateau) => plateau,
        Err(e) => return println!("Erreur : {:?}", e),
    };

    afficher_plateau(&plateau);

    let questions = ["Séléctionner une cases : ", "Case de fin : "];
    let mut reponses = Vec::new();

    for question in questions {
        let saisie = demander_saisie(question);
        reponses.push(saisie);
    }

    println!("{:?}", get_position_piece(&plateau, &reponses[0]));
    println!("{:?}", get_position_piece(&plateau, &reponses[1]));
}
