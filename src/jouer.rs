use tokio::time;
use crate::joueur::Joueur;
 use crate::outils::outils::{demander_réponse};
 use crate::outils::terminal::{afficher_bonne_reponse, afficher_en_tete, afficher_indice, afficher_mauvaise_reponse, afficher_question, afficher_reponse_precedante, afficher_score, afficher_score_fin, afficher_temp};


pub fn jouer(joueur: &mut Joueur, liste: &Vec<String>, nb_manche: usize) -> (usize, usize, bool) {
    let mut stop = false;
    let temp_début = time::Instant::now();

    while !joueur.fin(nb_manche) && !stop {
        stop = manche(joueur, liste, nb_manche);
    }

    let temp = temp_début.elapsed();
    afficher_score_fin(joueur);
    afficher_temp(temp);
    (joueur.bonne_reponse(),joueur.mauvaise_reponse(),stop)
}


fn manche(joueur: &mut Joueur, liste: &Vec<String>,nb_manche: usize) -> bool {
    let mut essai = false;
    afficher_en_tete();
    afficher_score(joueur,nb_manche);
    let mot = afficher_question(joueur.question(), &liste);
    let mut liste_essai:Vec<String> = vec![];
    while !essai { //syncroniser les résultats pour le multi ?
        let réponse = demander_réponse(&mut liste_essai,mot.chars().count()).unwrap();
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
        "stop" | "s" => {
            "stop".to_string()
        }
        "indice" | "i" => {
            afficher_indice(mot);
            "reposer".to_string()
        }
        "passe" | "p" => {
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
        _ if reponse.trim() == "" => "reposer".to_string(),
        _ => {  // Cas pour mauvaise réponse
            joueur.mauvaise_reponse_aj();
            afficher_mauvaise_reponse();
            "reposer".to_string()
        }
    }
}