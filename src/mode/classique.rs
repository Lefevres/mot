use crate::jeux::Jeux;
use crate::outils::outils::demander_nb_manche;
use crate::outils::terminal::afficher_score_fin;

pub fn classique(jeux: &mut Jeux) -> (usize, usize){
    let nb_manche = demander_nb_manche(jeux.nb_max_manche);
    while !jeux.joueur.fin(nb_manche) {
        if jeux.joue_une_manche(nb_manche) {
            break;
        }
    }

    afficher_score_fin(jeux.joueur);

    (jeux.joueur.bonne_reponse(),jeux.joueur.mauvaise_reponse())
}