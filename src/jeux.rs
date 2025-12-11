use serde::{Deserialize, Serialize};
use crate::joueur::Joueur;
use crate::mode::chronometre::chronomètre;
use crate::mode::classique::classique;
use crate::mode::survie::survie;
use crate::outils::outils::demander_réponse;
use crate::outils::terminal::{afficher_bonne_reponse, afficher_en_tete, afficher_indice, afficher_mauvaise_reponse, afficher_question, afficher_reponse_precedante, afficher_score, afficher_str};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Mode {
    Classique,
    Chronomètre,
    Survie,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Jeux {
    pub mode: Mode,
    pub joueur: Joueur,
    liste: Vec<(String, String)>,
    pub nb_max_manche: usize,
}


impl Jeux {
    pub fn nouveau(mode: Mode, joueur: Joueur, liste: Vec<(String,String)>, nb_max_manche: usize) -> Jeux {
        Jeux{mode, joueur, liste, nb_max_manche}
    }

    pub fn jouer(&mut self, nb_question: Option<usize>) -> (usize,usize){

        match self.mode {

            Mode::Classique => {
                if nb_question.is_some() {
                    classique(self, nb_question.unwrap())
                }else {
                    afficher_str("bein… y'a un problème");
                    (0,0)
                }


            }

            Mode::Chronomètre => {
                if nb_question.is_some() {
                    chronomètre(self, nb_question.unwrap())
                }else {
                    afficher_str("bein… y'a un problème");
                    (0,0)
                }
            }

            Mode::Survie => {
                survie(self)
            }

        }

    }


   pub fn joue_une_manche(&mut self,nb_manche_total:usize) -> bool {

        self.affiche_info(nb_manche_total);
        let mot = self.détermine_mot();
        let mut liste_essai:Vec<String> = vec!();

        loop {  //tant que le mot n'as pas été passer, ou stop
            let réponse = demander_réponse(&mut liste_essai,&mot.chars().count(),None).unwrap();

            match réponse.as_str() {
                "stop" | "s" => {
                    afficher_str("\n");
                    return true;
                }

                "indice" | "i" => {
                    afficher_indice(&mot);
                }

                "passe" | "p" => {
                    self.joueur.question_suivante();
                    self.joueur.mauvaise_reponse_aj();
                    afficher_reponse_precedante(&mot);
                    return false;
                }

                _ if réponse == mot => { // Si la réponse est égale au mot attention au \n
                    self.joueur.bonne_reponse_aj();
                    self.joueur.question_suivante();
                    afficher_bonne_reponse();
                    return false;
                }

                _ if réponse.trim() == "" => (),

                _ => {  // Cas pour mauvaise réponse
                    self.joueur.mauvaise_reponse_aj();
                    afficher_mauvaise_reponse();
                }
            }

            liste_essai.push(réponse);

        }

    }


    pub(crate) fn affiche_info(&self, nb_manche:usize) {
        afficher_en_tete();
        afficher_score(&self.joueur, nb_manche);
        afficher_question(self.joueur.question(),&self.liste);
    }


    pub(crate) fn détermine_mot(&self) -> String {
        self.liste[self.joueur.question()].0.clone()
    }

}