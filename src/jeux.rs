use serde::{Deserialize, Serialize};
use crate::joueur::Joueur;
use crate::outils::mot::Question;
use crate::outils::outils::demander_réponse;
use crate::outils::terminal::{afficher_bonne_reponse, afficher_en_tete, afficher_indice, afficher_mauvaise_reponse, afficher_question, afficher_reponse_precedante, afficher_score, afficher_score_fin, afficher_str};
use crate::mode::chronometre::Chronomètre;
use crate::mode::classique::Classique;


pub trait Jeux {
    /*fn nouveau(mode: Mode, joueur: Joueur, question: Question, est_multi : bool) -> Jeux {
        Jeux{mode, joueur, question, est_multi }
    }*/

    fn get_joueur(&self) -> &Joueur;
    fn get_joueur_mut(&mut self) -> &mut Joueur;
    fn get_nb_question(&self) -> &usize;
    fn quel_est_la_question(&mut self) -> Option<(String,String)>;


    fn jouer(&mut self) -> (usize, usize) {
        while !self.get_joueur().fin(self.get_nb_question()) {
            let nb_manche = self.get_nb_question().clone();
            if self.joue_une_manche(&nb_manche, None) {
                break;
            }
        }

        afficher_score_fin(self.get_joueur().clone());

        (self.get_joueur().bonne_reponse(), self.get_joueur().mauvaise_reponse())
    }


    fn stop(&self) -> bool {
        afficher_str("\n");
        true
    }


    fn indice(&self, mot: &String) {
        afficher_indice(&mot);
    }


    fn passe(&mut self, mot: &String) -> bool {
        self.get_joueur_mut().question_suivante();
        self.get_joueur_mut().mauvaise_reponse_aj();
        afficher_reponse_precedante(&mot);
        false
    }


    fn bonne_réponse(&mut self) -> bool {
        self.get_joueur_mut().bonne_reponse_aj();
        self.get_joueur_mut().question_suivante();
        afficher_bonne_reponse();
        false
    }


    fn mauvaise_reponse(&mut self) {
        self.get_joueur_mut().mauvaise_reponse_aj();
        afficher_mauvaise_reponse();
    }


    fn joue_une_manche(&mut self, nb_manche_total: &usize, fin: std::option::Option<std::time::Instant>) -> bool {
        let (mot, question) = self.quel_est_la_question().unwrap();
        self.affiche_info(nb_manche_total, &question);

        let mut liste_essai: Vec<String> = vec!();

        loop {  //tant que le mot n'as pas été passer, ou stop
            let réponse = demander_réponse(&mut liste_essai, &mot.chars().count(), fin).unwrap();

            match réponse.as_str() {
                "stop" | "s" => {
                    return self.stop()
                }

                "indice" | "i" => {
                    self.indice(&mot);
                }

                "passe" | "p" => {
                    return self.passe(&mot);
                }

                _ if réponse == mot => { // Si la réponse est égale au mot attention au \n
                    return self.bonne_réponse()
                }

                _ if réponse.trim() == "" => (),

                _ => {  // Cas pour mauvaise réponse
                    self.mauvaise_reponse()
                }
            }

            liste_essai.push(réponse);
        }
    }


    fn affiche_info(&self, nb_manche: &usize, question: &String) {
        afficher_en_tete();
        afficher_score(&self.get_joueur(), nb_manche);
        afficher_question(question);
    }




    fn nombre_question(&self) -> &usize {
        self.get_nb_question()
    }

}

