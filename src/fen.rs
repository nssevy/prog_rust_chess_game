/*
 * Ce fichier est en pause. Il me sert strictement à rien.
 */
use crate::erreur::ErreurPlateau;
const TAILLE_FEN: usize = 71;

pub fn calcule_taille_de_la_fen(fen: &str) -> usize {
    let mut chiffre: usize = 0;
    let mut lettre: usize = 0;

    for f in fen.chars() {
        if f.is_ascii() && !f.is_ascii_digit() {
            lettre += 1;
        }
        if f.is_ascii_digit() {
            let value = (f as usize) - ('0' as usize);
            chiffre += value;
        }
    }
    let taille = lettre + chiffre;
    taille
}

pub fn huit_elements_par_ligne(fen: &str) -> bool {
    let rangees: Vec<&str> = fen.split('/').collect();

    if rangees.len() != 8 {
        return false;
    }

    for rangee in rangees {
        let mut total_cases = 0;

        for c in rangee.chars() {
            if c.is_ascii_digit() {
                if let Some(nombre_cases_vides) = c.to_digit(10) {
                    total_cases += nombre_cases_vides;
                }
            } else {
                total_cases += 1;
            }
        }
        if total_cases != 8 {
            return false;
        }
    }

    true
}

pub fn valider_fen_structure(fen: &str) -> Result<usize, ErreurPlateau> {
    match calcule_taille_de_la_fen(fen) {
        n if n > TAILLE_FEN => Err(ErreurPlateau::TropDeCases),
        n if n < TAILLE_FEN => Err(ErreurPlateau::PasAssezDeCases),
        _ => Ok(0),
    }
    .and_then(|_| match huit_elements_par_ligne(fen) {
        true => Ok(0),
        false => Err(ErreurPlateau::NombreIncorrecteParLigne),
    })
}

pub fn extraire_colonnes_piece_fen(fen: &str, piece: char) -> Vec<usize> {
    let placement = fen.split_whitespace().next().unwrap_or("");
    let mut colonnes = Vec::new();

    for rangee in placement.split('/') {
        let mut colonne = 0;
        for c in rangee.chars() {
            if let Some(vides) = c.to_digit(10) {
                colonne += vides as usize;
            } else {
                if c == piece {
                    colonnes.push(colonne);
                }
                colonne += 1;
            }
        }
    }
    colonnes
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn get_l_index_des_colonnes_ou_la_piece_fou_noir() {
        let fen: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";
        assert_eq!(extraire_colonnes_piece_fen(fen, 'b'), [2, 5])
    }
}
