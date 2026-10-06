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
    TropCourte,
    TropLongue,
    RangeeInvalide,
}
