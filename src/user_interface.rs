use crate::ErreurPlateau;
use crate::create_piece::{Couleur, Piece, TypePiece};
use crate::fen::extraire_colonnes_piece_fen;
use crate::plateau::{CaseType, Plateau, Position};
use std::io::{self, Write};

pub fn affichage_sur_plateau(piece: &Piece) -> &str {
    match (piece.couleur, piece.type_piece) {
        (Couleur::Blanc, TypePiece::Roi) => "K",
        (Couleur::Blanc, TypePiece::Dame) => "Q",
        (Couleur::Blanc, TypePiece::Cavalier) => "N",
        (Couleur::Blanc, TypePiece::Tour) => "R",
        (Couleur::Blanc, TypePiece::Fou) => "B",
        (Couleur::Blanc, TypePiece::Pion) => "P",
        //___
        (Couleur::Noir, TypePiece::Roi) => "k",
        (Couleur::Noir, TypePiece::Dame) => "q",
        (Couleur::Noir, TypePiece::Cavalier) => "n",
        (Couleur::Noir, TypePiece::Tour) => "r",
        (Couleur::Noir, TypePiece::Fou) => "b",
        (Couleur::Noir, TypePiece::Pion) => "p",
    }
}

pub fn afficher_plateau(cases: &Plateau) {
    let mut i: usize = 0;
    let mut nombre_de_gauche: usize = 8;

    for _ in 0..8 {
        println!("");
        print!("{nombre_de_gauche}    ");
        for piece in cases[i] {
            match piece {
                CaseType::Piece(piece) => print!("{} ", affichage_sur_plateau(&piece)),
                CaseType::Vide => print!(". "),
            }
        }
        i += 1;
        nombre_de_gauche -= 1;
    }
    print!("\n");
    print!("\n     ");
    for lettre in 'a'..='h' {
        print!("{lettre} ");
    }
}

/* L'idée est de venir tester un char (exemple: r ou b) avec toute la fen et donner la sa ligne et sa colonne à partir de sa posiiton dans la fen.
Un compteur qui verifie combien de / il y a avant la piece + la position de la piece dans la fen.

Avec bn b apparait 2 fois,
*/
pub fn get_position_piece(fen: &str, position_saisis: &str) -> Result<Position, ErreurPlateau> {
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

    colonne = 8 - colonne; // Pour Obtenir la l'index de la ligne dans le plateau (inversion)

    Ok(Position {
        colonne: colonne,
        ligne: ligne?,
    })
}

pub fn demander_position() -> String {
    print!("\nJouer une position : ");
    io::stdout().flush().unwrap();

    let mut position_saisie = String::new();
    io::stdin()
        .read_line(&mut position_saisie)
        .expect("Échec de la lecture");

    print!("\x1B[A\x1B[2K\x1B[A\x1B[2K");
    io::stdout().flush().unwrap();

    position_saisie.trim().to_string()
}

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
