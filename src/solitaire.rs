use crate::affichage::terminal::AffichageTerminal;
use crate::mot::cree_liste;
use crate::preparation::{crée_joueur,demander_nb_manche};
use crate::jouer::jouer;



pub fn solitaire(){
    let mut joueur = crée_joueur();
    let liste = cree_liste();
    let nb_manche: usize = demander_nb_manche(liste.len());


    let affichage  = AffichageTerminal;
    
    // Lance la partie
    jouer(&mut joueur, &affichage, &liste, nb_manche);
}




