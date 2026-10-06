use crate::create_piece::{Couleur, Piece, TypePiece};
use crate::erreur::{ErreurPlateau, ErreurSaisie};
use crate::plateau::{CaseType, Plateau, Position, TAILLE_PLATEAU};
use std::io::{self, Write};

pub fn affichage_sur_plateau(piece: &Piece) -> &str {
    match (piece.couleur, piece.type_piece) {
        (Couleur::Blanc, TypePiece::Roi) => "K",
        (Couleur::Blanc, TypePiece::Dame) => "Q",
        (Couleur::Blanc, TypePiece::Cavalier) => "N",
        (Couleur::Blanc, TypePiece::Tour) => "R",
        (Couleur::Blanc, TypePiece::Fou) => "B",
        (Couleur::Blanc, TypePiece::Pion) => "P",
        //___
        (Couleur::Noir, TypePiece::Roi) => "k",
        (Couleur::Noir, TypePiece::Dame) => "q",
        (Couleur::Noir, TypePiece::Cavalier) => "n",
        (Couleur::Noir, TypePiece::Tour) => "r",
        (Couleur::Noir, TypePiece::Fou) => "b",
        (Couleur::Noir, TypePiece::Pion) => "p",
    }
}

pub fn afficher_plateau(cases: &Plateau) {
    let mut i: usize = 0;
    let mut nombre_de_gauche: usize = 8;

    for _ in 0..8 {
        println!("");
        print!("{nombre_de_gauche}    ");
        for piece in cases[i] {
            match piece {
                CaseType::Piece(piece) => print!("{} ", affichage_sur_plateau(&piece)),
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

pub fn demander_position() -> String {
    print!("\nJouer une position : ");
    io::stdout().flush().unwrap();

    let mut position_saisie = String::new();
    io::stdin()
        .read_line(&mut position_saisie)
        .expect("Échec de la lecture");

    print!("\x1B[A\x1B[2K\x1B[A\x1B[2K");
    io::stdout().flush().unwrap();

    position_saisie.trim().to_string()
}

fn convertit_les_colonne_en_lettre_en_chiffre(lettre: char) -> Result<usize, ErreurSaisie> {
    let chiffre = ('a'..='h')
        .contains(&lettre)
        .then(|| ((lettre as u32) - ('a' as u32)) as usize)
        .ok_or(ErreurSaisie::RangeeInvalide)?;
    Ok(chiffre)
}

/*
L'idée derrière get_colonne() est que à partir de bc8 qui veut dire -> Bishop(fou) en colonne c rangée 8.

Et de venir récuperer la piece -> b et la colonne -> c afin de vérifié une correspondance.

Si la fonction extraire_colonnes_piece_fen() retourne un tableau de [2, 4], alors ca veut dire qu'il y a bien un fou qui est sur la colonne c(2) donc la colonne est bien égale à 2.

En revanche qui la fonction extraire_colonnes_piece_fen() retourne un tableau de [3, 7] ca veut dire qu'il n'y a pas de fou sur la colonne c(2) donc l'utilisateur à saisis une position invalide.
*/

/*Remplacer la la fen, par le plateau afin d'obenir toutes les colonnes sur lesquelles une piece ou un pion se trouve, car dans tout les cas c'est le plateau qui change au fur à mesure de la partie donc autant se basé dessus.

+ si l'utilisateur entre une position avec par exemple un pion soit pa7 comme dès le début il y a 8 pions sur l'échequier, on peut ecrire la fonction de recherche dichotomique afin d'évité parcourir tout le tableau de colonne. Car le tableau sera rangée par ordre croissant, et on sait deja quelle valeur l'on veut chercher.
*/

fn get_colonne(
    cases: &Plateau,
    colonne_en_lettre: char,
    piece_en_lettre: char,
) -> Result<usize, ErreurSaisie> {
    let colonne_en_chiffre: usize = convertit_les_colonne_en_lettre_en_chiffre(colonne_en_lettre)?;
    //let tableau_de_colonne = extraire_colonnes_piece_fen(&fen, piece_en_lettre);
    //let tableau_de_colonne = extraire_colonnes_piece(&cases, piece_en_lettre);

    /*let tableau_de_colonne = match extraire_colonnes_piece(&cases, piece_en_lettre) {
        Ok(tb) => tb,
        Err(_) => return Err(ErreurSaisie::TropCourte), //méfiant
    };*/

    let Ok(tableau_de_colonne) = extraire_colonnes_piece(&cases, piece_en_lettre) else {
        return Err(ErreurSaisie::TropCourte);
    };

    dbg!(&piece_en_lettre);
    dbg!(&tableau_de_colonne);

    let colonne = if tableau_de_colonne.contains(&colonne_en_chiffre) {
        Ok(colonne_en_chiffre)
    } else {
        return Err(ErreurSaisie::PieceAbsenteDeLaColonne {
            piece: piece_en_lettre,
            colonne_attendue: colonne_en_chiffre,
            colonnes_trouvees: tableau_de_colonne,
        });
    };

    Ok(colonne?)
}

fn range_en_chiffre(chiffre_en_char: char) -> Result<usize, ErreurSaisie> {
    chiffre_en_char
        .to_digit(10)
        .map(|chiffre| chiffre as usize)
        .ok_or(ErreurSaisie::RangeeInvalide)
}

fn rangee_en_index(rangee_en_lettre: usize) -> usize {
    TAILLE_PLATEAU - rangee_en_lettre
}

pub fn get_position_piece(
    cases: &Plateau,
    position_saisis: &str,
) -> Result<Position, ErreurSaisie> {
    let mut chars = position_saisis.chars();

    let tuple = match (chars.next(), chars.next(), chars.next(), chars.next()) {
        (Some(piece_en_lettre), Some(colonne), Some(rangee), None) => {
            (piece_en_lettre, colonne, rangee)
        }
        (_, _, _, Some(_)) => return Err(ErreurSaisie::TropLongue),
        _ => return Err(ErreurSaisie::TropCourte),
    };

    //let colonne: Result<usize, ErreurSaisie> = get_colonne(&fen, tuple.1, tuple.0);
    let colonne: Result<usize, ErreurSaisie> = get_colonne(&cases, tuple.1, tuple.0);

    let rangee: usize = rangee_en_index(range_en_chiffre(tuple.2)?);

    Ok(Position {
        colonne: colonne?,
        rangee: rangee,
    })
}

/*
1 - J'ai mon char, je converti mon char en Piece/Pion

2 - Je parcour tout mon plateau, et dès lorsque une je rencontre mon/ma piece/pion je stock sa colonne dans un tableau.
*/
fn extraire_colonnes_piece(cases: &Plateau, piece: char) -> Result<Vec<usize>, ErreurPlateau> {
    let piece_extraite: Piece = Piece::construction_de_piece(&piece)?;
    let mut tableau_colonne_piece: Vec<usize> = vec![];

    let mut i: usize = 0;

    for _ in 0..8 {
        for (colonne, piece) in cases[i].into_iter().enumerate() {
            match piece {
                CaseType::Piece(p) if p == piece_extraite => tableau_colonne_piece.push(colonne),
                _ => {}
            }
        }
        i += 1;
    }

    Ok(tableau_colonne_piece)
}
//

#[cfg(test)]
mod tests {
    use super::*;
    use crate::creation_du_plateau;

    #[test]
    fn dans_la_fen_initiale_la_lettre_n_apparait_2_fois_dans_le_plateau_au_colonne_1_et_6() {
        let fen: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";
        let plateau = match creation_du_plateau(&fen) {
            Ok(plateau) => plateau,
            Err(e) => return println!("Erreur : {:?}", e),
        };
        let tableau = vec![1, 6];
        assert_eq!(tableau, extraire_colonnes_piece(&plateau, 'n').unwrap())
    }

    #[test]
    fn la_lettre_p_apparait_8_fois_dans_le_plateau() {
        let fen: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";
        let plateau = match creation_du_plateau(&fen) {
            Ok(plateau) => plateau,
            Err(e) => return println!("Erreur : {:?}", e),
        };
        assert_eq!(8, extraire_colonnes_piece(&plateau, 'p').unwrap().len())
    }

    #[test]
    fn la_lettre_b_apparait_2_fois_dans_le_plateau() {
        let fen: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";
        let plateau = match creation_du_plateau(&fen) {
            Ok(plateau) => plateau,
            Err(e) => return println!("Erreur : {:?}", e),
        };
        assert_eq!(2, extraire_colonnes_piece(&plateau, 'b').unwrap().len())
    }

    #[test]
    fn check_bc8_equivaut_à_position_colonne_2_et_rangée_0() {
        let fen: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";
        let plateau = match creation_du_plateau(&fen) {
            Ok(plateau) => plateau,
            Err(e) => return println!("Erreur : {:?}", e),
        };
        assert_eq!(
            get_position_piece(&plateau, "bc8"),
            Ok(Position {
                colonne: 2,
                rangee: 0
            })
        )
    }
}
