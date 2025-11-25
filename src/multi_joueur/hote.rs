use std::io;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use crate::preparation::crée_joueur;

pub fn hote(){

    let mut joueur = crée_joueur();
    //crée joueur

    println!("je suis maintenant un hote joueur");

    let nb_client:usize= 1;


    tokio::runtime::Runtime::new().unwrap().block_on(async {
        connextion_au_client(nb_client).await.unwrap();
    });

    println!("Je suis l'hote et tout vas bien !!");



    //si client se connecter et attendre
    //si h proposer la connection et attendre mon bon vouloir
    //si h crée la liste et la partager

}

async  fn connextion_au_client(nb_client:usize) -> Result<(), Box<dyn std::error::Error>>{
    let listener = TcpListener::bind("0.0.0.0:9000").await?;
    for i in 0..nb_client {
        //loop {
        // Accepter une connexion entrante
        let (mut socket, addresse) = listener.accept().await?;

        // Gérer la connexion
        tokio::spawn(async move {
            let mut buf = [0; 1024];

            loop {
                // Lire les données envoyées par le client
                let n = match socket.read(&mut buf).await {
                    Ok(0) => {
                        eprintln!("connexion {} fermée", addresse);
                        return;
                    } // Connexion fermée
                    Ok(n) => n,
                    Err(_) => {
                        eprintln!("erreur de lecture du client {}", addresse);
                        return;
                    } // Erreur de lecture
                };

                // Écrire une réponse au client
                if let Err(_) = socket.write_all(&buf[..n]).await {
                    return; // Erreur d'écriture
                }
            }
        });

        // }
    }
    Ok(())

}