use std::io;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use crate::preparation::crée_joueur;

pub fn hote(){

    let mut joueur = crée_joueur();
    //crée joueur

    println!("je suis maintenant un hote joueur");

    let nb_client:usize= 1;

    let nom: Vec<String> = tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(async {
            connextion_au_client(nb_client).await.unwrap()
        });

    println!("Bonjour {:?}",nom[0]);

    println!("Je suis l'hote et tout vas bien !!");



    //si client se connecter et attendre
    //si h proposer la connection et attendre mon bon vouloir
    //si h crée la liste et la partager

}

async fn connextion_au_client(nb_client: usize) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("0.0.0.0:9000").await?;
    let mut noms_joueurs = Vec::new();

    for _ in 0..nb_client {
        println!("En attente d'un client...");
        let (mut socket, adresse) = listener.accept().await?;
        println!("Client connecté : {}", adresse);

        let mut buffer = vec![0u8; 1024];
        let n = socket.read(&mut buffer).await?;
        if n == 0 {
            return Err("Le client a fermé la connexion".into());
        }

        // Convertir en String
        let nom = String::from_utf8_lossy(&buffer[..n]).to_string();
        println!("Nom reçu : {}", nom);

        noms_joueurs.push(nom);
    }

    Ok(noms_joueurs)
}