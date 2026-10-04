mod create_piece;
mod erreur;
mod fen;
mod plateau;
use crate::erreur::ErreurPlateau;
use crate::plateau::{Position, afficher_plateau, creation_du_plateau};
use std::io::{self, Write};

fn main() {
    let fen: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";
    let plateau = match creation_du_plateau(&fen) {
        Ok(plateau) => plateau,
        Err(e) => return println!("Erreur : {:?}", e),
    };

    afficher_plateau(&plateau);

    match plateau.get_piece(plateau.position_piece(0, 1)) {
        Ok(a) => println!("\n{:?}", a),
        Err(e) => return println!("{:?}", e),
    };

    print!("Jouer une position : ");
    io::stdout().flush().unwrap();

    // 2. Créer une variable mutable pour stocker la saisie
    let mut position_saisie = String::new();

    // 3. Lire la saisie dans le terminal
    io::stdin()
        .read_line(&mut position_saisie)
        .expect("Échec de la lecture de la ligne");

    /*#[derive(Debug)]
    pub struct Position {
        pub colonne: usize,
        pub ligne: usize,
    }*/

    fn get_position(mut text: &str) -> Result<Position, ErreurPlateau> {
        text = text.trim();
        if text.len() < 2 {
            return Err(ErreurPlateau::ChaineTropCourte);
        }
        if text.len() > 3 {
            return Err(ErreurPlateau::ChaineTropLongue);
        }

        let n1: usize = text[0..1]
            .parse()
            .map_err(|_| ErreurPlateau::PasUnChiffre)?;
        let n2: usize = text[1..2]
            .parse()
            .map_err(|_| ErreurPlateau::PasUnChiffre)?;

        Ok(Position {
            colonne: n1,
            ligne: n2,
        })
    }

    //bg!(tuple_position(&position_saisie));
    match get_position(&position_saisie) {
        Ok(a) => println!("{:?}", a),
        Err(e) => println!("{:?}", e),
    }
}
