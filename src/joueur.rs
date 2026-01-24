

/// Structure servant a représenter un joueur
/// Un joueur est un nom, un nombre de bonne réponse et un nombre de mauvaise réponse
pub struct Joueur<'a>{
    bonne_réponse: usize,
    mauvaise_réponse: usize,
    pub nom:&'a str,
}

impl Joueur<'_>{
    
    /// Fonction de création d'un nouveau [Joueur]
    ///
    /// # Paramètre
    /// - Demande un nom pour le joueur
    ///
    /// # Retour
    /// - Un nouveau joueur portant le nom passer en paramètre
    ///
    /// # Comportement
    /// - Crée un joueur
    pub fn nouveau(mon_nom: &str) -> Joueur<'_>{
        Joueur{bonne_réponse: 0,mauvaise_réponse: 0,nom: mon_nom}
    }

    /// Fonction permettant d'ajouter une bonne réponse au total du joueur
    ///
    /// # Paramètre
    /// - Prend en paramètre un joueur mutable
    ///
    /// # Retour 
    /// - Ne retourne rien
    ///
    /// # Comportement
    /// - Ajoute un au total de bonne réponse du joueur passer en paramètre
    pub fn ajout_bonne_réponse(&mut self){
        self.bonne_réponse += 1;
    }

    /// Fonction permettant d'ajouter une mauvaise réponse au total du joueur
    ///
    /// # Paramètre
    /// - Prend en paramètre un joueur mutable
    ///
    /// # Retour
    /// - Ne retourne rien
    ///
    /// # Comportement
    /// - Ajoute un au total de mauvaise réponse du joueur passer en paramètre
    pub fn ajout_mauvaise_réponse(&mut self){
        self.mauvaise_réponse += 1;
    }

    /// Fonction renvoyant le nombre de bonne réponse du joueur
    ///
    /// # Paramètre
    /// - Prend en paramètre un joueur en lecture
    ///
    /// # Retour
    /// - Retourne le nombre de bonne réponse du joueur
    ///
    /// # Comportement
    /// - Renvoi le nombre de bonne réponse du joueur
    pub fn combien_de_bonne_réponse(&self) -> usize{
        self.bonne_réponse
    }

    /// Fonction renvoyant le nombre de mauvaise réponse du joueur
    ///
    /// # Paramètre
    /// - Prend en paramètre un joueur en lecture
    ///
    /// # Retour
    /// - Retourne le nombre de mauvaise réponse du joueur
    ///
    /// # Comportement
    /// - Renvoi le nombre de mauvaise réponse du joueur
    pub fn combien_de_mauvaise_réponse(&self) -> usize{
        self.mauvaise_réponse
    }

    /// Fonction renvoyant le nom du joueur
    ///
    /// # Paramètre
    /// - Prend en paramètre un joueur en lecture
    ///
    /// # Retour
    /// - Retourne le nom du joueur
    ///
    /// # Comportement
    /// - Renvoi le nom du joueur
    pub fn voici_mon_nom(&self) -> &str{
        self.nom
    }
}
