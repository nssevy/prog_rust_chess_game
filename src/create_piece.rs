/*
Ce fichier permet de créer des pièces de jeu d'échecs.
*/
use crate::erreur::ErreurPlateau;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TypePiece {
    Roi,      //King
    Dame,     // Queen
    Cavalier, // Knignt
    Tour,     // Rook
    Fou,      // Bishop
    Pion,     // P awn
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Couleur {
    Blanc, // Majucule
    Noir,  // Minuscule
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Piece {
    pub couleur: Couleur,
    pub type_piece: TypePiece,
}

fn type_piece(a: char) -> Result<TypePiece, ErreurPlateau> {
    match a {
        'K' | 'k' => Ok(TypePiece::Roi),
        'Q' | 'q' => Ok(TypePiece::Dame),
        'N' | 'n' => Ok(TypePiece::Cavalier),
        'R' | 'r' => Ok(TypePiece::Tour),
        'B' | 'b' => Ok(TypePiece::Fou),
        'P' | 'p' => Ok(TypePiece::Pion),
        _ => return Err(ErreurPlateau::LettreInconnue),
    }
}

fn couleur_piece(lettre: char) -> Result<Couleur, ErreurPlateau> {
    match lettre {
        'A'..='Z' => Ok(Couleur::Blanc),
        'a'..='z' => Ok(Couleur::Noir),
        _ => return Err(ErreurPlateau::PieceInconnue),
    }
}

impl Piece {
    pub fn construction_de_piece(a: &char) -> Result<Piece, ErreurPlateau> {
        let c = couleur_piece(*a)?;
        let t = type_piece(*a)?;

        Ok(Piece {
            couleur: c,
            type_piece: t,
        })
    }
}
