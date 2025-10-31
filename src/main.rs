mod jouer;
mod mot;
mod joueur;

use colored::Colorize;
use rand::RngCore;
use crate::jouer::{jouer};
use crate::joueur::Joueur;
use crate::mot::cree_liste;

const DEFAULT_DATA: &str = include_str!("../mot.txt");

fn main() {  //il faudra faire attention a ce que le numéro de la question ne dépasse pas le nombre de question
    let mut joueur = Joueur::nouveau();  //crée le nouveau joueur
    let liste = cree_liste();  //crée la liste des questions
    jouer(&mut joueur, &liste, 3); //lance une manche (un mot)
}