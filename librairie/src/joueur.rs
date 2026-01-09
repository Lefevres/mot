use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Joueur  {
    bonne_reponse: usize,
    mauvaise_reponse: usize,
    question: usize,   //question actuelle
    nom: String,
}


impl Joueur {
    /// Fonction permettant de créer un joueur de base.
    ///
    /// # Paramètre
    /// - Ne prend pas de paramètre.
    ///
    /// # Retour
    /// - Une structure de Joueur.
    ///
    /// # Comportement
    /// Crée un Joueur avec des paramètres de base.
    ///
    pub fn nouveau() -> Joueur {
    Joueur {bonne_reponse : 0, mauvaise_reponse : 0, question : 0, nom: "".to_string() }
    }


    /// Fonction renvoyant le nombre de bonnes réponses.
    ///
    /// # Paramètre
    /// - Un &self.
    ///
    /// # Retour
    /// - Le nombre de bonnes réponses sous forme d’usize.
    ///
    pub fn bonne_reponse(&self) -> usize{
        self.bonne_reponse
    }


    /// Fonction ajoutant une bonne réponse.
    ///
    /// # Paramètre
    /// - Un &mut self.
    ///
    /// # Retour
    /// - Le nombre de bonnes réponses sous forme d’usize.
    ///
    /// # Comportement
    /// Ajoute un au nombre de bonnes réponses,
    /// renvoie le nombre de bonnes réponses.
    ///
    pub fn bonne_reponse_aj(&mut self) -> usize{
        self.bonne_reponse +=1;
        self.bonne_reponse
    }


    /// Fonction renvoyant le nombre de mauvaises réponse.
    ///
    /// # Paramètre
    /// - Un &self.
    ///
    /// # Retour
    /// - Le nombre de mauvaises réponse sous forme d’usize.
    ///
    pub fn mauvaise_reponse(&self) -> usize{
        self.mauvaise_reponse
    }  //renvoie le nombre de mauvaise réponse


    /// Fonction ajoutant une mauvaise réponse.
    ///
    /// # Paramètre
    /// - Un &mut self.
    ///
    /// # Retour
    /// - Le nombre de mauvaises réponse sous forme d’usize.
    ///
    /// # Comportement
    /// Ajoute un au nombre de mauvaises réponse,
    /// renvoie le nombre de mauvaises réponse.
    ///
    pub fn mauvaise_reponse_aj(&mut self) -> usize{
        self.mauvaise_reponse +=1;
        self.mauvaise_reponse
    }


    /// Fonction renvoyant le nombre de questions répondu.
    ///
    /// # Paramètre
    /// - Un &self.
    ///
    /// # Retour
    /// - Le nombre de questions répondu sous forme d’usize.
    ///
    pub fn question(&self) -> usize{
        self.question
    }

    pub fn question_suivante(&mut self) -> usize{
        self.question += 2;
        self.question
    }  //augmente question de deux pour accéder a la question suivante



    pub fn fin(&self, manche:usize) -> bool {
        self.question / 2 == manche
    } //renvoie true si on a fais manche tour


    /// Fonction servant à définir le nom du joueur.
    ///
    /// # Paramètre
    /// - Un &mut self,
    /// - un nom sous forme de string.
    ///
    /// # Retour
    /// - Ne retourne rien.
    ///
    /// # Comportement
    ///Remplace le nom du joueur par celui passer en paramètre.
    ///
    pub fn défini_nom(&mut self, nom: String){
        self.nom = nom;
    }


    /// Fonction renvoyant le nom du joueur.
    ///
    /// # Paramètre
    /// - Un &self.
    ///
    /// # Retour
    /// - Le nom sous forme d’usize.
    ///
    /// # Comportement
    /// Clone le nom et le renvoie.
    ///
    pub fn nom(&self) -> String{
        self.nom.clone()
    }


    /// Fonction remettant le nombre de questions à zéro.
    ///
    /// # Paramètre
    /// - Un &mut self.
    ///
    /// # Retour
    /// - Ne retourne rien.
    ///
    /// # Comportement
    /// Définie le nombre de questions répondu à zéro.
    ///
    pub fn remet_les_questions_a_zero(&mut self){
        self.question = 0;
    }
}