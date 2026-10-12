use crate::create_piece::Piece;

#[derive(Debug, PartialEq, Clone)]
pub enum ErreurPlateau {
    TropDeCases(usize),
    PasAssezDeCases(usize),
    NombreIncorrecteParLigne,
    CaseVide,
    Piece(ErreurPiece),
}

#[derive(Debug, PartialEq, Clone)]
pub enum ErreurPiece {
    PieceInconnue,
    LettreInconnue(char),
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
    Piece(ErreurPiece),
}

impl From<ErreurPlateau> for ErreurSaisie {
    fn from(erreur: ErreurPlateau) -> Self {
        ErreurSaisie::Plateau(erreur)
    }
}

impl From<ErreurPiece> for ErreurPlateau {
    fn from(erreur: ErreurPiece) -> Self {
        ErreurPlateau::Piece(erreur)
    }
}
impl From<ErreurPiece> for ErreurSaisie {
    fn from(erreur: ErreurPiece) -> Self {
        ErreurSaisie::Piece(erreur)
    }
}
