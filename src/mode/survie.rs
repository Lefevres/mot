use crate::jeux::Jeux;
use crate::joueur::Joueur;
use crate::outils::outils::demander_réponse;
use crate::outils::terminal::{afficher, afficher_bonne_reponse, afficher_indice, afficher_reponse_precedante, afficher_str};

pub fn survie(jeux: &mut Jeux) -> (usize, usize) {
    loop {
       if joue_une_manche(jeux) {
            jeux.devrais_je_arreter = true;
            break;
       }
    }

    afficher_score_fin(&mut jeux.joueur);
    (jeux.joueur.bonne_reponse(),jeux.joueur.mauvaise_reponse())
}



fn afficher_score_fin(joueur: &mut Joueur) {
    let manche:&str;
    if joueur.bonne_reponse() > 1 {
        manche = "manches";
    }else {
        manche = "manche";
    }
    afficher(format!("Bravo tu as tenue {} {}",joueur.bonne_reponse(), manche));

}


fn joue_une_manche(jeux:&mut Jeux) -> bool {


    let (mot,question) = jeux.détermine_mot();
    jeux.affiche_info(jeux.nombre_question(),&question, mot.len());
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