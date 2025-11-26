use crate::joueur::Joueur;

pub trait préparer{
    fn crée_joueur(&self) -> Joueur;

    fn demander_nb_manche(&self,taille_liste:usize) -> usize;
}