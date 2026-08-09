use crate::create_piece::Piece;

pub struct Fen {
    fen: Vec<char>
}

impl Fen {
    pub fn who_is_type_of_fen(a: &str){
        //let tab_fen = fen(a);
        print_fen(fen(a));
    }
}
fn fen(a: &str) -> Vec<char> {
    //crée un tableau à partir d'une chaine de str
    a.chars().collect()
}

fn print_fen(a: Vec<char>) {
    for f in a.iter(){
        let p = Piece::construction_de_piece(f);
        println!("{:?}", p);
    }
}