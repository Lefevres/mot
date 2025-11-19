use crate::affichage::affichage::Affichage;
use crate::multi_joueur::multi_joueur;
use crate::solitaire::solitaire;


mod joueur;
mod jouer;
mod mot;
mod affichage;
mod solitaire;
mod multi_joueur;

fn main() {
    multi_joueur();
}
