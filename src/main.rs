#![allow(dead_code)]
mod create_piece;

fn fen(a: &str) -> Vec<char> {
    //crée un tableau à partir d'une chaine de str
    a.chars().collect()
}

struct Plateau <'a>{
    fen: &'a str,
    echequier: Vec<Vec<char>>,
}

fn echequier(a: &str) -> Vec<Vec<char>> {

    let s: Vec<_> = a.split('/').collect(); // ["rnbqkbn", "RNBQKBN"]

    let mut i: usize = 0;
    let mut un = vec![];
    let mut deux = vec![];

    un = fen(s[i]);
    i += 1;
    deux = fen(s[i]);

    let mut echequier = vec![];
    echequier.push(un);
    echequier.push(deux);
    echequier
}

fn print_echequier(a: Vec<Vec<char>>) {
    for p in a.iter(){
        println!("{:?}", p);
    }
}

fn main(){
    //rnbqkbnr
    let base_fen: &str = "rnbqkbn/RNBQKBN";

    let plateau = echequier(base_fen);

    print_echequier(plateau);

    let fen = fen(base_fen); // ['r', 'n', 'b', 'q', 'k', 'b', 'n', '/', 'R', 'N', 'B', 'Q', 'K', 'B', 'N']

    // Permet de reconnaitre le type de piece dans le tableau (echequier)
    for f in fen.iter(){
        let p = create_piece::Piece::construction_de_piece(*f);
        println!("{:?}", p);
    }

    //println!("{:?}", fen);
}
