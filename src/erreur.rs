use crate::create_piece::Piece;

#[derive(Debug, PartialEq, Clone)]
pub enum ErreurPlateau {
    TropDeCases,
    PasAssezDeCases,
    PieceInconnue,
    LettreInconnue,
    NombreIncorrecteParLigne,
    CaseVide,
}

#[derive(Debug, PartialEq)]
pub enum ErreurSaisie {
    PieceAbsenteDeLaColonne {
        piece: char,
        colonne_attendue: usize,
        colonnes_trouvees: Vec<usize>,
    },
    MauvaisePiece {
        piece: char,
        rangee_attendue: usize,
        pieces_trouvees: Piece,
    },
    TropCourte,
    TropLongue,
    RangeeInvalide,
    Plateau(ErreurPlateau),
}

impl From<ErreurPlateau> for ErreurSaisie {
    fn from(erreur: ErreurPlateau) -> Self {
        ErreurSaisie::Plateau(erreur)
    }
}
