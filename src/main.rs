use crate::affichage::affichage::Affichage;
use crate::affichage::terminal::AffichageTerminal;
use crate::jouer::jouer;
use crate::outils::outils::{demander, se_préparer};

mod joueur;
mod mot;
mod affichage;
mod multi_joueur;
pub mod jouer;
pub mod preparation;
mod outils;

static  AFFICHAGE: AffichageTerminal = AffichageTerminal;


fn main() {
    println!("Mode de jeu ? solitaire(1) ou multi_joueur(2) ?");
    loop {
        let mode = demander(String::new());
        match mode.trim() {
            "solitaire" | "1" => {
                solitaire();
                println!("Lancement d'une nouvelle partie");
                println!("Mode de jeu ? solitaire ou multi_joueur ?");
            }
            "multi_joueur" | "2" => {
                multi_joueur();
                println!("Lancement d'une nouvelle partie");
                println!("Mode de jeu ? solitaire ou multi_joueur ?");
            }
            _ => {
                println!("N'importe quoi !!");
            }
        }

    }
}


fn solitaire() {
    let mut préparation = se_préparer(false);
    jouer(&mut préparation.0, &AFFICHAGE, &préparation.1, préparation.2);
}


pub fn multi_joueur(){
    let role1 = "hote";
    let role2 = "client";
    let role = choix_role(&role1, &role2);
    println!("Tu as choisie {}!", role);

    match role.as_str() {
        r if r == role1 => multi_joueur::hote::hote(),
        r if r == role2 => multi_joueur::client::client(),
        _ => println!("Rôle inconnu tu as rentrer : {}", role),
    }
}


fn choix_role(role1 : &str, role2 : &str) -> String{  // les roles sont hote ou client
    AFFICHAGE.afficher(format!("Role : {} ou {}", role1, role2));
    loop {
        let mut role = demander(String::new());

        match &role {
            r if r == role1 || r == role2 => return role,
            _ => (),
        }
    }

}