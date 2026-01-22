use std::fs;


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
    fn nouveau(fichier: &str){
        Definition{
        liste : fs::read_to_string(fichier)
            .expect("Je n'arrive pas a lire la définition")
            .lines()
            .split(":")
            .skip(1)
            .step_by(2)
            .map(|v| v.to_string())
            .collect::<Vec<String>>(),
        curseur : 0
        }
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
