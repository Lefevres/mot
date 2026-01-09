use std::io::stdout;
use colored::Colorize;
use crate::joueur::Joueur;
use crate::outils::outils::demander;


pub fn afficher_en_tete(){
    crossterm::execute!(stdout(), crossterm::terminal::Clear(crossterm::terminal::ClearType::All)).unwrap();
    let en_tete = "\n\nLes mots sont sans majuscule (sauf pour les noms propre) mais avec accent, les locutions latine ne prenne pas de tiret".green();
    let indices = "\n\
    {indice/i} pour avoir le nombre de lettres\n\
    {passe/p} pour changer de mot\n\
    {stop/s} pour arrêter".red();
    println!("{en_tete}\n{indices}");
}


pub fn afficher_nb_lettre(nb_lettre: usize){
    println!("\nC’est un mot de {} lettres", nb_lettre);
}

pub fn afficher_question(question : &str){  //renvoie le mot attendu
        let question = question.to_string().bright_yellow();
        println!("\n\n{}\n\n\n", question ); //nb_question.0 est la réponse, nb_question.1 est la question
    }


pub fn afficher_indice(mot: &str) {
    let len = mot.chars().count();
    let revelation = len / 3;

    let prefix: String = mot.chars().take(revelation).collect();

    let mut indice = prefix;
    for _ in 0..(len - revelation) {
        indice.push_str("_ ");
    }

    println!("\n\n{}", indice);
}

pub fn afficher_reponse_precedante(mot : &str){
        println!("\nLa réponse était {}\n",mot.green());
        attendre_validation();
    }

pub fn afficher_bonne_reponse() {
        println!("\n\nFélicitation c'est une bonne réponse 👍\n");
        attendre_validation();
    }

fn attendre_validation(){
    demander(Some("Entrer pour continuer : \n"));
}

pub fn afficher_mauvaise_reponse() {
        println!("\nÇa n'est malheureusement pas ça \n");
    }

pub fn afficher_score(joueur: &Joueur, nb_manche: usize) {

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

pub fn afficher_score_fin(joueur: Joueur) {

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


pub fn afficher(texte: String) {
        println!("{}", texte);
    }


pub fn afficher_str(texte: &str) {
    println!("{}", texte);
}

