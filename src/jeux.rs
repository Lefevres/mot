use crate::joueur::Joueur;
use crate::outils::outils::demander_réponse;
use crate::outils::terminal::{afficher_en_tete, afficher_indice, afficher_question, afficher_reponse_precedante, afficher_score, afficher_score_fin, afficher_str};

enum Mode {
    Classique,
    Chronomètre,
}

struct Jeux<'a> {
    mode: Mode,
    joueur: &'a mut Joueur,
    liste: Vec<(String, String)>,
    nb_manche: usize,
}


impl Jeux<'_> {
    pub fn nouveau(mode: Mode, joueur: &mut Joueur, liste: Vec<(String,String)>, nb_manche: usize) -> Jeux {
        Jeux{mode, joueur, liste, nb_manche}
    }

    pub fn jouer(&mut self) {

        while !self.joueur.fin(self.nb_manche) {
            if self.joue_une_manche(){
                return;
            }
        }

        //affiche les résultats ?
        afficher_score_fin(self.joueur);
    }

    fn joue_une_manche(&mut self) -> bool {

        self.affiche_info();
        let mot = self.détermine_mot();
        let mut liste_essai:Vec<String> = vec!();

        loop {  //tant que le mot n'as pas été passer, ou stop
            let réponse = demander_réponse(&mut liste_essai,&mot.chars().count()).unwrap();

            match réponse.as_str() {
                "stop" | "s" => {
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
                _=> afficher_str("comment on en est arrivé là ?")
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
        self.liste[self.nb_manche].0.clone()
    }



}















