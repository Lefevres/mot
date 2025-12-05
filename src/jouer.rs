 use crate::joueur::Joueur;
 use crate::outils::outils::demander;
 use crate::outils::terminal::{afficher_bonne_reponse, afficher_en_tete, afficher_indice, afficher_mauvaise_reponse, afficher_question, afficher_reponse_precedante, afficher_score};

 pub fn jouer(joueur: &mut Joueur, liste: &Vec<String>, nb_manche: usize) -> (usize, usize) {
    let mut stop = false;
    while !joueur.fin(nb_manche) && !stop {
        stop = manche(joueur, liste, nb_manche);
    }
    afficher_score(joueur,nb_manche);
    (joueur.bonne_reponse(),joueur.mauvaise_reponse())
}


fn manche(joueur: &mut Joueur, liste: &Vec<String>,nb_manche: usize) -> bool {
    let mut essai = false;
    afficher_en_tete();
    afficher_score(joueur,nb_manche);
    let mot = afficher_question(joueur.question(), &liste);
    let mut liste_essai:Vec<String> = vec![];
    while !essai { //syncroniser les résultats pour le multi ?
        let réponse = demander(String::new());
        let reaction = réagir(joueur, &réponse, &mot);
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
        liste_essai.push(réponse);
    }
    false
}

 
 fn réagir(joueur: &mut Joueur, reponse: &String, mot: &String) -> String {
    match reponse.as_str() {
        "stop" => {
            "stop".to_string()
        }
        "indice" => {
            afficher_indice(mot);
            "reposer".to_string()
        }
        "passe" => {
            joueur.question_suivante();
            joueur.mauvaise_reponse_aj();
            afficher_reponse_precedante(mot);
            "suivant".to_string()
        }
        _ if reponse == mot => { // Si la réponse est égale au mot attention au \n
            joueur.bonne_reponse_aj();
            joueur.question_suivante();
            afficher_bonne_reponse();
            "suivant".to_string()
        }
        _ => {  // Cas pour mauvaise réponse
            joueur.mauvaise_reponse_aj();
            afficher_mauvaise_reponse();
            "reposer".to_string()
        }
    }
}