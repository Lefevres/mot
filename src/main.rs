use crate::affichage::affichage::Affichage;
use crate::affichage::graphique::AffichageGraphique;


mod detail;
mod affichage;
mod jeux;
mod multi_joueur;

mod outils;

mod joueur;
mod mode;



fn main(){
    let affichage: Box<dyn Affichage> = Box::new(AffichageGraphique { score: 3 }); //Box::new(AffichageTerminal);
    affichage.afficher("bonjour".to_string());
    detail::gère(&affichage);
}