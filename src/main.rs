use std::io;
use std::io::stdout;
use crate::affichage::affichage::Affichage;
use crate::solitaire::solitaire;
use crate::affichage::terminal::AffichageTerminal;
use crate::jouer::jouer;
use crate::joueur::Joueur;
use crate::mot::cree_liste;

mod joueur;
mod jouer;
mod mot;
mod affichage;
mod solitaire;

fn main() {
    solitaire();   
}
