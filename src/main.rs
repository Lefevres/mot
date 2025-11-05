use std::io;
use std::io::stdout;
use crate::affichage::affichage::Affichage;
use crate::affichage::terminal::AffichageTerminal;
use crate::jouer::jouer;
use crate::joueur::Joueur;
use crate::mot::cree_liste;

mod joueur;
mod jouer;
mod mot;
mod affichage;
//const DEFAULT_DATA: &str = include_str!("../mot.txt"); //pas besoin je pense

fn main() {  //il faudra faire attention a ce que le numéro de la question ne dépasse pas le nombre de question
    //placer correctement les fichier, installateur     ?
    //il faut absolument cmake
    let mut joueur = Joueur::nouveau();  //crée le nouveau joueur
    let liste = cree_liste();  //crée la liste des questions
    let nb_manche: usize = demander_nb_manche();


    let affichage: Box<dyn Affichage> = Box::new(AffichageTerminal);

    // Lance la partie
    jouer(&mut joueur, &*affichage, &liste, nb_manche);
}

fn demander_nb_manche() -> usize {
    loop {
        crossterm::execute!(stdout(), crossterm::terminal::Clear(crossterm::terminal::ClearType::All)).unwrap();
        let mut entree = String::new(); // Crée une nouvelle chaîne à chaque itération
        println!("Combien de manche ? \n");
        println!("Nombre max de manches : {} \n\n\n", usize::MAX.to_string());

        io::stdin()
            .read_line(&mut entree)
            .expect("Erreur lors de la lecture du nombre de manches");

        match entree.trim().parse::<usize>() {
            Ok(num) => return num, // ✅ Retourne le nombre valide et quitte la boucle
            Err(_) => println!("Entrée invalide, veuillez entrer un nombre entier positif."),
        }
    }
}