use std::io::Read;
use std::io;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener,TcpStream};
use crate::affichage::terminal::AffichageTerminal;
use crate::jouer::jouer;
use crate::mot::cree_liste;
use crate::preparation::{crée_joueur, demander_nb_manche};

pub fn hote(){

    let mut joueur = crée_joueur(true);
    let liste = cree_liste();
    let nb_client:usize= demander_nb_joueur();

    let nom: Vec<String> = tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(async {
            connextion_au_client(nb_client).await.unwrap()
        });


    for joueur in nom {
        println!("Bonjour {}",joueur);
    }

    let nb_manche: usize = demander_nb_manche(liste.len());

    let affichage  = AffichageTerminal;

    // Lance la partie
    jouer(&mut joueur, &affichage, &liste, nb_manche);

}


fn demander_nb_joueur() -> usize {
    println!("Pour combien de joueur ? (hormis toi)");
    loop {
        let mut nb_joueur:String = String::new();
        io::stdin().read_line(&mut nb_joueur).unwrap();
        nb_joueur = nb_joueur.trim().to_string();
        if nb_joueur.parse::<i32>().is_ok(){
            return nb_joueur.parse::<i32>().unwrap() as usize;
        }
    }

}


async fn lis_buffer(mut socket:TcpStream) -> Result<String,Box<dyn std::error::Error>>{
    let mut buffer = vec![0u8; 1024];
    let n = socket.read(&mut buffer).await?;
    if n == 0 {
        return Err("Le client a fermé la connexion".into());
    }
    let littérale = String::from_utf8_lossy(&buffer[..n]).to_string();
    Ok(littérale)
}



async fn connextion_au_client(nb_client: usize) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("0.0.0.0:9000").await?;
    let mut noms_joueurs = Vec::new();

    for _ in 0..nb_client {
        println!("En attente d'un client...");
        let (mut socket, adresse) = listener.accept().await?;
        println!("Client connecté : {}", adresse);

        let nom = lis_buffer(socket).await?;
        /*let mut buffer = vec![0u8; 1024];
        let n = socket.read(&mut buffer).await?;
        if n == 0 {
            return Err("Le client a fermé la connexion".into());
        }*/

        // Convertir en String
        //let nom = String::from_utf8_lossy(&buffer[..n]).to_string();

        noms_joueurs.push(nom);

    }

    Ok(noms_joueurs)
}