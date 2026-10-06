use crate::create_piece::{Couleur, Piece, TypePiece};
use crate::plateau::{CaseType, Plateau};
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

    print!("\x1B[A\x1B[2K");
    io::stdout().flush().unwrap();

    position_saisie.trim().to_string()
}
