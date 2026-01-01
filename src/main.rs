use crate::multi_joueur::client::client;
use crate::multi_joueur::hote::hote;
use crate::outils::terminal::afficher_str;
use crate::outils::outils::{crée_partie, demander, rejouer};


mod joueur;
mod multi_joueur;
mod outils;
mod jeux;
mod mode;

fn main() {

        if est_ce_multi() {
            match choisir_le_role() {
                true => {
                    hote();
                }

                false => {
                    client();
                }
            }

        }else {
            let mut jeux = crée_partie(false, None, None, None);

            loop {
                jeux.jouer();
                if !rejouer() {
                    break;
                }
                jeux = crée_partie(false, None, Some(jeux.mode().clone()), None);

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


