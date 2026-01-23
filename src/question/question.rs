use crate::question::definition::Definition;
use crate::question::mot::Mot;
use crate::question::vocabulaire::Vocabulaire;
use std::env::home_dir;
use std::io;
use std::path::PathBuf;


pub struct Question{
    curseur: usize,
    nombre_question: usize,
    liste_définition: Definition,
    liste_mot: Mot,
}

impl Question{
   
    /// Fonction de création de [Question]
    ///
    /// # Paramètre
    /// - Le constructeur ne prend pas de paramètre
    ///
    /// # Retour
    /// - Retourne un résulte de [Question] ou d'une erreur si le fichier n'a pas pu être lu correctement ou si le fichier n'est pas correcte
    ///
    /// # Comportement
    /// - Crée une [Question] avec le fichier par défaut, home/.mot/mot.txt, et transmet le chemin a [Mot] et [Definition]. Si quelque chose se passe mal, fichier mal lu ou fichier incorrecte, il renvoi une erreur sinon une [Question]
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



    /// Fonction de création de [Question]
    ///
    /// # Paramètre
    /// - Le constructeur prend un chemin de fichier
    ///
    /// # Retour
    /// - Retourne un résulte de [Question] ou d'une erreur si le fichier n'a pas pu être lu correctement ou si le fichier n'est pas correcte
    ///
    /// # Comportement
    /// - Crée une [Question] avec le fichier passer en paramètre, et transmet le chemin a [Mot] et [Definition]. Si quelque chose se passe mal, fichier mal lu ou fichier incorrecte, il renvoi une erreur sinon une [Question]
    pub fn nouveau_avec_fichier(fichier: &str) -> Result<Question, io::Error>{
        let chemin = PathBuf::from(fichier);
       
        let def = Definition::nouveau(&chemin)?;
        let mot = Mot::nouveau(&chemin)?;
        
        if def.nombre_définition() != mot.nombre_mot() {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Le fichier n'est pas bon, le nombre de definition et de mot ne sont pas identique"));
        }

        Ok(
            Question { curseur: 0, nombre_question: def.nombre_définition(), liste_définition: def, liste_mot: mot }
            )
    }

    /// Fonction retourne le numéro de la question actuel
    ///
    /// # Paramètre
    /// - Prend une [Question] en lecture
    ///
    /// # Retour
    /// - Retourne un nombre, celui du numéro de la question courante
    ///
    /// # Comportement
    /// - Renvoie simplement le numéro de la question actuel
    pub fn numéro_actuel(&self) -> usize {
        self.curseur
    }


    /// Fonction passant a la question suivante
    ///
    /// # Paramètre
    /// - Prend une [Question] en modification
    ///
    /// # Retour
    /// - Renvoie un résult contenant une [Definition] [Mot] ou une erreur
    ///
    /// # Comportement
    /// - Prend la [Definition] et le [Mot] suivant, vérifie que tout vas bien (on ne sais pas trop pourquoi), et les renvoi si tout vas bien dans un résult ou une erreur sinon
    pub fn et_après(&mut self) -> Result<(String, String), &str>{
        let mot = self.liste_mot.suivant();
        let definition = self.liste_définition.suivant();

        if mot.is_some() && definition.is_some(){
            self.curseur += 1;
            Ok((definition.unwrap(), mot.unwrap()))
        }else{
        Err("Le fichier n'est malheureusement pas correcte")
        }
    }

    /// Fonction qui retourne le numéro de la question actuel
    ///
    /// # Paramètre
    /// - Prend une [Question] en lecture
    ///
    /// # Retour
    /// - Retourne un nombre, celui du numéro de la question courante
    ///
    /// # Comportement
    /// - Renvoie simplement le numéro de la question actuel
    pub fn combien(&self) -> usize{
        self.curseur
    }

    /// Fonction qui renvoie le nombre de question en tout
    ///
    /// # Paramère 
    /// - Prend en paramètre une [Question] en lecture
    ///
    /// # Retour
    /// - Retourne un nombre, celui du nombre de question total
    ///
    /// # Comportement
    /// - Renvoie simplement le nombre total de question
    pub fn combien_en_tout(&self) -> usize{
        self.nombre_question
    }
}




















