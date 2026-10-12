use crate::create_piece::Piece;
use crate::erreur::{ErreurPlateau, ErreurSaisie};
use crate::plateau::{CaseType, Plateau, Position, TAILLE_PLATEAU};

fn convertit_les_colonnes_en_lettre_en_chiffre(lettre: char) -> Result<usize, ErreurSaisie> {
    if !('a'..='h').contains(&lettre) {
        return Err(ErreurSaisie::ColonneInvalide(lettre));
    }
    let chiffre = ((lettre as u32) - ('a' as u32)) as usize;
    Ok(chiffre)
}

fn extraire_colonnes_piece(plateau: &Plateau, piece: char) -> Result<Vec<usize>, ErreurPlateau> {
    let piece_extraite: Piece = Piece::construction_de_piece(&piece)?;
    let mut tableau_colonne_piece: Vec<usize> = vec![];

    let mut i: usize = 0;

    for _ in 0..8 {
        for (colonne, piece) in plateau[i].into_iter().enumerate() {
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
    plateau: &Plateau,
    colonne_en_lettre: char,
    piece_en_lettre: char,
) -> Result<usize, ErreurSaisie> {
    let colonne: usize = convertit_les_colonnes_en_lettre_en_chiffre(colonne_en_lettre)?;

    let tableau_de_colonne = extraire_colonnes_piece(plateau, piece_en_lettre)?;

    match tableau_de_colonne.contains(&colonne) {
        true => return Ok(colonne),
        false => {
            return Err(ErreurSaisie::PieceAbsenteDeLaColonne {
                piece: piece_en_lettre,
                colonne_attendue: colonne,
                colonnes_trouvees: tableau_de_colonne,
            });
        }
    };
}

fn rangee_en_chiffre(chiffre_en_char: char) -> Result<usize, ErreurSaisie> {
    if !('1'..='8').contains(&chiffre_en_char) {
        return Err(ErreurSaisie::RangeeInvalide(chiffre_en_char));
    }
    chiffre_en_char
        .to_digit(10)
        .map(|chiffre| chiffre as usize)
        .ok_or(ErreurSaisie::RangeeInvalide(chiffre_en_char))
}

fn rangee_en_index(rangee_en_lettre: usize) -> usize {
    TAILLE_PLATEAU - rangee_en_lettre
}

pub fn get_position_piece(
    plateau: &Plateau,
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

    let colonne: Result<usize, ErreurSaisie> = get_colonne(&plateau, tuple.1, tuple.0);

    let rangee: usize = rangee_en_index(rangee_en_chiffre(tuple.2)?);

    let position = Position {
        colonne: colonne.clone()?,
        rangee: rangee,
    };

    let piece_extraite: Piece = Piece::construction_de_piece(&tuple.0)?;

    match plateau.get_piece(&position) {
        Ok(piece) if piece == piece_extraite => return Ok(position),
        Ok(piece_trouvee) => {
            return Err(ErreurSaisie::MauvaisePiece {
                piece_attendu: piece_extraite,
                piece_trouve: piece_trouvee,
                colonne: colonne?,
                rangee: rangee,
            });
        }
        Err(e) => return Err(e.into()),
    }
}

pub fn get_position_plateau(position_saisis: &str) -> Result<Position, ErreurSaisie> {
    let mut chars = position_saisis.chars();

    let tuple = match (chars.next(), chars.next(), chars.next()) {
        (Some(colonne), Some(rangee), None) => (colonne, rangee),
        (_, _, Some(_)) => return Err(ErreurSaisie::TropLongue),
        _ => return Err(ErreurSaisie::TropCourte),
    };

    let colonne: usize = convertit_les_colonnes_en_lettre_en_chiffre(tuple.0)?;
    let rangee: usize = rangee_en_index(rangee_en_chiffre(tuple.1)?);

    Ok(Position {
        colonne: colonne,
        rangee: rangee,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::creation_du_plateau;

    #[test]
    fn la_position_e6_retourne_la_position_colonne_4_et_rangee_2() {
        let position = Position {
            colonne: 4,
            rangee: 2,
        };
        assert_eq!(position, get_position_plateau("e6").unwrap())
    }

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
