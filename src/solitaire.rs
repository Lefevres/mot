use crate::affichage::terminal::AffichageTerminal;
use crate::jouer::jouer;
use crate::mot::cree_liste;
use crate::preparation::{crée_joueur, demander_nb_manche};

pub fn solitaire(){
    let mut joueur = crée_joueur(false);
    let liste = cree_liste();
    let nb_manche: usize = demander_nb_manche(liste.len());


    let affichage  = AffichageTerminal;

    // Lance la partie
    jouer(&mut joueur, &affichage, &liste, nb_manche);
}




