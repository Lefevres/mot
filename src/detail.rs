use crate::affichage::affichage::Affichage;
use crate::jeux::{Jeux, Mode};
use crate::multi_joueur::client::client;
use crate::multi_joueur::hote::hote;
use crate::outils::outils::{demander, demander_nb_manche, demander_temp, se_préparer};

pub fn gère(affichage : &Box<dyn Affichage>) {

    loop {

        if est_ce_multi(affichage) {
            match choisir_le_role(affichage) {
                true => {
                    let mode = mode_de_jeu(affichage);
                    hote(mode, affichage);
                }

                false => {
                    client(affichage);
                }
            }

        }else {
            let mode = mode_de_jeu(affichage);
            let mut préparation = se_préparer("solitaire",affichage);
            let mut jeux = Jeux::nouveau(mode.clone(), préparation.0, préparation.1.clone(),préparation.3);


            let option = récuperer_détails(mode, préparation.1.len(), affichage);


            jeux.jouer(option,affichage);
        }


        if !rejouer(affichage){
            break;
        }

    }
}


fn récuperer_détails(mode: Mode, limite: usize, affichage : &Box<dyn Affichage>) -> Option<usize> {

    match mode {
        Mode::Classique => {
            Some(demander_nb_manche(limite, affichage))
        }
        Mode::Chronomètre => {
            Some(demander_temp(affichage))
        }//demander temp
        _ => None
    }
}

fn mode_de_jeu(affichage : &Box<dyn Affichage>) -> Mode {
    affichage.afficher_str("Classique ? Chronomètre ? Survie ?");

    match demander().as_str() {
        "classique"  | "1" | "cl" => {
            Mode::Classique
        }
        "chronomètre" | "2" | "ch"  => {
            Mode::Chronomètre
        }
        "survie" | "3" | "s" | "su" => Mode::Survie,

        _ => {
            affichage.afficher_str("bon… bha on va dire classique alors…");
            Mode::Classique
        }
    }
}


fn est_ce_multi(affichage : &Box<dyn Affichage>) -> bool{

    loop {
        affichage.afficher_str("Mode de jeu : solitaire ou multi_joueur ?");
        let mode = demander();
        match mode.trim() {
            "solitaire" | "1" | "s" | "S" | "SOLITAIRE" => {
                return false
            }
            "multi_joueur" | "2" | "m" | "M" | "MULTIJOUEUR" => {
                return true
            }
            _ => {
                affichage.afficher_str("N'importe quoi !!");

            }
        }
    }

}


fn choisir_le_role(affichage : &Box<dyn Affichage>) -> bool {
    affichage.afficher_str("Role : hote ou client");
    loop {
        match demander().trim() {
            "hote" | "h" | "1" => {
                return true
            }

            "client" | "c" | "2" => {
                return false
            }

            _ => {
                affichage.afficher_str("N'importe quoi !!");
            }

        }
    }

}


fn rejouer(affichage : &Box<dyn Affichage>) -> bool{
    affichage.afficher(String::from("\n\nrejouer ? "));

    loop{
        let réponse = demander();
        match réponse.as_str() {
            "oui" | "o" =>  return true,
            "non" | "n" => return false,
            _ => affichage.afficher_str("si tu n'arrive même pas a répondre a une question aussi simple tu n'es pas prêt pour la suite"),
        }
    }

}