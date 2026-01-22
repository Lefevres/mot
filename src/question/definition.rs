


pub struct Definition{
    liste: Vec<String>,
    curseur: usize,
}

impl Definition{
    fn nouveau(fichier: &str){
        Definition{
        liste = fs::read_to_string(fichier)
            .expect("Je n'arrive pas a lire la définition")
            .lines()
            .skip(1)
            .step_by(2)
            .map(|v| v.to_string())
            .collect::Vec<String>(),
        curseur = 0;
        }
    }
}

impl Iterator for Definition{
    type Item = String;  //type de retour de l'itérateur

    fn next(&mut self)-> Option<Self::Item>{
        if self.curseur < self.liste.len(){
            let def = self.liste[self.curseur].clone();
            curseur += 1;
            Some(def)
        }
        else{
            None
        }
    }
}
