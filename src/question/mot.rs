use std::fs;
use crate::question::vocabulaire::Vocabulaire;

pub struct Mot{
    liste: Vec<String>,
    curseur: usize,
}

impl Mot{

    /// Fonction de création d'un nouvel itérateur de [Mot]
    ///
    /// # Paramètre 
    /// - Prend en paramètre un chemin de fichier
    ///
    /// # Retour
    /// - Retourne un itérateur de question
    ///
    /// # Comportement
    /// - Crée un itérateur de mot tiré du fichier passer en paramètre
    pub fn nouveau(fichier: &str) -> Mot{
        Mot{
        liste : fs::read_to_string(fichier)
            .expect("Je n'arrive pas a lire le mot")
            .lines()
            .flat_map(|ligne| ligne.split(":")) 
            .step_by(2)
            .map(|v| v.trim().to_string())
            .collect::<Vec<String>>(),
        curseur : 0
        }
    }

    /// Fonction renvoyant le nombre de mot disponible
    ///
    /// # Paramètre
    /// - Prend en paramètre un chemin de fichier
    ///
    /// # Retour
    /// - Retour un nombre, celui du nombre de mot disponible
    ///
    /// # Comportement
    /// - Renvoie simplement le nombre de mot disponible (en tous)
    pub fn nombre_mot(&self) -> usize{
        self.liste.len()
    }
}

/// Ajout du trait [Iterator] à la structure [Mot]
impl Iterator for Mot{
    /// L'itérateur renvoi des String
    type Item = String;  //type de retour de l'itérateur

    /// Fonction itérant l'itérateur
    ///
    /// # Paramètre
    /// - Prend en paramètre un [Mot] mutable
    ///
    /// # Retour 
    /// - Une option, contenant la mot
    ///
    /// # Comportement
    /// Clone la mot de la liste déterminer grâce au curseur
    /// incrémente le curseur
    /// renvoi la mot
    fn next(&mut self)-> Option<Self::Item>{
        if self.curseur < self.liste.len(){
            let def = self.liste[self.curseur].clone();
            self.curseur += 1;
            Some(def)
        }
        else{
            None
        }
    }
}


/// Mot utilise le trait Vocabulaire
impl Vocabulaire for Mot{

    /// Fonction renvoyant la question suivante, en faisant avancer le curseur
    fn suivant(&mut self) -> Option<String>{
        self.next()
    }

    /// Fonction renvoyant le numéro de la question
    fn quel_numéro(&self) -> usize{
        self.curseur
    }
}















