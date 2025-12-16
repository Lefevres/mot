use std::io::stdout;
use colored::Colorize;
use crate::affichage::affichage::Affichage;
use crate::joueur::Joueur;
use crate::outils::outils::demander;


pub struct AffichageTerminal;

impl Affichage for AffichageTerminal {
    fn afficher_en_tete(&self){
        crossterm::execute!(stdout(), crossterm::terminal::Clear(crossterm::terminal::ClearType::All)).unwrap();
        let en_tete = "\n\nLes mots sont sans majuscule (sauf pour les noms propre) mais avec accent, les locutions latine ne prenne pas de tiret".green();
        let indices = "\n\
    {indice/i} pour avoir le nombre de lettres\n\
    {passe/p} pour changer de mot\n\
    {stop/s} pour arrêter".red();
        println!("{en_tete}\n{indices}");
    }

    fn afficher_question(&self, position : usize, liste : &Vec<(String, String)>){  //renvoie le mot attendu
        let question = format!("{}", liste[position].1).bright_yellow();
        println!("\n\n{}\n\n\n", question ); //nb_question.0 est la réponse, nb_question.1 est la question
    }


    fn afficher_indice(&self, mot: &String) {
        let len = mot.chars().count();
        let revelation = len / 3;

        let prefix: String = mot.chars().take(revelation).collect();

        let mut indice = prefix;
        for _ in 0..(len - revelation) {
            indice.push_str("_ ");
        }

        println!("\n\nC’est un mot de {} lettres", len);
        println!("{}", indice);
    }

    fn afficher_reponse_precedante(&self, mot : &String){
        println!("\nLa réponse était {}\n",mot.green());
        self.attendre_validation();
    }

    fn afficher_bonne_reponse(&self) {
        println!("\n\nFélicitation c'est une bonne réponse 👍\n");
        self.attendre_validation();
    }



    fn afficher_mauvaise_reponse(&self) {
        println!("\nÇa n'est malheureusement pas ça \n");
    }

    fn afficher_score(&self, joueur: &Joueur, nb_manche: usize) {

        let total = joueur.bonne_reponse() + joueur.mauvaise_reponse();
        let ratio = if total > 0 {
            (joueur.bonne_reponse() as f32 / total as f32) * 100.0
        } else {
            0.0
        };
        println!(
            "\nScore : {} bonne(s) réponse(s), {} mauvaise(s) réponse(s) — Ratio : {:.1}%            Question {}/{}",
            joueur.bonne_reponse(), joueur.mauvaise_reponse(), ratio, joueur.question()/2+1,nb_manche
        );

    }




    fn afficher(&self, texte: String) {
        println!("{}", texte);
    }



    fn afficher_str(&self, texte: &str) {
        println!("{}", texte);
    }


    fn afficher_score_fin(&self, joueur: Joueur) {

        let total = joueur.bonne_reponse() + joueur.mauvaise_reponse();
        let ratio = if total > 0 {
            (joueur.bonne_reponse() as f32 / total as f32) * 100.0
        } else {
            0.0
        };
        println!(
            "\nScore : {} bonne(s) réponse(s), {} mauvaise(s) réponse(s) — Ratio : {:.1}%",
            joueur.bonne_reponse(), joueur.mauvaise_reponse(), ratio
        );
    }



    fn attendre_validation(&self){
        println!("Entrer pour continuer : \n");
        demander();
    }

}






