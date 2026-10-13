#[derive(Debug, PartialEq, Clone)]
pub enum ErreurPiece {
    PieceInconnue,
    LettreInconnue(char),
}
