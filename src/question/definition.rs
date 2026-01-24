use std::fs;
use std::io;
use crate::question::vocabulaire::Vocabulaire;
use std::path::PathBuf;

/// Structure représentant l'ensemble des définitions
/// Elle lit dans un fichier toute les définitions 
/// et en fait un itérateur.
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
    pub fn nouveau(fichier_chemin: &PathBuf) -> Result<Definition, io::Error>{
        let fichier = fs::read_to_string(fichier_chemin)?;
        Ok(Definition{
        liste: fichier
            .lines()
            .filter(|l| !l.trim().is_empty())
            .flat_map(|ligne| ligne.split(":"))
            .skip(1)
            .step_by(2)
            .map(|v| v.trim().to_string())
            .collect::<Vec<String>>(),
        curseur : 0
        })
    }

    /// Fonction renvoyant le nombre de définition disponible
    ///
    /// # Paramètre
    /// - Prend en paramètre une [Definition] en lecture 
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















