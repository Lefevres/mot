use crate::joueur::Joueur;
use crate::outils::outils::demander_réponse;
use crate::outils::terminal::{afficher_bonne_reponse, afficher_en_tete, afficher_indice, afficher_mauvaise_reponse, afficher_question, afficher_reponse_precedante, afficher_score, afficher_score_fin, afficher_str};

pub enum Mode {
    Classique,
    Chronomètre,
    Survie,
}

pub struct Jeux<'a> {
    mode: Mode,
    joueur: &'a mut Joueur,
    liste: Vec<(String, String)>,
    nb_manche: usize,
}


impl Jeux<'_> {
    pub fn nouveau(mode: Mode, joueur: &mut Joueur, liste: Vec<(String,String)>, nb_manche: usize) -> Jeux<'_> {
        Jeux{mode, joueur, liste, nb_manche}
    }

    pub fn jouer(&mut self) -> (usize,usize){
        match self.mode {
            Mode::Classique => {
                self.classique()
            }

            Mode::Chronomètre => {
                self.chronomètre()
            }
            _ => (0,0)

        }

    }


    fn chronomètre(&mut self) -> (usize,usize){
        afficher_str("il ne se passe pas grand chose pour le moment");
        (0,0)
    }





    fn classique(&mut self) -> (usize,usize){
        while !self.joueur.fin(self.nb_manche) {
            if self.joue_une_manche(){
                break;
            }
        }


        afficher_score_fin(self.joueur);

        (self.joueur.bonne_reponse(),self.joueur.mauvaise_reponse())
    }

    fn joue_une_manche(&mut self) -> bool {

        self.affiche_info();
        let mot = self.détermine_mot();
        let mut liste_essai:Vec<String> = vec!();

        loop {  //tant que le mot n'as pas été passer, ou stop
            let réponse = demander_réponse(&mut liste_essai,&mot.chars().count()).unwrap();

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


    fn affiche_info(&self) {
        afficher_en_tete();
        afficher_score(&self.joueur, self.nb_manche);
        afficher_question(self.joueur.question(),&self.liste);
    }


    fn détermine_mot(&self) -> String {
        self.liste[self.joueur.question()].0.clone()
    }

}