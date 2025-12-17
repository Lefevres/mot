use std::time::{Duration, Instant};
use crate::jeux::Jeux;
use crate::outils::outils::{demander_réponse};
use crate::outils::terminal::{afficher_bonne_reponse, afficher_indice, afficher_mauvaise_reponse, afficher_reponse_precedante, afficher_score_fin, afficher_str};

pub fn chronomètre(jeux:&mut Jeux, durée: usize) -> (usize, usize){

    let début = Instant::now();
    let fin = début + Duration::from_secs(durée as u64);

    loop {
        if Instant::now() >= fin {
            afficher_str("Le temps est passé !");
            afficher_score_fin(jeux.joueur.clone());
            return (jeux.joueur.bonne_reponse(),jeux.joueur.mauvaise_reponse())
        }
        joue_une_manche(jeux,jeux.nombre_question(),fin);
    }
}



fn joue_une_manche(jeux:&mut Jeux,nb_manche_total:usize,fin:Instant) -> bool {


    let (mot, question) = jeux.détermine_mot();
    jeux.affiche_info(nb_manche_total,&question);
    let mut liste_essai:Vec<String> = vec!();

    loop {  //tant que le mot n'as pas été passer, ou stop
        let réponse = demander_réponse(&mut liste_essai, &mot.chars().count(), Option::from(fin)).unwrap();

        match réponse.as_str() {
            "stop" | "s" => {
                afficher_str("\n");
                return true;
            }

            "indice" | "i" => {
                afficher_indice(&mot);
            }

            "passe" | "p" => {
                jeux.joueur.question_suivante();
                jeux.joueur.mauvaise_reponse_aj();
                afficher_reponse_precedante(&mot);
                return false;
            }

            _ if réponse == mot => { // Si la réponse est égale au mot attention au \n
                jeux.joueur.bonne_reponse_aj();
                jeux.joueur.question_suivante();
                afficher_bonne_reponse();
                return false;
            }

            _ if réponse.trim() == "" => (),

            _ => {  // Cas pour mauvaise réponse
                jeux.joueur.mauvaise_reponse_aj();
                afficher_mauvaise_reponse();
            }
        }

        liste_essai.push(réponse);

    }

}