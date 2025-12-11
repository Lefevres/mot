use crate::jeux::Jeux;
use crate::outils::terminal::afficher_score_fin;

pub fn classique(jeux: &mut Jeux, nb_question: usize) -> (usize, usize){
    /*if nb_question.is_none() {
        nb_question = Some(demander_nb_manche(jeux.nb_max_manche));
    }*/
    while !jeux.joueur.fin(nb_question) {
        if jeux.joue_une_manche(nb_question) {
            break;
        }
    }

    afficher_score_fin(jeux.joueur.clone());

    (jeux.joueur.bonne_reponse(),jeux.joueur.mauvaise_reponse())
}