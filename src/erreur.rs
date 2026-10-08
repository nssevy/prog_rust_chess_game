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

#[derive(Debug, PartialEq, Clone)]
pub enum ErreurSaisie {
    PieceAbsenteDeLaColonne {
        piece: char,
        colonne_attendue: usize,
        colonnes_trouvees: Vec<usize>,
    },
    MauvaisePiece {
        piece_attendu: Piece,
        piece_trouve: Piece,
        colonne: usize,
        rangee: usize,
    },
    TropCourte,
    TropLongue,
    RangeeInvalide(char),
    ColonneInvalide(char),
    Plateau(ErreurPlateau),
}

impl From<ErreurPlateau> for ErreurSaisie {
    fn from(erreur: ErreurPlateau) -> Self {
        ErreurSaisie::Plateau(erreur)
    }
}
