#![allow(dead_code)]

#[derive(Debug)]
enum TypePiece {
    Roi, //K ing
    Dame, // Q uenne
    Cavalier, // N knignt
    Tour,// R ook
    Fou, // B ishop
    Pion, // P awn
    Vide
}

#[derive(Debug)]
enum Couleur {
    Blanc, // Majucule
    Noir, // Minuscule
    Vide, // Vide
}

#[derive(Debug)]
pub struct Piece {
    couleur: Couleur,
    type_piece : TypePiece,
}

fn couleur_piece(a: char) -> Option<Couleur> {
    if a.is_ascii_uppercase() {
        Some(Couleur::Blanc)
    } else if a.is_ascii_lowercase() {
        Some(Couleur::Noir)
    } else {
        None
    }
}

fn type_piece(a: char) -> Option<TypePiece>{
    match a {
        'K' | 'k' => Some(TypePiece::Roi),
        'Q' | 'q' => Some(TypePiece::Dame),
        'N' | 'n' => Some(TypePiece::Cavalier),
        'R' | 'r' => Some(TypePiece::Tour),
        'B' | 'b' => Some(TypePiece::Fou),
        'P' | 'p' => Some(TypePiece::Pion),
        //1..=8 => Some(TypePiece::Vide),
        _ => None
    }
}
// Si la fonction renvoie None crée une case vide, de type Option<Piece>
// et les cases vides sont construite par des chiffres allant de 1 à 8.
impl Piece {
    pub fn construction_de_piece(a: char) -> Option<Piece> {
        let c = couleur_piece(a);
        let t = type_piece(a);

        match (c, t) {
            (Some(c),  Some(t)) => Some(Piece{couleur: c, type_piece: t}),
            //(Some(c), Some()) =>
            _ => None,
        }
    }

}