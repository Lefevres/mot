use std::io::{Bytes, Read};
use std::io;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener,TcpStream};
use crate::affichage::terminal::AffichageTerminal;
use crate::jouer::jouer;
use crate::mot::cree_liste;
use crate::preparation::{crée_joueur, demander_nb_manche};


#[tokio::main]
pub async fn hote(){

    let mut joueur = crée_joueur(true);
    let liste = cree_liste();
    let nb_client:usize= demander_nb_joueur();

    let clients = connextion_au_client(nb_client).await.unwrap();


    let nom = clients.0;
    let mut sockets = clients.1;


    for joueur in &nom {
        println!("Bonjour {}",joueur);
    }

    let nb_manche: usize = demander_nb_manche(liste.len());
    envoie_message(&mut sockets[0],nb_manche.to_string()).await;

    //il faudrait attendre, il envoie les deux en même temps, donc la réception du nombre de client ce passe mal
    envoie_message_vecteur_string(&mut sockets[0],liste[0..nb_manche].to_vec()).await;

    //envoie les nb_manche première question

    //let affichage  = AffichageTerminal;

    // Lance la partie
    //jouer(&mut joueur, &affichage, &liste, nb_manche);

}



async fn envoie_message_vecteur_string(socket:&mut TcpStream, message:Vec<String>){
    let mut message_bytes = Vec::new();
    for mess in message {
        message_bytes.extend_from_slice(mess.as_bytes());
    }
    socket.write_all(&message_bytes).await.unwrap();

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


async fn lis_buffer(socket:&mut TcpStream) -> Result<String,Box<dyn std::error::Error>>{
    let mut buffer = vec![0u8; 1024];
    let n = socket.read(&mut buffer).await?;
    if n == 0 {
        return Err("Le client a fermé la connexion".into());
    }
    let littérale = String::from_utf8_lossy(&buffer[..n]).to_string();
    Ok(littérale)
}


async fn envoie_message(socket:&mut TcpStream, message:String){
    let message_bytes = message.as_bytes();
    socket.write_all(message_bytes).await.unwrap();
    println!("J'envoie le message : {}", message);
}



async fn connextion_au_client(nb_client: usize) -> Result<(Vec<String>,Vec<TcpStream>), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("0.0.0.0:9000").await?;
    let mut noms_joueurs = Vec::new();
    let mut sockets = Vec::new();
    for _ in 0..nb_client {
        println!("En attente d'un client...");
        let (mut socket, adresse) = listener.accept().await?;
        println!("Client connecté : {}", adresse);

        let nom = lis_buffer(&mut socket).await?;
        /*let mut buffer = vec![0u8; 1024];
        let n = socket.read(&mut buffer).await?;
        if n == 0 {
            return Err("Le client a fermé la connexion".into());
        }*/

        // Convertir en String
        //let nom = String::from_utf8_lossy(&buffer[..n]).to_string();

        noms_joueurs.push(nom);
        sockets.push(socket);

    }

    Ok((noms_joueurs,sockets))
}