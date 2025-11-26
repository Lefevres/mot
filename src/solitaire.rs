use crate::affichage::terminal::AffichageTerminal;
use crate::logique::jeux::Jeux;
use crate::logique::jouer::Jouer;
use crate::logique::preparation::Preparation;
use crate::logique::preparer::Préparer;
use crate::mot::cree_liste;




pub fn solitaire(){
    let logique = Preparation;
    let jeux = Jouer;
    let mut joueur = logique.crée_joueur();
    let liste = cree_liste();
    let nb_manche: usize = logique.demander_nb_manche(liste.len());


    let affichage  = AffichageTerminal;
    
    // Lance la partie
    jeux.jouer(&mut joueur, &affichage, &liste, nb_manche);
}




