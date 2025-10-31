pub struct Joueur  {
    bonneRreponse:usize,
    mauvaiseReponse:usize,
    question:usize,
}
impl Joueur {
    pub fn nouveau() -> Joueur {
    Joueur {bonneRreponse : 0, mauvaiseReponse : 0, question : 0}
    } //crée un nouveau Joueur avec les valeurs de base

    pub fn bonneReponse(&self) -> usize{
        self.bonneRreponse
    } //renvoie le nombre de bonne réponse

    pub fn bonneReponseAj(&mut self) -> usize{
        self.bonneRreponse +=1;
        self.bonneRreponse
    }  //ajoute un au nombre de bonne réponse puis le renvoie

    pub fn mauvaiseReponse(&self) -> usize{
        self.mauvaiseReponse
    }  //renvoie le nombre de mauvaise réponse

    pub fn mauvaiseReponseAj(&mut self) -> usize{
        self.mauvaiseReponse +=1;
        self.mauvaiseReponse
    }  //ajoute un au nombre de mauvaise réponse puis le renvoie

    pub fn question(&self) -> usize{
        self.question
    }  //renvoie le nombre de question répondue

    pub fn questionSuivante(&mut self) -> usize{
        self.question += 2;
        self.question
    }  //augmente question de deux pour accéder a la question suivante

}