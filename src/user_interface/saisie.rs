use std::io::{self, Write};

pub fn demander_coup() -> (String, String) {
    let depart = demander_saisie("Pièce à déplacer : ");
    let arrivee = demander_saisie("Case d'arrivée : ");
    (depart, arrivee)
}

pub fn demander_saisie(question: &str) -> String {
    print!("{question}");
    io::stdout().flush().unwrap();

    let mut reponse = String::new();
    io::stdin()
        .read_line(&mut reponse)
        .expect("Échec de la lecture");

    reponse.trim().to_string()
}
