#![allow(dead_code)]

#[derive(Debug)]
pub enum TypePiece {
    Roi,      //King
    Dame,     // Queen
    Cavalier, // Knignt
    Tour,     // Rook
    Fou,      // Bishop
    Pion,     // P awn
}

impl TypePiece {
    pub fn affichage(&self) -> &str {
        match self {
            TypePiece::Roi => "R",
            TypePiece::Dame => "Q",
            TypePiece::Cavalier => "N",
            TypePiece::Tour => "R",
            TypePiece::Fou => "B",
            TypePiece::Pion => "P",
        }
    }
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
    pub fn construction_de_piece(a: &char) -> Piece {
        let c = couleur_piece(*a);
        let t = type_piece(*a);

        Piece {
            couleur: c,
            type_piece: t,
        }
    }
}

fn couleur_piece(lettre: char) -> Couleur {
    match lettre {
        'A'..='Z' => Couleur::Blanc,
        'a'..='z' => Couleur::Noir,
        _ => panic!("Ce n'est pas une lettre de l'alphabet"),
    }
}

fn type_piece(a: char) -> TypePiece {
    match a {
        'K' | 'k' => TypePiece::Roi,
        'Q' | 'q' => TypePiece::Dame,
        'N' | 'n' => TypePiece::Cavalier,
        'R' | 'r' => TypePiece::Tour,
        'B' | 'b' => TypePiece::Fou,
        'P' | 'p' => TypePiece::Pion,
        _ => panic!("La piece n'existe pas"),
    }
}
