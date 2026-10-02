/*
Ce fichier vient établir des règles : dans le plateau, il peut y avoir que deux types d'éléments :
- une pièce
- une case vide
On vient créer le tableau par rapport à la fen Afin de pouvoir déplacer les pièces, etc., à la fin il y a une fonction d'affichage du tableau qui est juste purement esthétique.
 */
use crate::create_piece::Piece;
use crate::erreur::ErreurPlateau;

#[derive(Debug)]
pub enum CaseType {
    Piece(Piece),
    Vide,
}

pub fn creation_du_plateau(fen: &str) -> Result<Vec<CaseType>, ErreurPlateau> {
    let mut plateau: Vec<CaseType> = vec![];
    for f in fen.chars() {
        if f.is_ascii_alphabetic() {
            plateau.push(CaseType::Piece(Piece::construction_de_piece(&f)?))
        }

        if f.is_ascii_digit() {
            let chiffre: u32 = f.to_digit(10).unwrap();
            for _ in 0..chiffre {
                plateau.push(CaseType::Vide);
            }
        }
    }
    if plateau.len() > 64 {
        return Err(ErreurPlateau::TropDeCases);
    }
    if plateau.len() < 64 {
        return Err(ErreurPlateau::PasAssezDeCases);
    }

    Ok(plateau)
}

pub fn afficher_plateau(cases: &[CaseType]) {
    for ligne in cases.chunks(8) {
        for case in ligne {
            match case {
                CaseType::Piece(piece) => print!("{} ", piece.affichage_sur_plateau()),
                CaseType::Vide => print!(". "),
            }
        }
        println!();
    }
}
