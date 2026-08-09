use crate::create_piece::Piece;

pub struct Fen {
    fen: Vec<char>
}

//Crée un tableau à partir d'une chaine de str
fn fen(a: &str) -> Vec<char> {
    a.chars().collect()
}

//Afficher le type de tous les éléments du tableau
fn print_fen(a: Vec<char>) {
    for f in a.iter(){
        let p = Piece::construction_de_piece(f);
        println!("{:?}", p);
    }
}

impl Fen {
    //Appel les deux fonctions
    pub fn who_is_type_of_fen(a: &str){
        print_fen(fen(a));
    }
}