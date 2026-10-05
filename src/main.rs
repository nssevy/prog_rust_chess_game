mod create_piece;
mod erreur;
mod fen;
mod plateau;
mod user_interface;
use crate::erreur::ErreurPlateau;
use crate::plateau::creation_du_plateau;
use crate::user_interface::{afficher_plateau, demander_position, get_position_piece};

fn main() {
    let fen: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";
    let plateau = match creation_du_plateau(&fen) {
        Ok(plateau) => plateau,
        Err(e) => return println!("Erreur : {:?}", e),
    };

    afficher_plateau(&plateau);

    let position_saisis: String = demander_position();

    let position = match get_position_piece(&fen, &position_saisis) {
        Ok(a) => a,
        Err(e) => return println!("Erreur : {:?}", e),
    };

    match plateau.get_piece(&position) {
        Ok(piece) => println!(
            "Vous avez saisis : {} \nLa position sur le plateau est : {:?} \nA cette position il y a la piece : {:?}",
            &position_saisis, &position, piece
        ),
        Err(e) => return println!("{:?}", e),
    };
}
