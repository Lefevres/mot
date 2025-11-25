use crate::affichage::affichage::Affichage;
use crate::multi_joueur::multi_joueur::multi_joueur;
use crate::solitaire::solitaire;

mod joueur;
mod jouer;
mod mot;
mod affichage;
mod solitaire;
mod multi_joueur;
mod preparation;

fn main() {
    solitaire();
    //multi_joueur();
}
