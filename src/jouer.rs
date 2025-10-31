use std::io;
use colored::Colorize;
use crate::joueur::Joueur;


pub fn jouer(joueur: &mut Joueur, liste: &Vec<String>, nb_manche : usize) {
    while joueur.question() < nb_manche {
        manche(joueur, liste);
    }
}
fn manche(joueur : &mut Joueur, liste : &Vec<String>){
    let mut essai = false;
    afficherLEnTete();
    let mot = afficherQuestion(joueur.question(),&liste);
    while !essai {
        essai = attendreReponse(&mot);
        reagir(joueur,essai);
    }

}

fn afficherLEnTete(){
    println!("\n\n{} \n\n +1 points pour une bonne réponse, -1 points pour une mauvaise \n
    {} pour avoir le nombre de lettre\n
    {} pour changer de mot \n
    {} pour arreter \n\n"
             ,"les mots sont sans majuscule mais avec accent".green(),"indice".red(),"passe".red(),"stop".red());

}
fn afficherQuestion(nbQuestion : usize, liste : &Vec<String>) -> &String{  //renvoie le mot attendu
    println!("{}", liste[nbQuestion+1]); //nbQuestion est la réponse, nbQuestion +1 est la question
    &liste[nbQuestion]
}

fn  attendreReponse(attendue :&String) -> bool{
    let mut saisie = String :: new();
    io::stdin()
        .read_line(&mut saisie)
        .expect("Erreur lors de la lecture");

    saisie = saisie.trim().to_lowercase();

    attendue.as_str() == saisie
}

fn reagir(joueur: &mut Joueur,reponse :bool){
    match reponse{
        true => {
            joueur.bonneReponseAj();
            joueur.questionSuivante();
            println!("bonne réponse");
        }
        false => {
            joueur.mauvaiseReponseAj();
            println!("mauvaise réponse");
        }
    }
}