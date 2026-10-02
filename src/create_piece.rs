/*
Ce fichier permet de créer des pièces de jeu d'échecs.
*/
use crate::erreur::ErreurPlateau;

#[derive(Debug)]
pub enum TypePiece {
    Roi,      //King
    Dame,     // Queen
    Cavalier, // Knignt
    Tour,     // Rook
    Fou,      // Bishop
    Pion,     // P awn
}

#[derive(Debug)]
pub enum Couleur {
    Blanc, // Majucule
    Noir,  // Minuscule
}

#[derive(Debug)]
pub struct Piece {
    pub couleur: Couleur,
    pub type_piece: TypePiece,
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

    pub fn affichage_sur_plateau(&self) -> &str {
        match (&self.couleur, &self.type_piece) {
            (Couleur::Blanc, TypePiece::Roi) => "R",
            (Couleur::Blanc, TypePiece::Dame) => "Q",
            (Couleur::Blanc, TypePiece::Cavalier) => "N",
            (Couleur::Blanc, TypePiece::Tour) => "R",
            (Couleur::Blanc, TypePiece::Fou) => "B",
            (Couleur::Blanc, TypePiece::Pion) => "P",
            //___
            (Couleur::Noir, TypePiece::Roi) => "r",
            (Couleur::Noir, TypePiece::Dame) => "q",
            (Couleur::Noir, TypePiece::Cavalier) => "n",
            (Couleur::Noir, TypePiece::Tour) => "r",
            (Couleur::Noir, TypePiece::Fou) => "b",
            (Couleur::Noir, TypePiece::Pion) => "p",
        }
    }
}

fn couleur_piece(lettre: char) -> Result<Couleur, ErreurPlateau> {
    match lettre {
        'A'..='Z' => Ok(Couleur::Blanc),
        'a'..='z' => Ok(Couleur::Noir),
        _ => return Err(ErreurPlateau::PieceInconnue),
    }
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
