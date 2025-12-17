use serde::{Deserialize, Serialize};
use crate::joueur::Joueur;
use crate::mode::chronometre::chronomètre;
use crate::mode::classique::classique;
use crate::mode::survie::survie;
use crate::outils::mot::Question;
use crate::outils::outils::{demander_nb_manche, demander_réponse, demander_temp};
use crate::outils::terminal::{afficher_bonne_reponse, afficher_en_tete, afficher_indice, afficher_mauvaise_reponse, afficher_question, afficher_reponse_precedante, afficher_score, afficher_str};


#[derive(Serialize, Deserialize, Debug, Clone)]
enum ModeJeu {
    Classique,
    Chronomètre,
    Survie,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Mode{
    mode: ModeJeu,
    détail: Option<usize>,
}

impl Mode{
    pub fn nouveau(mode_jeu: &str) -> Option<Mode>{

        match mode_jeu {
            "classique" => Some(Mode{mode : ModeJeu::Classique, détail: Some(demander_nb_manche(300)) }),//limite
            "chronomètre" => Some(Mode{mode : ModeJeu::Chronomètre, détail : Some(demander_temp()) }),
            "survie" => Some(Mode{mode : ModeJeu::Survie, détail : None }),
            _ => {
                eprintln!("On as un problème");
                None
            }
        }
    }


}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Jeux {
    mode: Mode,
    pub(crate) joueur: Joueur,
    question: Question,
    est_multi: bool,
}


impl Jeux {

    pub fn nouveau(mode: Mode, joueur: Joueur, question: Question, est_multi : bool) -> Jeux {
        Jeux{mode, joueur, question, est_multi }
    }

    pub fn jouer(&mut self) -> (usize,usize){

        match self.mode.mode {

            ModeJeu::Classique => {
                if self.mode.détail.is_some() {
                    classique(self, self.mode.détail.unwrap())
                }else {
                    afficher_str("bein… y'a un problème");
                    (0,0)
                }


            }

            ModeJeu::Chronomètre => {
                if self.mode.détail.is_some() {
                    chronomètre(self, self.mode.détail.unwrap())
                }else {
                    afficher_str("bein… y'a un problème");
                    (0,0)
                }
            }

            ModeJeu::Survie => {
                    survie(self)

            }

        }

    }


   pub fn joue_une_manche(&mut self,nb_manche_total:usize) -> bool {
        let (mot,question) = self.détermine_mot();
        self.affiche_info(nb_manche_total,&question);

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


    pub(crate) fn affiche_info(&self, nb_manche:usize, question: &String) {
        afficher_en_tete();
        afficher_score(&self.joueur, nb_manche);
        afficher_question(question);
    }


    pub(crate) fn détermine_mot(&mut self) -> (String,String) {
        self.question.next().unwrap()
        //self.liste[self.joueur.question()].0.clone()
    }

    pub fn nombre_question(&self) -> usize {
        self.question.nb_questions()
    }


    pub fn mode(&self) -> &Mode {
        &self.mode
    }


    pub fn question(&self) -> &Question {
        &self.question
    }

}