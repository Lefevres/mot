use std::io;
use crate::affichage::affichage::Affichage;
use crate::affichage::terminal::AffichageTerminal;
use crate::multi_joueur::client::client;
use crate::multi_joueur::hote::hote;

static  affichage: AffichageTerminal = AffichageTerminal;
pub fn multi_joueur(){
    let role1 = "hote";
    let role2 = "client";
    let role = choix_role(&role1, &role2);
    println!("Tu as choisie {}!", role);

    match role.as_str() {
        r if r == role1 => hote(),
        r if r == role2 => client(),
        _ => println!("Rôle inconnu tu as rentrer : {}", role),
    }


}



fn choix_role(role1 : &str, role2 : &str) -> String{  // les roles sont hote ou client

    affichage.afficher(format!("Role : {} ou {}", role1, role2));
    loop {
        let mut role = String::new();
        io::stdin()
            .read_line(&mut role)
            .expect("Erreur lors de la lecture du role");

        role = role.trim().to_string();
        match &role {
            r if r == role1 || r == role2 => return role,
            _ => (),
        }
    }

}