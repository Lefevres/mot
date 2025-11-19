use std::io;
use crate::affichage::affichage::Affichage;
use crate::affichage::terminal::AffichageTerminal;

static  affichage: AffichageTerminal = AffichageTerminal;
pub fn multi_joueur(){
    let role = choix_role();
    println!("Tu as choisie {}!", role);
}



fn choix_role() -> String{  // les roles sont hebergeure ou client
    let role1 = "hebergeure";
    let role2 = "client";
    affichage.afficher(format!("Role : {} ou {}", role1, role2));
    loop {
        let mut role = String::new();
        io::stdin()
            .read_line(&mut role)
            .expect("Erreur lors de la lecture du role");


        match role.trim().to_lowercase().as_str() {
            r if r == role1 || r == role2 => return role,
            _ => (),
        }
    }

}