/*
Ce fichier vient établir des règles : dans le plateau, il peut y avoir que deux types d'éléments :
- une pièce
- une case vide
On vient créer le tableau par rapport à la fen Afin de pouvoir déplacer les pièces, etc., à la fin il y a une fonction d'affichage du tableau qui est juste purement esthétique.
 */
use crate::create_piece::Piece;
use crate::erreur::ErreurPlateau;

const TAILLE_FEN: usize = 71;

#[derive(Copy, Clone, Debug, PartialEq)] //PartialEq
pub enum CaseType {
    Piece(Piece),
    Vide,
}

type Plateau = [[CaseType; 8]; 8];

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

pub fn creation_du_plateau(fen: &str) -> Result<Plateau, ErreurPlateau> {
    if calcule_taille_de_la_fen(&fen) > TAILLE_FEN {
        return Err(ErreurPlateau::TropDeCases);
    }
    if calcule_taille_de_la_fen(&fen) < TAILLE_FEN {
        return Err(ErreurPlateau::PasAssezDeCases);
    }

    let mut plateau: Plateau = [[CaseType::Vide; 8]; 8];
    let mut ligne: usize = 0;
    let mut colonne: usize = 0;

    for f in fen.chars() {
        if f == '/' {
            ligne += 1;
            colonne = 0;
        }

        if f.is_ascii_alphabetic() {
            plateau[ligne][colonne] = CaseType::Piece(Piece::construction_de_piece(&f)?);
            colonne += 1;
        }

        if f.is_ascii_digit() {
            let chiffre: usize = (f as usize) - ('0' as usize);

            while colonne == chiffre {
                plateau[ligne][colonne] = CaseType::Vide;
                colonne += 1;
            }
        }
    }

    Ok(plateau)
}

pub fn afficher_plateau(cases: &Plateau) {
    let mut i: usize = 0;

    for _ in 0..8 {
        println!("");
        for piece in cases[i] {
            match piece {
                CaseType::Piece(piece) => print!("{} ", piece.affichage_sur_plateau()),
                CaseType::Vide => print!(". "),
            }
        }
        i += 1;
    }
}
