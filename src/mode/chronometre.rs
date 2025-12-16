use std::time::{Duration, Instant};
use crate::affichage::affichage::Affichage;
use crate::jeux::Jeux;
use crate::outils::outils::{demander_réponse};

pub fn chronomètre(jeux:&mut Jeux, durée: usize, affichage : &Box<dyn Affichage>) -> (usize, usize){

    let début = Instant::now();
    let fin = début + Duration::from_secs(durée as u64);

    loop {
        if Instant::now() >= fin {
            affichage.afficher_str("Le temps est passé !");
            affichage.afficher_score_fin(jeux.joueur.clone());
            return (jeux.joueur.bonne_reponse(),jeux.joueur.mauvaise_reponse())
        }
        joue_une_manche(jeux,jeux.nb_max_manche,fin, affichage);
    }
}



fn joue_une_manche(jeux:&mut Jeux, nb_manche_total:usize, fin:Instant, affichage : &Box<dyn Affichage>) -> bool {

    jeux.affiche_info(nb_manche_total, affichage);
    let mot = jeux.détermine_mot();
    let mut liste_essai:Vec<String> = vec!();

    loop {  //tant que le mot n'as pas été passer, ou stop
        let réponse = demander_réponse(&mut liste_essai, &mot.chars().count(), Option::from(fin)).unwrap();

        match réponse.as_str() {
            "stop" | "s" => {
                affichage.afficher_str("\n");
                return true;
            }

            "indice" | "i" => {
                affichage.afficher_indice(&mot);
            }

            "passe" | "p" => {
                jeux.joueur.question_suivante();
                jeux.joueur.mauvaise_reponse_aj();
                affichage.afficher_reponse_precedante(&mot);
                return false;
            }

            _ if réponse == mot => { // Si la réponse est égale au mot attention au \n
                jeux.joueur.bonne_reponse_aj();
                jeux.joueur.question_suivante();
                affichage.afficher_bonne_reponse();
                return false;
            }

            _ if réponse.trim() == "" => (),

            _ => {  // Cas pour mauvaise réponse
                jeux.joueur.mauvaise_reponse_aj();
                affichage.afficher_mauvaise_reponse();
            }
        }

        liste_essai.push(réponse);

    }

}