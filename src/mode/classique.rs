use crate::affichage::affichage::Affichage;
use crate::jeux::Jeux;


pub fn classique(jeux: &mut Jeux, nb_question: usize, affichage : &Box<dyn Affichage>) -> (usize, usize){
    
    while !jeux.joueur.fin(nb_question) {
        if jeux.joue_une_manche(nb_question, affichage) {
            break;
        }
    }

    affichage.afficher_score_fin(jeux.joueur.clone());

    (jeux.joueur.bonne_reponse(),jeux.joueur.mauvaise_reponse())
}