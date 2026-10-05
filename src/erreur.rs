#[derive(Debug, PartialEq, Clone)]
pub enum ErreurPlateau {
    TropDeCases,
    PasAssezDeCases,
    PieceInconnue,
    LettreInconnue,
    NombreIncorrecteParLigne,
    CaseVide,
    PasUnChiffre,
    ChaineTropCourte,
    ChaineTropLongue,
}
