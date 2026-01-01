use crate::jeux::Jeux;
use crate::outils::outils::demander;
use crate::outils::terminal::{afficher, afficher_score_fin, afficher_str};

pub fn classique(jeux: &mut Jeux, nb_question: usize) -> (usize, usize){
    
    while !jeux.joueur.fin(nb_question) {
        if jeux.joue_une_manche(nb_question) {
            break;
        }
    }

    afficher_score_fin(jeux.joueur.clone());

    (jeux.joueur.bonne_reponse(),jeux.joueur.mauvaise_reponse())
}

/// Fonction déterminant le nombre de manche qui serons jouer en mode Classique.
///
/// # Paramètre
/// - Le nombre de question maximum sous forme d’usize.
///
/// # Retour
/// - le nombre de question sous forme d’usize.
///
/// # Comportement
/// Demande au joueur le nombre de manche souhaiter,
/// détermine le plus petit entre le nombre de question maximal passer en paramètre et le nombre maximum pouvant être contenu dans un usize (sait on jamais),
/// vérifie si l’entrée utilisateur est bien convertible en nombre sinon redemande.
///
pub fn demander_nb_manche(taille_liste: usize) -> usize {
    loop {

        afficher_str("Combien de manche ? ");
        let max = if taille_liste < usize::MAX {
            taille_liste
        } else {
            usize::MAX
        };
        afficher(format!("Nombre max de manches : {}", max.to_string()));
        let entree = demander();


        match entree.parse::<usize>() {
            Ok(num) => {
                if num <= max {
                    return num
                }
            }, //  Retourne le nombre valide et quitte la boucle si le nombre n’est pas trop grand, sinon on va dépasser la taille de la liste
            Err(_) => afficher_str("Entrée invalide, veuillez entrer un nombre entier positif."),
        }
    }
}