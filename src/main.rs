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
    let nb_manche = 3;

    let affichage: Box<dyn Affichage> = Box::new(AffichageTerminal);

    // Lance la partie
    jouer(&mut joueur, &*affichage, &liste, nb_manche);
}