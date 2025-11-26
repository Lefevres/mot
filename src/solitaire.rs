use crate::affichage::terminal::AffichageTerminal;
use crate::logique::jeux::jeux;
use crate::logique::jouer::jouer;
use crate::logique::preparation::preparation;
use crate::logique::preparer::préparer;
use crate::mot::cree_liste;




pub fn solitaire(){
    let logique = preparation;
    let jeux = jouer;
    let mut joueur = logique.crée_joueur();
    let liste = cree_liste();
    let nb_manche: usize = logique.demander_nb_manche(liste.len());


    let affichage  = AffichageTerminal;
    
    // Lance la partie
    jeux.jouer(&mut joueur, &affichage, &liste, nb_manche);
}




