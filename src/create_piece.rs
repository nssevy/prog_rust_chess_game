use crate::erreur::ErreurPiece;

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

impl Couleur {
    pub fn tour_suivant(self) -> Couleur {
        match self {
            Couleur::Blanc => Couleur::Noir,
            Couleur::Noir => Couleur::Blanc,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Piece {
    pub couleur: Couleur,
    pub type_piece: TypePiece,
}

pub fn type_piece(lettre: char) -> Result<TypePiece, ErreurPiece> {
    match lettre {
        'K' | 'k' => Ok(TypePiece::Roi),
        'Q' | 'q' => Ok(TypePiece::Dame),
        'N' | 'n' => Ok(TypePiece::Cavalier),
        'R' | 'r' => Ok(TypePiece::Tour),
        'B' | 'b' => Ok(TypePiece::Fou),
        'P' | 'p' => Ok(TypePiece::Pion),
        _ => return Err(ErreurPiece::LettreInconnue(lettre)),
    }
}

fn couleur_piece(lettre: char) -> Result<Couleur, ErreurPiece> {
    match lettre {
        'A'..='Z' => Ok(Couleur::Blanc),
        'a'..='z' => Ok(Couleur::Noir),
        _ => return Err(ErreurPiece::PieceInconnue),
    }
}

impl Piece {
    pub fn construction_de_piece(a: &char) -> Result<Piece, ErreurPiece> {
        let c = couleur_piece(*a)?;
        let t = type_piece(*a)?;

        Ok(Piece {
            couleur: c,
            type_piece: t,
        })
    }
}
