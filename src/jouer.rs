 use std::io;
 use crate::affichage::affichage::Affichage;
 use crate::joueur::Joueur;
 use crate::outils::outils::demander;
 use crate::preparation::demander_nb_manche;

 pub fn jouer(joueur: &mut Joueur, affichage: &dyn Affichage, liste: &Vec<String>, nb_manche: usize) -> (usize, usize) {
    let mut stop = false;
    while !joueur.fin(nb_manche) && !stop {
        stop = manche(joueur, affichage, liste);
    }
    affichage.afficher_score(joueur);
    (joueur.bonne_reponse(),joueur.mauvaise_reponse())
}


fn manche(joueur: &mut Joueur, affichage: &dyn Affichage, liste: &Vec<String>) -> bool {
    let mut essai = false;
    affichage.afficher_en_tete();
    affichage.afficher_score(joueur);
    let mot = affichage.afficher_question(joueur.question(), &liste);
    while !essai {
        let reponse = attendre_réponse();
        let reaction = réagir(joueur, affichage, &reponse, &mot);
        match reaction.as_str() {
            "stop" => {
                return true;  //on arrete bel et bien
            }

            "suivant" => {
                essai = true;  //l'essai est correcte
            }

            "reposer" => {}

            _ => {
                println!("comment on en est arrivé là ?");
            }
        }
    }
    false
}


 fn attendre_réponse() -> String {
     demander(String::new())
}


fn réagir(joueur: &mut Joueur, affichage: &dyn Affichage, reponse: &String, mot: &String) -> String {
    match reponse.as_str() {
        "stop" => {
            "stop".to_string()
        }
        "indice" => {
            affichage.afficher_indice(mot);
            "reposer".to_string()
        }
        "passe" => {
            joueur.question_suivante();
            joueur.mauvaise_reponse_aj();
            affichage.afficher_reponse_precedante(mot);
            "suivant".to_string()
        }
        _ if reponse == mot => { // Si la réponse est égale au mot attention au \n
            joueur.bonne_reponse_aj();
            joueur.question_suivante();
            affichage.afficher_bonne_reponse();
            "suivant".to_string()
        }
        _ => {  // Cas pour mauvaise réponse
            joueur.mauvaise_reponse_aj();
            affichage.afficher_mauvaise_reponse();
            "reposer".to_string()
        }
    }
}
