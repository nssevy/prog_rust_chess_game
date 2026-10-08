use crate::create_piece::Piece;
use crate::erreur::{ErreurPlateau, ErreurSaisie};
use crate::plateau::{CaseType, Plateau, Position, TAILLE_PLATEAU};

fn convertit_les_colonne_en_lettre_en_chiffre(lettre: char) -> Result<usize, ErreurSaisie> {
    let chiffre = ('a'..='h')
        .contains(&lettre)
        .then(|| ((lettre as u32) - ('a' as u32)) as usize)
        .ok_or(ErreurSaisie::RangeeInvalide)?;
    Ok(chiffre)
}

fn extraire_colonnes_piece(cases: &Plateau, piece: char) -> Result<Vec<usize>, ErreurPlateau> {
    let piece_extraite: Piece = Piece::construction_de_piece(&piece)?;
    let mut tableau_colonne_piece: Vec<usize> = vec![];

    let mut i: usize = 0;

    for _ in 0..8 {
        for (colonne, piece) in cases[i].into_iter().enumerate() {
            match piece {
                CaseType::Piece(p) if p == piece_extraite => tableau_colonne_piece.push(colonne),
                _ => {}
            }
        }
        i += 1;
    }

    Ok(tableau_colonne_piece)
}

fn get_colonne(
    cases: &Plateau,
    colonne_en_lettre: char,
    piece_en_lettre: char,
) -> Result<usize, ErreurSaisie> {
    let colonne_en_chiffre: usize = convertit_les_colonne_en_lettre_en_chiffre(colonne_en_lettre)?;

    let tableau_de_colonne = extraire_colonnes_piece(cases, piece_en_lettre)?;

    match tableau_de_colonne.contains(&colonne_en_chiffre) {
        true => return Ok(colonne_en_chiffre),
        false => {
            return Err(ErreurSaisie::PieceAbsenteDeLaColonne {
                piece: piece_en_lettre,
                colonne_attendue: colonne_en_chiffre,
                colonnes_trouvees: tableau_de_colonne,
            });
        }
    };
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

pub fn get_position_piece(
    cases: &Plateau,
    position_saisis: &str,
) -> Result<Position, ErreurSaisie> {
    let mut chars = position_saisis.chars();

    let tuple = match (chars.next(), chars.next(), chars.next(), chars.next()) {
        (Some(piece_en_lettre), Some(colonne), Some(rangee), None) => {
            (piece_en_lettre, colonne, rangee)
        }
        (_, _, _, Some(_)) => return Err(ErreurSaisie::TropLongue),
        _ => return Err(ErreurSaisie::TropCourte),
    };

    let colonne: Result<usize, ErreurSaisie> = get_colonne(&cases, tuple.1, tuple.0);

    let rangee: usize = rangee_en_index(range_en_chiffre(tuple.2)?);

    let position = Position {
        colonne: colonne.clone()?,
        rangee: rangee,
    };

    let piece_extraite: Piece = Piece::construction_de_piece(&tuple.0)?;

    match cases.get_piece(&position) {
        Ok(piece) if piece == piece_extraite => return Ok(position),
        Ok(piece_trouvee) => {
            return Err(ErreurSaisie::MauvaisePiece {
                piece_attendu: piece_extraite,
                piece_trouve: piece_trouvee, // Maintenant accessible !
                colonne: colonne?,
                rangee: rangee,
            });
        }
        Err(e) => return Err(e.into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::creation_du_plateau;

    #[test]
    fn dans_la_fen_initiale_la_lettre_n_apparait_2_fois_dans_le_plateau_au_colonne_1_et_6() {
        let fen: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";
        let plateau = match creation_du_plateau(&fen) {
            Ok(plateau) => plateau,
            Err(e) => return println!("Erreur : {:?}", e),
        };
        let tableau = vec![1, 6];
        assert_eq!(tableau, extraire_colonnes_piece(&plateau, 'n').unwrap())
    }

    #[test]
    fn la_lettre_p_apparait_8_fois_dans_le_plateau() {
        let fen: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";
        let plateau = match creation_du_plateau(&fen) {
            Ok(plateau) => plateau,
            Err(e) => return println!("Erreur : {:?}", e),
        };
        assert_eq!(8, extraire_colonnes_piece(&plateau, 'p').unwrap().len())
    }

    #[test]
    fn la_lettre_b_apparait_2_fois_dans_le_plateau() {
        let fen: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";
        let plateau = match creation_du_plateau(&fen) {
            Ok(plateau) => plateau,
            Err(e) => return println!("Erreur : {:?}", e),
        };
        assert_eq!(2, extraire_colonnes_piece(&plateau, 'b').unwrap().len())
    }

    #[test]
    fn check_bc8_equivaut_à_position_colonne_2_et_rangée_0() {
        let fen: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";
        let plateau = match creation_du_plateau(&fen) {
            Ok(plateau) => plateau,
            Err(e) => return println!("Erreur : {:?}", e),
        };
        assert_eq!(
            get_position_piece(&plateau, "bc8"),
            Ok(Position {
                colonne: 2,
                rangee: 0
            })
        )
    }
}
