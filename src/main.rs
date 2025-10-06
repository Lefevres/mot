use std::{fs, io};
use rand::RngCore;

fn main() {
    let fichier = fs::read_to_string("mot.txt").unwrap();
    let mut autorisation: Vec<&str> = vec!();
    let prep_autorisation:Vec<&str> = fichier.lines().collect();
    for element in  prep_autorisation {
        autorisation.push(element)
    }

    loop {
        let mot = choisi_mot(autorisation);
        let separer = mot.0.split(":").collect::<Vec<&str>>();
        autorisation = mot.1;
        let mot = separer[0];
        let def = separer[1];
        if essai(mot,def) == true{
            break;
        }
    }



}



fn essai(mot: &str,def: &str) -> bool{
    let mut saisie = String::new();
    let mut sortie ="";
    loop {
        println!("Definition : {}", def);
        saisie.clear();
        io::stdin()
            .read_line(&mut saisie)
            .expect("Erreur lors de la lecture");
        if saisie.trim().to_lowercase()   == mot.trim().to_lowercase() {
            println!("bravo tu as trouver !");
            sortie = "trouver";
            break;
        }


        if saisie.trim().to_lowercase() == "indice"{
         println!("c'est un mot de {} lettres", mot.len());
        }

        if saisie.trim().to_lowercase() == "stop"{
            sortie = "abandon";
            break;
        }
        //println!("{}", saisie.trim().to_lowercase());

    }
    true

}

fn choisi_mot(mut autorisation:Vec<&str>) -> (&str,Vec<&str>) {
    let mut aleatoire = rand::rng();
    let mut int_aleatoire = aleatoire.next_u32();


    //let lignes: Vec<&str> = fichier.lines().collect();
    // l//et nb_lignes = lignes.len();


    int_aleatoire = int_aleatoire % (autorisation.len()-1) as u32;
    let mot = autorisation[int_aleatoire as usize].clone();

    autorisation.retain(|m| m != &mot);

    //interdiction.push(int_aleatoire as usize);
    (mot,autorisation)


}
