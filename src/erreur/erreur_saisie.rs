use crate::create_piece::Piece;
use crate::erreur::ErreurPiece;
use crate::erreur::ErreurPlateau;
use std::error::Error;
use std::fmt;

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

impl From<ErreurPiece> for ErreurSaisie {
    fn from(erreur: ErreurPiece) -> Self {
        ErreurSaisie::Piece(erreur)
    }
}

impl From<ErreurPlateau> for ErreurSaisie {
    fn from(erreur: ErreurPlateau) -> Self {
        ErreurSaisie::Plateau(erreur)
    }
}

impl fmt::Display for ErreurSaisie {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErreurSaisie::PieceAbsenteDeLaColonne {
                piece: p,
                colonne_attendue: ca,
                colonnes_trouvees: ct,
            } => write!(
                f,
                "Piece {} absence de la colonne {}, en revanche la pieces se trouve peut etre la {:?}",
                p, ca, ct
            ),
            ErreurSaisie::MauvaisePiece {
                piece_attendu: pa,
                piece_trouve: pt,
                colonne: c,
                rangee: r,
            } => write!(
                f,
                "A la position {} {} il y a la piece {:?}, et non {:?}",
                c, r, pt, pa
            ),
            ErreurSaisie::TropCourte => write!(f, "Vous n'avez pas entrez assez de caractere"),
            ErreurSaisie::TropLongue => write!(f, "Vous avez entrez trop de caractere"),
            ErreurSaisie::RangeeInvalide(range) => write!(f, "La range {} n'existe pas", range),
            ErreurSaisie::ColonneInvalide(colonne) => {
                write!(f, "La colonne {} n'existe pas", colonne)
            }
            ErreurSaisie::Plateau(erreur) => write!(f, "{erreur}"),
            ErreurSaisie::Piece(erreur) => write!(f, "{erreur}"),
        }
    }
}

impl Error for ErreurSaisie {}
