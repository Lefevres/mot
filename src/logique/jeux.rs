use crate::affichage::affichage::Affichage;
use crate::joueur::Joueur;

pub trait Jeux{

    fn jouer(&self,joueur: &mut Joueur, affichage: &dyn Affichage, liste: &Vec<String>, nb_manche : usize) -> (usize,usize);
    fn manche(&self,joueur : &mut Joueur,affichage: &dyn Affichage, liste : &Vec<String>) -> bool;

    fn attendre_réponse(&self) -> String;

    fn réagir(&self,joueur: &mut Joueur,affichage : &dyn Affichage, reponse: &String, mot: &String) -> String;
}