/*
Ce fichier vient établir des règles : dans le plateau, il peut y avoir que deux types d'éléments :
- une pièce
- une case vide
On vient créer le tableau par rapport à la fen Afin de pouvoir déplacer les pièces, etc., à la fin il y a une fonction d'affichage du tableau qui est juste purement esthétique.
 */
use crate::create_piece::Piece;
use crate::erreur::ErreurPlateau;
use crate::fen::valider_fen_structure;
use std::ops::{Index, IndexMut};

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub enum CaseType {
    Piece(Piece),
    #[default]
    Vide,
}

#[derive(Debug, PartialEq)]
pub struct Position {
    pub colonne: usize,
    pub ligne: usize,
}

/*struct LettreChiffre(char, usize);

impl LettreChiffre {
    fn lettre_en_chiffre() -> usize {

    }
}

e7*/

#[derive(Default, Debug)]
pub struct Plateau([[CaseType; 8]; 8]);

impl Plateau {
    pub fn position_piece(&self, colonne: usize, ligne: usize) -> Position {
        Position { colonne, ligne }
    }

    pub fn get_piece(&self, position: Position) -> Result<Piece, ErreurPlateau> {
        match self[position.colonne][position.ligne] {
            CaseType::Piece(a) => Ok(a),
            CaseType::Vide => Err(ErreurPlateau::CaseVide),
        }
    }
}

impl Index<usize> for Plateau {
    type Output = [CaseType; 8];
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for Plateau {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

pub fn creation_du_plateau(fen: &str) -> Result<Plateau, ErreurPlateau> {
    valider_fen_structure(&fen)?;

    let mut plateau = Plateau::default();
    let mut ligne: usize = 0;
    let mut colonne: usize = 0;

    for f in fen.chars() {
        if f == '/' {
            ligne += 1;
            colonne = 0;
        }

        if f.is_ascii_alphabetic() {
            plateau[ligne][colonne] = CaseType::Piece(Piece::construction_de_piece(&f)?);
            colonne += 1;
        }

        if f.is_ascii_digit() {
            let chiffre: usize = (f as usize) - ('0' as usize);

            while colonne == chiffre {
                plateau[ligne][colonne] = CaseType::Vide;
                colonne += 1;
            }
        }
    }

    Ok(plateau)
}

pub fn afficher_plateau(cases: &Plateau) {
    let mut i: usize = 0;
    let mut nombre_de_gauche: usize = 8;

    for _ in 0..8 {
        println!("");
        print!("{nombre_de_gauche}    ");
        for piece in cases[i] {
            match piece {
                CaseType::Piece(piece) => print!("{} ", piece.affichage_sur_plateau()),
                CaseType::Vide => print!(". "),
            }
        }
        i += 1;
        nombre_de_gauche -= 1;
    }
    print!("\n");
    print!("\n     ");
    for lettre in 'a'..='h' {
        print!("{lettre} ");
    }
}
