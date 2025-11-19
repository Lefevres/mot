use crate::affichage::affichage::Affichage;
use crate::multi_joueur::multi_joueur::multi_joueur;

mod joueur;
mod jouer;
mod mot;
mod affichage;
mod solitaire;
mod multi_joueur;
mod preparation;

fn main() {
    multi_joueur();
}
