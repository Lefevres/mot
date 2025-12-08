use crate::jouer::jouer;
use crate::outils::outils::{demander, se_préparer};
use crate::outils::terminal::{afficher, afficher_str};

mod joueur;
mod multi_joueur;
pub mod jouer;
mod outils;


fn main() {
    loop {
        afficher_str("Mode de jeu : solitaire ou multi_joueur ?");
        let mode = demander(String::new());
        match mode.trim() {
            "solitaire" | "1" => {
                if solitaire(){
                    return
                };
            }
            "multi_joueur" | "2" => {
                multi_joueur();
            }
            _ => {
                afficher_str("N'importe quoi !!");
            }
        }
        if !rejouer(){
            break;
        }

    }
}


fn rejouer() -> bool{
    afficher(String::from("\n\nrejouer ? "));
    loop{
        let réponse = demander(String::new());
        match réponse.as_str() {
            "oui" | "o" =>  return true,
            "non" | "n" => return false,
            _ => (),
        }
    }

}

fn solitaire() -> bool{
    let mut préparation = se_préparer("solitaire".to_string());
    jouer(&mut préparation.0, &préparation.1, préparation.2).2
}


pub fn multi_joueur(){
    let role1 = "hote";
    let role2 = "client";
    let role = choix_role(&role1, &role2);

    match role.as_str() {
        r if r == role1  => multi_joueur::hote::hote(),
        r if r == role2 => multi_joueur::client::client(),
        _ => afficher(format!("Rôle inconnu tu as rentrer : {}", role)),
    }
}


fn choix_role(role1 : &str, role2 : &str) -> String{  // les roles sont hote ou client
    afficher(format!("Role : {} ou {}", role1, role2));
    loop {
        let role = demander(String::new());

        match role.as_str() {
            r if r == role1 || r == "1" || r == "h" => {
                return role1.to_string();
            }
            r if r == role2 || r == "2" || r == "c"=> {
                return role2.to_string();
            }
            _ => {
                afficher("Choix invalide, réessayez.".to_string());
            }
        }
    }
}