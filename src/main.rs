use std::{io};
use colored::Colorize;
use rand::RngCore;
use std::process::Command;


const DEFAULT_DATA: &str = include_str!("../mot.txt");
fn main() {
    let fichier = DEFAULT_DATA;
    let mut autorisation:Vec<&str> = fichier.lines().collect();
    let mut point:u32 = 0;
    let mut mot_avant:String = String::new();
    loop {
        Command::new("clear")
            .status()
            .expect("Échec de la commande clear");

        println!("\n\n{} \n\n +1 points pour une bonne réponse, -1 points pour une mauvaise \n
    {} pour avoir le nombre de lettre\n
    {} pour changer de mot \n
    {} pour arreter \n\n"
     ,"les mots sont sans majuscule mais avec accent".green(),"indice".red(),"passe".red(),"stop".red());


        if mot_avant != ""{
            println!("{} \n", mot_avant);
        }else {
            println!("\n");
        }

        let mot = choisi_mot(autorisation);
        let separer = mot.0.split(":").collect::<Vec<&str>>();
        autorisation = mot.1;
        let mot = separer[0];
        let def = separer[1];
        match essai(mot,def,&point){
            "stop" => {
                println!("fini !");
                break;
            },
            "passe" => {
                if point > 0 {
                    point = point-1;
                }
                mot_avant = format!("Le mot étais {}", mot);

            },

            "trouver" => {
                point = point+1;
                mot_avant = "Bravo tu as trouvé le mot !".to_string();
            },


            _ => {},

        }
        
    }

}



fn essai<'a>(mot: &str,def: &str, point: &u32) -> &'a str{
    let mut saisie = String::new();
    let phrase = format!("Tu as actuellement {} points !", point);
    let longueur:usize = phrase.len();
    loop {
        
        
        println!("Definition : {}.                 {:<longueur$}", def,phrase, longueur = longueur );
        saisie.clear();
        io::stdin()
            .read_line(&mut saisie)
            .expect("Erreur lors de la lecture");

        

        if saisie.trim().to_lowercase() == mot.trim().to_lowercase() {
            return "trouver";
        }


        if saisie.trim().to_lowercase() == "indice"{
         println!("c'est un mot de {} lettres", mot.len()-1);
        }

        if saisie.trim().to_lowercase() == "passe"{
            return "passe";

        }

        if saisie.trim().to_lowercase() == "stop"{
            return "stop";
        }

    }

}

fn choisi_mot(mut autorisation:Vec<&str>) -> (&str,Vec<&str>) {
    let mut aleatoire = rand::rng();
    let mut int_aleatoire = aleatoire.next_u32();

    int_aleatoire = int_aleatoire % (
        if autorisation.len() == 1 {
            autorisation.len()
        } else {
            autorisation.len()-1
        }) as u32;
    let mot = autorisation[int_aleatoire as usize];

    autorisation.retain(|m| m != &mot);

    (mot,autorisation)


}
