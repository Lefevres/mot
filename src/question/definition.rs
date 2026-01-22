use std::fs;
use crate::question::vocabulaire::Vocabulaire;

pub struct Definition{
    liste: Vec<String>,
    curseur: usize,
}

impl Definition{

    /// Fonction de création d'un nouvel itérateur de [Definition]
    ///
    /// # Paramètre 
    /// - Prend en paramètre un chemin de fichier
    ///
    /// # Retour
    /// - Retourne un itérateur de question
    ///
    /// # Comportement
    /// - Crée un itérateur de définition tiré du fichier passer en paramètre
    pub fn nouveau(fichier: &str) -> Definition{
        Definition{
        liste : fs::read_to_string(fichier)
            .expect("Je n'arrive pas a lire la définition")
            .lines()
            .flat_map(|ligne| ligne.split(":"))
            .skip(1)
            .step_by(2)
            .map(|v| v.to_string())
            .collect::<Vec<String>>(),
        curseur : 0
        }
    }

    /// Fonction renvoyant le nombre de définition disponible
    ///
    /// # Paramètre
    /// - Prend en paramètre un chemin de fichier
    ///
    /// # Retour
    /// - Retour un nombre, celui du nombre de définition disponible
    ///
    /// # Comportement
    /// - Renvoie simplement le nombre de définition disponible (en tous)
    pub fn nombre_définition(&self) -> usize{
        self.liste.len()
    }
}

/// Ajout du trait [Iterator] à la structure [Definition]
impl Iterator for Definition{
    /// L'itérateur renvoi des String
    type Item = String;  //type de retour de l'itérateur

    /// Fonction itérant l'itérateur
    ///
    /// # Paramètre
    /// - Prend en paramètre un [Definition] mutable
    ///
    /// # Retour 
    /// - Une option, contenant la définition
    ///
    /// # Comportement
    /// Clone la définition de la liste déterminer grâce au curseur
    /// incrémente le curseur
    /// renvoi la définition
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


/// Definition utilise le trait Vocabulaire
impl Vocabulaire for Definition{

    /// Fonction renvoyant la question suivante, en faisant avancer le curseur
    fn suivant(&mut self) -> Option<String>{
        self.next()
    }

    /// Fonction renvoyant le numéro de la question
    fn quel_numéro(&self) -> usize{
        self.curseur
    }
}















