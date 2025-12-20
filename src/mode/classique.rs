use serde::{Deserialize, Serialize};
use crate::jeux::Jeux;
use crate::joueur::Joueur;
use crate::outils::mot::Question;
use crate::outils::outils::demander;
use crate::outils::terminal::{afficher, afficher_str};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Classique {
    joueur: Joueur,
    question: Question,
    nb_question: Option<usize>
}


impl Jeux for Classique {
    fn get_joueur(&self) -> &Joueur {
        &self.joueur
    }

    fn get_joueur_mut(&mut self) -> &mut Joueur {
        &mut self.joueur
    }

    fn get_nb_question(&mut self) -> usize {
        if self.nb_question.is_some() {
            self.nb_question.unwrap()
        }
        else {
            self.nb_question = Some(self.demander_nb_manche(*self.question.nb_questions()));
            self.nb_question.unwrap()
        }

    }

    fn get_string_mode(&self) -> &str {
        "classique"
    }

    fn quel_est_la_question(&mut self) -> Option<(String, String)> {
        self.question.next()
    }
}

impl Classique {

    fn demander_nb_manche(&self, taille_liste: usize) -> usize {
        loop {

            afficher_str("Combien de manche ? ");
            let min = if taille_liste < usize::MAX {
                taille_liste
            } else {
                usize::MAX
            };
            afficher(format!("Nombre max de manches : {}", min.to_string()));
            let entree = demander();


            match entree.parse::<usize>() {
                Ok(num) => {
                    if num <= min {
                        return num
                    }
                }, //  Retourne le nombre valide et quitte la boucle si le nombre n’est pas trop grand, sinon on va dépasser la taille de la liste
                Err(_) => afficher_str("Entrée invalide, veuillez entrer un nombre entier positif."),
            }
        }
    }
    pub fn nouveau(joueur: Joueur, question: Question) -> Classique {
        Classique{joueur, question, nb_question: None }
    }


}