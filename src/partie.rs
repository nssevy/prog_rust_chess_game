use crate::Plateau;
use crate::notation::analyser_coup;
use crate::user_interface::{afficher_plateau, effacement_terminal, saisie};

pub fn lancer_partie(plateau: &mut Plateau) {
    let mut tour: u8 = 0;

    loop {
        effacement_terminal();
        afficher_plateau(&plateau);

        if tour % 2 == 0 {
            println!("\n-Tour des blanc\n");
        } else {
            println!("\n-Tour des noir\n");
        }

        let (depart, arrivee) = saisie::demander_coup();
        tour += 1;

        match analyser_coup(&plateau, &depart, &arrivee) {
            Ok(coup) => match plateau.deplacer_pion(&coup) {
                Ok(()) => {
                    //rien
                }
                Err(e) => println!("Erreur lors du déplacement : {:?}", e),
            },
            Err(e) => println!("Erreur de saisie : {:?}", e),
        }
        if tour == 2 {
            effacement_terminal();
            afficher_plateau(&plateau);
            break;
        }
    }
}
