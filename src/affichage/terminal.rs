use std::io;
use std::io::stdout;
use colored::Colorize;
use crate::affichage::affichage::Affichage;

pub struct AffichageTerminal;

impl Affichage for AffichageTerminal {
    fn afficher_en_tete(&self){
        crossterm::execute!(stdout(), crossterm::terminal::Clear(crossterm::terminal::ClearType::All)).unwrap();
        let en_tete = "\n\nLes mots sont sans majuscule mais avec accent".green();
        let instruction = "\n +1 points pour une bonne réponse, -1 points pour une mauvaise réponse\n".red();
        let indices = "{indice} pour avoir le nombre de lettres\n{passe} pour changer de mot\n{stop} pour arrêter".red();
        println!("{en_tete}\n{instruction}\n{indices}");
    }

    fn afficher_question<'a>(&self, nb_question : usize, liste : &'a Vec<String>) -> &'a String{  //renvoie le mot attendu
        let question = format!("{}", liste[nb_question+1]).bright_yellow();
        println!("\n\n{}\n\n", question ); //nb_question est la réponse, nb_question +1 est la question
        &liste[nb_question]
    }

    fn afficher_reponse_precedante(&self, mot : &String){
        println!("La réponse étais {}\n",mot.green());
        println!("Entrer pour continuer : \n");
        let mut saisie = String::new();
        io::stdin()
            .read_line(&mut saisie)
            .expect("Erreur lors de la lecture");


    }

    fn afficher_indice(&self, mot:&String){
        println!("Le mot a {} lettres \n",mot.len());
    }

    fn afficher_bonne_reponse(&self) {
        println!("Félicitation ce fût une bonne réponse 👍\n");
    }

    fn afficher_mauvaise_reponse(&self) {
        println!("Ça n'est malheureusement pas ça \n");
    }
}