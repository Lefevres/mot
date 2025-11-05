use colored::Colorize;
use crate::affichage::affichage::Affichage;

pub struct AffichageTerminal;

impl Affichage for AffichageTerminal {
    fn afficher_en_tete(&self){
        let en_tete = "\n\nLes mots sont sans majuscule mais avec accent".green();
        let instruction = "\n +1 points pour une bonne réponse, -1 points pour une mauvaise réponse\n".red();
        let indices = "{indice} pour avoir le nombre de lettres\n{passe} pour changer de mot\n{stop} pour arrêter".red();
        println!("{en_tete}\n{instruction}\n{indices}");
    }

    fn afficher_question<'a>(&self, nb_question : usize, liste : &'a Vec<String>) -> &'a String{  //renvoie le mot attendu
        println!("{}", liste[nb_question+1]); //nb_question est la réponse, nb_question +1 est la question
        &liste[nb_question]
    }

    fn afficher_indice(&self, mot:&String){
        println!("Le mot a {} lettres",mot.len());
    }
}