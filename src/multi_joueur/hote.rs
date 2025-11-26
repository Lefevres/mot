use std::io;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener,TcpStream};
use crate::affichage::terminal::AffichageTerminal;
use crate::logique::jeux::jeux;
use crate::logique::jouer::jouer;
use crate::mot::cree_liste;
use crate::logique::preparation::{preparation};
use crate::logique::preparer::préparer;

#[tokio::main]
pub async fn hote(){
    let prep = preparation;
    let jeux = jouer;
    
    let mut joueur = prep.crée_joueur();
    let liste = cree_liste();
    let nb_client:usize= demander_nb_joueur();
    let clients = connextion_au_client(nb_client).await.unwrap();
    let noms = clients.0;
    let mut sockets = clients.1;
    for joueur in &noms {
        println!("Bonjour {}",joueur);
    }

    let nb_manche: usize = prep.demander_nb_manche(liste.len());
    message_initialisation(&mut sockets, nb_manche, liste[0..nb_manche*2].to_vec()).await;  //fois deux pour question réponse

    
    let affichage  = AffichageTerminal;
    // Lance la partie
    jeux.jouer(&mut joueur, &affichage, &liste, nb_manche);

    let résultats = recevoir_résultat(sockets).await;

    for i in 0..nb_client {
        println!("{} a eu {} bonne réponses pour {} mauvaise ",noms[i], résultats[i].0, résultats[i].1);
    }




}

async fn recevoir_résultat(sockets : Vec<TcpStream>) -> Vec<(String,String)> {
    let mut résultats:Vec<(String,String)> = Vec::new();
    for mut socket in sockets {
        let buffer = lis_buffer(&mut socket).await.unwrap();
        let mut itérateur = buffer.splitn(2,";");
        let bonne_réponse = itérateur.next().unwrap();
        let mauvaise_réponse = itérateur.next().unwrap();
        let résultat = (bonne_réponse.to_string(),mauvaise_réponse.to_string());
        résultats.push(résultat);
    }
    résultats
}

async fn message_initialisation(sockets: &mut Vec<TcpStream>, nb_manche: usize, questions: Vec<String>){
    let mut message_string:String = String::from(nb_manche.to_string());
    for mess in &questions {
        message_string+=";";
        message_string+= &mess;
    }
    for socket in sockets {
        envoie_message(socket,message_string.clone()).await;
    }

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
        noms_joueurs.push(nom);
        sockets.push(socket);
    }

    Ok((noms_joueurs,sockets))
}