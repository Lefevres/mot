pub struct Joueur  {
    bonne_reponse:usize,
    mauvaise_reponse:usize,
    question:usize,
    est_multi:bool,
}


impl Joueur {
    pub fn nouveau(est_multi:bool) -> Joueur {
    Joueur {bonne_reponse : 0, mauvaise_reponse : 0, question : 0, est_multi }
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

    pub fn question(&self) -> usize{
        self.question
    }  //renvoie le nombre de question répondue

    pub fn question_suivante(&mut self) -> usize{
        self.question += 2;
        self.question
    }  //augmente question de deux pour accéder a la question suivante

    pub fn fin(&self, manche:usize) -> bool {
        self.question / 2 == manche
    } //renvoie true si on a fais manche tour
}