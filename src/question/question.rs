use crate::question::definition::Definition;
use crate::question::mot::Mot;
use crate::question::vocabulaire::Vocabulaire;
use std::env::home_dir;
use std::io;


pub struct Question{
    curseur: usize,
    nombre_question: usize,
    liste_définition: Definition,
    liste_mot: Mot,
}

impl Question{
   

    pub fn nouveau() -> Result<Question, io::Error>{
        let chemin =  home_dir()
            .ok_or(io::Error::new(io::ErrorKind::NotFound, "Je ne trouve pas le dossier home"))?
            .join(".mot")
            .join("mot.txt");
       
        let def = Definition::nouveau(&chemin)?;
        let mot = Mot::nouveau(&chemin)?;
        
        if def.nombre_définition() != mot.nombre_mot() {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Le fichier n'est pas bon, le nombre de definition et de mot ne sont pas identique"));
        }

        Ok(
            Question { curseur: 0, nombre_question: def.nombre_définition(), liste_définition: def, liste_mot: mot }
            )


    }

    pub fn numéro_actuelle(&self) -> usize {
        self.curseur
    }

    pub fn et_après(&mut self) -> Result<(String, String), &str>{
        let mot = self.liste_mot.suivant();
        let definition = self.liste_définition.suivant();

        if mot.is_some() && definition.is_some(){
            Ok((mot.unwrap(), definition.unwrap()))
        }else{
        Err("Le fichier n'est malheureusement pas correcte")
        }
    }


    pub fn combien(&self) -> usize{
        self.curseur
    }

    pub fn combien_en_tout(&self) -> usize{
        self.nombre_question
    }
}




















