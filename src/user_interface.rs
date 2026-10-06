use crate::create_piece::{Couleur, Piece, TypePiece};
use crate::erreur::ErreurSaisie;
use crate::fen::extraire_colonnes_piece_fen;
use crate::plateau::{CaseType, Plateau, Position, TAILLE_PLATEAU};
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

fn convertit_les_colonne_en_lettre_en_chiffre(lettre: char) -> Result<usize, ErreurSaisie> {
    let chiffre = ('a'..='h')
        .contains(&lettre)
        .then(|| ((lettre as u32) - ('a' as u32)) as usize)
        .ok_or(ErreurSaisie::RangeeInvalide)?;
    Ok(chiffre)
}

/*
L'idée derrière get_colonne() est que à partir de bc8 qui veut dire -> Bishop(fou) en colonne c rangée 8.

Et de venir récuperer la piece -> b et la colonne -> c afin de vérifié une correspondance.

Si la fonction extraire_colonnes_piece_fen() retourne un tableau de [2, 4], alors ca veut dire qu'il y a bien un fou qui est sur la colonne c(2) donc la colonne est bien égale à 2.

En revanche qui la fonction extraire_colonnes_piece_fen() retourne un tableau de [3, 7] ca veut dire qu'il n'y a pas de fou sur la colonne c(2) donc l'utilisateur à saisis une position invalide.
*/

fn get_colonne(
    fen: &str,
    colonne_en_lettre: char,
    piece_en_lettre: char,
) -> Result<usize, ErreurSaisie> {
    let colonne_en_chiffre: usize = convertit_les_colonne_en_lettre_en_chiffre(colonne_en_lettre)?;
    let tableau_de_colonne = extraire_colonnes_piece_fen(&fen, piece_en_lettre);

    let colonne = if tableau_de_colonne.contains(&colonne_en_chiffre) {
        Ok(colonne_en_chiffre)
    } else {
        return Err(ErreurSaisie::PieceAbsenteDeLaColonne {
            piece: piece_en_lettre,
            colonne_attendue: colonne_en_chiffre,
            colonnes_trouvees: tableau_de_colonne,
        });
    };

    Ok(colonne?)
}

fn range_en_chiffre(chiffre_en_char: char) -> Result<usize, ErreurSaisie> {
    chiffre_en_char
        .to_digit(10)
        .map(|chiffre| chiffre as usize)
        .ok_or(ErreurSaisie::RangeeInvalide)
}

fn rangee_en_index(rangee_en_lettre: usize) -> usize {
    TAILLE_PLATEAU - rangee_en_lettre
}

pub fn get_position_piece(fen: &str, position_saisis: &str) -> Result<Position, ErreurSaisie> {
    let mut chars = position_saisis.chars();

    let tuple = match (chars.next(), chars.next(), chars.next(), chars.next()) {
        (Some(piece_en_lettre), Some(colonne), Some(rangee), None) => {
            (piece_en_lettre, colonne, rangee)
        }
        (_, _, _, Some(_)) => return Err(ErreurSaisie::TropLongue),
        _ => return Err(ErreurSaisie::TropCourte),
    };

    let colonne: Result<usize, ErreurSaisie> = get_colonne(&fen, tuple.1, tuple.0);

    let rangee: usize = rangee_en_index(range_en_chiffre(tuple.2)?);

    Ok(Position {
        colonne: colonne?,
        rangee: rangee,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_bc8_equivaut_à_position_colonne_2_et_rangée_o() {
        let fen: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";
        assert_eq!(
            get_position_piece(fen, "bc8"),
            Ok(Position {
                colonne: 2,
                rangee: 0
            })
        )
    }
}
