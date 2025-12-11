use crate::jeux::Jeux;
use crate::outils::terminal::afficher_score_fin;

pub fn classique(jeux: &mut Jeux, nb_question: usize) -> (usize, usize){
    
    while !jeux.joueur.fin(nb_question) {
        if jeux.joue_une_manche(nb_question) {
            break;
        }
    }

    afficher_score_fin(jeux.joueur.clone());

    (jeux.joueur.bonne_reponse(),jeux.joueur.mauvaise_reponse())
}