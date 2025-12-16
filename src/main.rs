use crate::affichage::affichage::Affichage;
use crate::affichage::terminal::AffichageTerminal;

mod detail;
mod affichage;
mod jeux;
mod multi_joueur;

mod outils;

mod joueur;
mod mode;



fn main(){
    let affichage: Box<dyn Affichage> = Box::new(AffichageTerminal);
    detail::gère(&affichage);
}