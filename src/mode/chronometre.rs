use std::time::Duration;
use crate::jeux::Jeux;
use crate::outils::outils::demander;
use crate::outils::terminal::{afficher_score_fin, afficher_str};

pub fn chronomètre(jeux:&mut Jeux) -> (usize, usize){
    afficher_str("Combien de seconde ?");
    let temp = Duration::from_secs(demander().parse().unwrap());
    let début = std::time::Instant::now();


    loop {
        if début.elapsed() >= temp {
            afficher_str("Le temp est passer !");
            afficher_score_fin(jeux.joueur);
            return (jeux.joueur.bonne_reponse(),jeux.joueur.mauvaise_reponse())
        }
        jeux.joue_une_manche(0);
    }
}