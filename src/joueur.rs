use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Joueur  {
    bonne_reponse: usize,
    mauvaise_reponse: usize,
    nb_question: usize,   //nb_question actuelle
    mut pub(crate) nom: String,
}


impl Joueur {
    pub fn nouveau() -> Joueur {
    Joueur {bonne_reponse : 0, mauvaise_reponse : 0, nb_question : 0, nom: "".to_string() }
    } //crée un nouveau Joueur avec les valeurs de base

    pub fn bonne_reponse(&self) -> usize{
        self.bonne_reponse
    }  //renvoie le nombre de bonne réponse

    pub fn bonne_reponse_aj(&mut self) -> usize{
        self.bonne_reponse +=1;
        self.bonne_reponse
    }  //ajoute un au nombre de bonne réponse puis le renvoie

    pub fn mauvaise_reponse(&self) -> usize{
        self.mauvaise_reponse
    }  //renvoie le nombre de mauvaise réponse

    pub fn mauvaise_reponse_aj(&mut self) -> usize{
        self.mauvaise_reponse +=1;
        self.mauvaise_reponse
    }  //ajoute un au nombre de mauvaise réponse puis le renvoie

    pub fn nb_question(&self) -> usize{
        self.nb_question
    }  //renvoie le nombre de nb_question répondue

    pub fn question_suivante(&mut self) -> usize{
        self.nb_question += 2;
        self.nb_question
    }  //augmente nb_question de deux pour accéder a la nb_question suivante

    pub fn fin(&self, manche: &usize) -> bool {
        self.nb_question / 2 == *manche
    } //renvoie true si on a fais manche tour


    pub fn défini_nom(&mut self, nom: String){
        self.nom = nom;
    }


    pub fn nom(&self) -> String{
        self.nom.clone()
    }
}