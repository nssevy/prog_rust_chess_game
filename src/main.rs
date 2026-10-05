mod create_piece;
mod erreur;
mod fen;
mod plateau;
use crate::erreur::ErreurPlateau;
use crate::fen::extraire_colonnes_piece_fen;
use crate::plateau::{Position, afficher_plateau, creation_du_plateau};
use std::io::{self, Write};

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_que_la_fen_initial_avec_bc8_on_optient_la_position_du_fou() {
        let fen: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";
        assert_eq!(
            get_position_piece(fen, "bc8"),
            Ok(Position {
                colonne: 0,
                ligne: 2
            })
        )
    }
}

fn get_position_piece(fen: &str, position_saisis: &str) -> Result<Position, ErreurPlateau> {
    let mut chars = position_saisis.chars();

    let tuple = match (chars.next(), chars.next(), chars.next()) {
        (Some(piece_en_lettre), Some(colonne), Some(ligne)) => (piece_en_lettre, colonne, ligne),
        _ => return Err(ErreurPlateau::ChaineTropCourte),
    };

    let ligne_en_chiffre: usize = ('a'..='h')
        .contains(&tuple.1)
        .then(|| ((tuple.1 as u32) - ('a' as u32)) as usize)
        .ok_or(ErreurPlateau::ChaineTropLongue)?;

    let tableau = extraire_colonnes_piece_fen(&fen, tuple.0);

    let ligne = if tableau.contains(&ligne_en_chiffre) {
        Ok(ligne_en_chiffre)
    } else {
        return Err(ErreurPlateau::PasUnChiffre);
    };

    let mut colonne: usize = tuple
        .2
        .to_digit(10)
        .map(|chiffre| chiffre as usize)
        .ok_or(ErreurPlateau::ChaineTropLongue)?;

    dbg!(&colonne);

    colonne = 8 - colonne; // Pour Obtenir la l'index de la ligne dans le plateau (inversion)

    Ok(Position {
        colonne: colonne,
        ligne: ligne?,
    })
}

fn main() {
    let fen: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";
    let plateau = match creation_du_plateau(&fen) {
        Ok(plateau) => plateau,
        Err(e) => return println!("Erreur : {:?}", e),
    };

    /* L'idée est de venir tester un char (exemple: r ou b) avec toute la fen et donner la sa ligne et sa colonne à partir de sa posiiton dans la fen.
    Un compteur qui verifie combien de / il y a avant la piece + la position de la piece dans la fen.

    Avec bn b apparait 2 fois,
    */

    afficher_plateau(&plateau);

    match plateau.get_piece(plateau.position_piece(0, 0)) {
        Ok(a) => println!("\n{:?}", a),
        Err(e) => return println!("{:?}", e),
    };

    print!("Jouer une position : ");
    io::stdout().flush().unwrap();

    // 2. Créer une variable mutable pour stocker la saisie
    let mut position_saisie = String::new();

    // 3. Lire la saisie dans le terminal
    io::stdin()
        .read_line(&mut position_saisie)
        .expect("Échec de la lecture de la ligne");

    let position_saisis = match get_position_piece(&fen, &position_saisie) {
        Ok(a) => a,
        Err(e) => return println!("Erreur : {:?}", e),
    };

    dbg!(&position_saisis);

    match plateau.get_piece(position_saisis) {
        Ok(a) => println!("\n{:?}", a),
        Err(e) => return println!("{:?}", e),
    };
}
