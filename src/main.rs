use crate::jeux::{Jeux, Mode};
use crate::multi_joueur::client::client;
use crate::multi_joueur::hote::hote;
use crate::outils::outils::{demander, demander_nb_manche, demander_temp, se_préparer};
use crate::outils::terminal::{afficher, afficher_str};

mod joueur;
mod multi_joueur;
mod outils;
mod jeux;
mod mode;

fn main() {

    loop {

        if est_ce_multi() {
            match choisir_le_role() {
                true => {
                    let mode = mode_de_jeu();
                    hote(mode);
                }

                false => {
                    client();
                }
            }

        }else {
            let mode = mode_de_jeu();
            let mut préparation = se_préparer("solitaire");
            let mut jeux = Jeux::nouveau(mode.clone(), préparation.0, préparation.1.clone(),préparation.3);


            let option = récuperer_détails(mode, préparation.1.len());


            jeux.jouer(option);
        }


        if !rejouer(){
            break;
        }

    }
}


fn récuperer_détails(mode: Mode, limite: usize) -> Option<usize> {

    match mode {
        Mode::Classique => {
            Some(demander_nb_manche(limite))
        }
        Mode::Chronomètre => {
            Some(demander_temp())
        }//demander temp
        _ => None
    }
}

fn mode_de_jeu() -> Mode {
    afficher_str("Classique ? Chronomètre ? Survie ?");

    match demander().as_str() {
        "classique"  | "1" | "cl" => {
            Mode::Classique
        }
        "chronomètre" | "2" | "ch"  => {
            Mode::Chronomètre
        }
        "survie" | "3" | "s" | "su" => Mode::Survie,

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