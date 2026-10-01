use crate::create_piece::{Piece, TypePiece};
#[derive(Debug)]
pub enum CaseType {
    Piece(Piece),
    Vide,
}

pub fn plateau(fen: &str) -> Vec<CaseType> {
    let mut plateau: Vec<CaseType> = vec![];
    for f in fen.chars() {
        if f.is_ascii_alphabetic() {
            plateau.push(CaseType::Piece(Piece::construction_de_piece(&f)))
        }

        if f.is_ascii_digit() {
            let chiffre: u32 = f.to_digit(10).unwrap();
            let mut i: u32 = 0;
            while i < chiffre {
                plateau.push(CaseType::Vide);
                i += 1;
            }
        }
    }

    plateau
}

pub fn afficher_plateau(cases: &[CaseType]) {
    let mut nbr_case: usize = cases.len(); // 16
    dbg!(nbr_case);
    let mut i: usize = 0;
    let mut tour: usize = 0;

    while nbr_case > 0 {
        print!("\n");
        while tour < 8 {
            match &cases[i] {
                CaseType::Piece(piece) => {
                    let symbole = piece.type_piece.affichage();
                    print!("{} ", symbole)
                }
                CaseType::Vide => print!("."),
            }
            i += 1;
            tour += 1;
        }
        nbr_case -= 8;
        tour = 0;
    }
}

/*
Formule quelle pièce est en (rangée x, colonne x)
    index = rangée * 8 + colonne
*/

/*pub fn plateau(fen: &str) -> Vec<Piece> {
    let mut plateau: Vec<Piece> = vec![];

    for f in fen.chars() {
        if f.is_ascii_alphabetic() {
            plateau.push(Piece::construction_de_piece(&f))
        } else if f.is_ascii_digit() {
            let chiffre = f.to_digit(10).unwrap();
            let count: &mut u32 = &mut 0;
            loop {
                *count += 1;

                plateau.push(None);

                if *count == chiffre {
                    break;
                }
            }
        } else {
            //s'il y a un /, fait rien
        }
    }
    plateau
}*/
