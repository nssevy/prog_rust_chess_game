use crate::erreur::ErreurPiece;
use std::error::Error;
use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum ErreurPlateau {
    TropDeCases(usize),
    PasAssezDeCases(usize),
    NombreIncorrecteParLigne,
    CaseVide,
    Piece(ErreurPiece),
}

impl From<ErreurPiece> for ErreurPlateau {
    fn from(erreur: ErreurPiece) -> Self {
        ErreurPlateau::Piece(erreur)
    }
}

impl fmt::Display for ErreurPlateau {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErreurPlateau::TropDeCases(nombre) => {
                write!(f, "Il y a trop de case dans le tableau {nombre}")
            }
            ErreurPlateau::PasAssezDeCases(nombre) => {
                write!(f, "Il n'y a pas assez de case dans le tableau {nombre}")
            }
            ErreurPlateau::NombreIncorrecteParLigne => {
                write!(f, "il y a trop d'éléments dans une ligne")
            }
            ErreurPlateau::CaseVide => {
                write!(f, "La piece que vous saisissez est sur une cases vide")
            }
            ErreurPlateau::Piece(erreur) => write!(f, "{erreur}"),
        }
    }
}

impl Error for ErreurPlateau {}

impl fmt::Display for ErreurPiece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErreurPiece::PieceInconnue => write!(f, "Cette pièce n'existe pas"),
            ErreurPiece::LettreInconnue(lettre) => {
                write!(f, "La lettre {lettre} ne correspond à aucune pièce")
            }
        }
    }
}
