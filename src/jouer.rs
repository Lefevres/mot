 use std::io;
use crate::affichage::affichage::Affichage;
use crate::joueur::Joueur;

pub fn jouer(joueur: &mut Joueur, affichage: &dyn Affichage, liste: &Vec<String>, nb_manche : usize) {
    let mut stop = false;
    while !joueur.fin(nb_manche) && !stop{
        stop = manche(joueur,affichage, liste);
    }
}
fn manche(joueur : &mut Joueur,affichage: &dyn Affichage, liste : &Vec<String>) -> bool{
    let mut essai = false;  //essai incorrecte
    affichage.afficher_en_tete();  //pas de retour donc je peux utiliser afficher_en_tete en méthode sans problème
    let mot = affichage.afficher_question(joueur.question(),&liste);  //retour donc pénible
    while !essai {
        let reponse = attendre_reponse();
        let reaction = reagir(joueur,affichage,&reponse,&mot);

        match reaction.as_str() {
            "stop" => {
                return true;  //on arrete bel et bien
            }

            "suivant" => {
                essai = true;  //l'essai est correcte
            }

            "reposer" => {

            }

            _ => {
                println!("comment on en est arrivé là ?");
            }
        }
    }
    false

}

fn  attendre_reponse() -> String{
    let mut saisie = String :: new();
    io::stdin()
        .read_line(&mut saisie)
        .expect("Erreur lors de la lecture");

    saisie.trim().to_string()
}

fn reagir(joueur: &mut Joueur,affichage : &dyn Affichage, reponse: &String, mot: &String) -> String {
    match reponse.as_str() {
        "stop" => {
            "stop".to_string()
        }
        "indice" => {
            affichage.afficher_indice(mot);
            "reposer".to_string()
        }
        _ if reponse == mot => { // Si la réponse est égale au mot attention au \n
            joueur.bonne_reponse_aj();
            joueur.question_suivante();
            println!("Bonne réponse");
            "suivant".to_string()
        }
        _ => {  // Cas pour mauvaise réponse
            joueur.mauvaise_reponse_aj();
            println!("Mauvaise réponse");
            "reposer".to_string()
        }
    }
}