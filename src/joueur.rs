


pub struct Joueur<'a>{
    bonne_réponse: usize,
    mauvaise_réponse: usize,
    pub nom:&'a str,
}

impl Joueur<'_>{
    
    pub fn nouveau(mon_nom: &str) -> Joueur<'_>{
        Joueur{bonne_réponse: 0,mauvaise_réponse: 0,nom: mon_nom}
    }


    pub fn ajout_bonne_réponse(&mut self){
        self.bonne_réponse += 1;
    }

    pub fn ajout_mauvaise_réponse(&mut self){
        self.mauvaise_réponse += 1;
    }

    pub fn combien_de_bonne_réponse(&self) -> usize{
        self.bonne_réponse
    }

    pub fn combien_de_mauvaise_réponse(&self) -> usize{
        self.mauvaise_réponse
    }

    pub fn voici_mon_nom(&self) -> &str{
        self.nom
    }


}
