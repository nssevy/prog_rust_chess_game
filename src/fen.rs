/*
 * Ce fichier est en pause. Il me sert strictement à rien.
 */

pub fn calcule_taille_de_la_fen(fen: &str) -> usize {
    let mut chiffre: usize = 0;
    let mut lettre: usize = 0;

    for f in fen.chars() {
        if f.is_ascii() && !f.is_ascii_digit() {
            lettre += 1;
        }
        if f.is_ascii_digit() {
            let value = (f as usize) - ('0' as usize);
            chiffre += value;
        }
    }
    let taille = lettre + chiffre;
    taille
}
/*
use crate::create_piece::Piece;

pub struct Fen {
    fen: Vec<char>,
}

fn fen(a: &str) -> Vec<char> {
    a.chars().collect()
}

fn print_fen(tableau: Vec<char>) {
    for f in tableau.iter() {
        let p = Piece::construction_de_piece(f);
        println!("{:?}", p);
    }
}

impl Fen {
    //Appel les deux fonctions
    pub fn who_is_type_of_fen(a: &str) {
        print_fen(fen(a));
    }
}
*/
