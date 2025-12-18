use serde::{Deserialize, Serialize};
use crate::jeux::{Jeux, Mode};
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

    fn get_question(&self) -> &Question {
        &self.question
    }

    fn get_mode(&self) -> &Mode {
        todo!()
    }
}

impl Classique {
    pub fn nouveau(joueur: Joueur, question: Question) -> Classique {
        Classique{joueur, question}
    }
}