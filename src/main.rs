use crate::jeux::Jeux;
use crate::mode::classique::Classique;
use crate::outils::outils::{crée_partie, demander};
use crate::outils::terminal::{afficher, afficher_str};

mod joueur;
mod multi_joueur;
mod outils;
mod jeux;
mod mode;

fn main() {

        if est_ce_multi() {
            match choisir_le_role() {
                true => {
                    //hote();
                }

                false => {
                    //client();
                }
            }

        }else {
            
            let mut jeux = crée_partie(false, None, None, None);

            loop {
                jeux.jouer();
                if !rejouer() {
                    break;
                }
                jeux = crée_partie(false, None, None, None);

            }


        }


}







fn est_ce_multi() -> bool{

    loop {
        afficher_str("Mode de jeu : solitaire ou multi_joueur ?");
        let mode = demander();
        match mode.trim() {
            "solitaire" | "1" | "s" | "S" | "SOLITAIRE" => {
                return false
            }
            "multi_joueur" | "2" | "m" | "M" | "MULTIJOUEUR" => {
                return true
            }
            _ => {
                afficher_str("N'importe quoi !!");

            }
        }
    }

}


fn choisir_le_role() -> bool {
    afficher_str("Role : hote ou client");
    loop {
        match demander().trim() {
            "hote" | "h" | "1" => {
                return true
            }

            "client" | "c" | "2" => {
                return false
            }

            _ => {
                afficher_str("N'importe quoi !!");
            }

        }
    }

}


fn rejouer() -> bool{
    afficher(String::from("\n\nrejouer ? "));

    loop{
        let réponse = demander();
        match réponse.as_str() {
            "oui" | "o" =>  return true,
            "non" | "n" => return false,
            _ => afficher_str("si tu n'arrive même pas a répondre a une question aussi simple tu n'es pas prêt pour la suite"),
        }
    }

}