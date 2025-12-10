use crate::jeux::{Jeux, Mode};
use crate::multi_joueur::client::client;
use crate::multi_joueur::hote::hote;
use crate::outils::outils::{demander, se_préparer};
use crate::outils::terminal::{afficher, afficher_str};

mod joueur;
mod multi_joueur;
mod outils;
mod jeux;


fn main() {

    loop {

        if est_ce_multi() {
            match choisir_le_role() {
                true => {
                    let mode = mode_de_jeu();
                    hote(mode);  //il faudra que l'hote envoie le role au client
                }

                false => {
                    client();
                }
            }

        }else {
            let mode = mode_de_jeu();
            let mut préparation = se_préparer("solitaire");
            let mut jeux = Jeux::nouveau(mode, &mut préparation.0, préparation.1, préparation.2);
            jeux.jouer();
        }


        if !rejouer(){
            break;
        }

    }
}


fn mode_de_jeu() -> Mode {
    afficher_str("Classique ? Chronomètre ?");

    match demander().as_str() {
        "Classique" | "classique"  | "1" | "cl" | "Cl" => {
            Mode::Classique
        }
        "Chronomètre" | "chronomètre" | "2" | "ch" | "Ch"  => {
            Mode::Chronomètre
        }
        _ => {
            afficher_str("bon… bha on va dire classique alors…");
            Mode::Classique
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
            _ => (),
        }
    }

}