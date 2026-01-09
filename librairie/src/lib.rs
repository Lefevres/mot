use crate::jeux::Mode;
use crate::mode::classique::classique;
use crate::multi_joueur::client::client;
use crate::multi_joueur::hote::hote;
use crate::outils::mot;
use crate::outils::terminal::afficher_str;
use crate::outils::outils::{crée_partie, demander, rejouer};


mod joueur;
mod multi_joueur;
pub mod outils;
mod jeux;
pub mod mode;

pub fn main(multi: bool, mode: String, detail: usize, role: String, nom: String ) {
    println!("le multi est {multi}, le mode est {mode}, le detail est {detail}, le role est {role}, le nom est {nom}");
    if !multi{
        let mode_v = match mode.as_str() {
            "classique" => Mode::nouveau_simple("classique", Some(detail)).unwrap(),
            "chronomètre" => Mode::nouveau_simple("chronomètre", Some(detail)).unwrap(),
            "survie" => Mode::nouveau_simple("survie", Some(detail)).unwrap(),
            _ => Mode::nouveau_simple("classique", Some(detail)).unwrap(),
        };

        let mut jeux = crée_partie(None, Some(mode_v), None);

        jeux.jouer();
    }
    else {
        match role.as_str() {
            "hote" => {
                hote();
            }

            "client" => {
                client();
            }
            _ => client(),
        }
    }
}







fn est_ce_multi() -> bool{

    loop {
        let mode = demander(Some("Mode de jeu : solitaire ou multi_joueur ?"));
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
    loop {
        match demander(Some("Role : hote ou client")).trim() {
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


