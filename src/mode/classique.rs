use serde::{Deserialize, Serialize};
use crate::jeux::Jeux;
use crate::joueur::Joueur;
use crate::outils::mot::Question;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Classique {
    joueur: Joueur,
    question: Question
}


impl Jeux for Classique {
    fn get_joueur(&self) -> &Joueur {
        &self.joueur
    }

    fn get_joueur_mut(&mut self) -> &mut Joueur {
        &mut self.joueur
    }

    fn get_nb_question(&self) -> &usize {
        &self.question.nb_questions()
    }

    fn quel_est_la_question(&mut self) -> Option<(String, String)> {
        self.question.next()
    }
}

impl Classique {
    pub fn nouveau(joueur: Joueur, question: Question) -> Classique {
        Classique{joueur, question}
    }
}