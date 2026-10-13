use crate::Plateau;
use crate::create_piece::Couleur;
use crate::notation::{Coup, analyser_coup};
use crate::user_interface::{afficher_plateau, effacement_terminal, saisie};
use std::error::Error;

pub fn lancer_partie(plateau: &mut Plateau) {
    let mut tour = Couleur::Blanc;
    let mut message = String::new();
    let mut fin: usize = 0;

    loop {
        effacement_terminal();
        afficher_plateau(plateau);

        if !message.is_empty() {
            println!("\n{message}");
            println!("Veuillez réessayer");
        }

        match tour {
            Couleur::Blanc => println!("\n°Tour des blancs\n"),
            Couleur::Noir => println!("\n°Tour des noirs\n"),
        }

        match jouer_un_tour(plateau, tour) {
            Ok(()) => {
                tour = tour.tour_suivant();
                message.clear();
                fin += 1;
            }
            Err(erreur) => message = erreur.to_string(),
        }

        if fin == 2 {
            effacement_terminal();
            afficher_plateau(plateau);
            break;
        }
    }
}

pub fn jouer_un_tour(plateau: &mut Plateau, tour: Couleur) -> Result<(), Box<dyn Error>> {
    let (depart, arrivee) = saisie::demander_coup();

    let coup_jouer: Coup = analyser_coup(plateau, &depart, &arrivee)?;
    let piece = plateau.get_piece(&coup_jouer.depart)?;

    if piece.couleur == tour {
        plateau.deplacer_pion(&coup_jouer)?;
        Ok(())
    } else {
        Err("Ce n'est pas votre tour".into())
    }
}
