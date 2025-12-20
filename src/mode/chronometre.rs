use std::time::{Duration, Instant};
use serde::{Deserialize, Serialize};
use crate::jeux::{Jeux};
use crate::joueur::Joueur;
use crate::mode::classique::Classique;
use crate::outils::mot::Question;
use crate::outils::outils::demander;
use crate::outils::terminal::{afficher_score_fin, afficher_str};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Chronomètre{
    joueur: Joueur,
    question: Question
}

impl Jeux for Chronomètre{
    fn get_joueur(&self) -> &Joueur {
        &self.joueur
    }

    fn get_joueur_mut(&mut self) -> &mut Joueur {
        &mut self.joueur
    }

    fn get_nb_question(&mut self) -> usize {
        *self.question.nb_questions()
    }

    fn get_string_mode(&self) -> &str {
        "chronomètre"
    }

    fn quel_est_la_question(&mut self) -> Option<(String, String)> {
        self.question.next()
    }


    fn jouer(&mut self) -> (usize, usize) {
        let début = Instant::now();
        let durée = demander_temps();
        let fin = début + Duration::from_secs(durée as u64);

        loop {
            if Instant::now() >= fin {
                afficher_str("Le temps est passé !");
                afficher_score_fin(self.get_joueur().clone());
                return (self.get_joueur().bonne_reponse(),self.get_joueur().mauvaise_reponse())
            }
            let nb_question = self.get_nb_question().clone();
            self.joue_une_manche(&nb_question, Some(fin));
        }
    }
}


impl Chronomètre {
    pub fn nouveau(joueur: Joueur, question: Question) -> Chronomètre {
        Chronomètre{joueur, question}
    }
}

pub fn demander_temps() -> usize{
    loop {
        afficher_str("Combien de secondes ?");
        let entrée = demander();

        match entrée.parse::<usize>() {
            Ok(num) => {
                return num
            }, //  Retourne le nombre valide et quitte la boucle si le nombre n’est pas trop grand, sinon on va dépasser la taille de la liste
            Err(_) => afficher_str("Entrée invalide, veuillez entrer un nombre entier positif."),
        }
    }

}