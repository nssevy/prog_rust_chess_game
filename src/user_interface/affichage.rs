use crate::create_piece::{Couleur, Piece, TypePiece};
use crate::plateau::{CaseType, Plateau};
use std::io;
use std::io::Write;

fn affichage_sur_plateau(piece: &Piece) -> &str {
    match (piece.couleur, piece.type_piece) {
        (Couleur::Blanc, TypePiece::Roi) => "♚",
        (Couleur::Blanc, TypePiece::Dame) => "♛",
        (Couleur::Blanc, TypePiece::Cavalier) => "♞",
        (Couleur::Blanc, TypePiece::Tour) => "♜",
        (Couleur::Blanc, TypePiece::Fou) => "♝",
        (Couleur::Blanc, TypePiece::Pion) => "♟",
        //___
        (Couleur::Noir, TypePiece::Roi) => "♔",
        (Couleur::Noir, TypePiece::Dame) => "♕",
        (Couleur::Noir, TypePiece::Cavalier) => "♘",
        (Couleur::Noir, TypePiece::Tour) => "♖",
        (Couleur::Noir, TypePiece::Fou) => "♗",
        (Couleur::Noir, TypePiece::Pion) => "♙",
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
    print!("\n");
}

pub fn effacement_terminal() {
    print!("\x1B[2J\x1B[H");
    io::stdout().flush().unwrap();
}
