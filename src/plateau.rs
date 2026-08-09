use crate::create_piece::Piece;

/*
Formule quelle pièce est en (rangée x, colonne x)
    index = rangée * 8 + colonne
*/

pub fn plateau(fen: &str) -> Vec<Option<Piece>> {

    let mut plateau: Vec<Option<Piece>> = vec![];

    for f in fen.chars(){
        if f.is_ascii_alphabetic() {
            plateau.push(Piece::construction_de_piece(&f))
        } else if f.is_ascii_digit(){
            let chiffre = f.to_digit(10).unwrap();
            let count: &mut u32 = &mut 0;
            loop {
                *count += 1;

                plateau.push(Piece::construction_de_piece(&f));

                if *count == chiffre {
                    break;
                }
            }
        } else {

        }
    }

    plateau
}
