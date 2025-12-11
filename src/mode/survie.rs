use crate::jeux::Jeux;
use crate::joueur::Joueur;
use crate::outils::outils::demander_réponse;
use crate::outils::terminal::{afficher, afficher_bonne_reponse, afficher_indice, afficher_reponse_precedante, afficher_str};

pub fn survie(jeux: &mut Jeux) -> (usize, usize) {
    loop {
       if joue_une_manche(jeux) {
            break;
       }
    }

    afficher_score_fin(&mut jeux.joueur);
    (jeux.joueur.bonne_reponse(),jeux.joueur.mauvaise_reponse())
}



fn afficher_score_fin(joueur: &mut Joueur) {
    if joueur.bonne_reponse() > 1 {
        afficher(format!("Bravo tu as tenue {} manches",joueur.bonne_reponse()));
    }else {
        afficher(format!("Bravo tu as tenue {} manche",joueur.bonne_reponse()));
    }

}


fn joue_une_manche(jeux:&mut Jeux) -> bool {

    jeux.affiche_info(jeux.nb_max_manche);
    let mot = jeux.détermine_mot();
    let mut liste_essai:Vec<String> = vec!();

    loop {  //tant que le mot n'as pas été passer, ou stop
        let réponse = demander_réponse(&mut liste_essai,&mot.chars().count(),None).unwrap();

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
                return true;
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
                afficher_reponse_precedante(&mot);
                return true;
            }
        }

        liste_essai.push(réponse);

    }

}