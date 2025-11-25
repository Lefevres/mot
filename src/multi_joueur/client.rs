use std::io;
use tokio::net::TcpStream;
use tokio::io::{AsyncWriteExt, AsyncReadExt};
use crate::affichage::terminal::AffichageTerminal;
use crate::jouer::jouer;
use crate::preparation::{crée_joueur};

const PORT: &str = ":9000";


#[tokio::main]
pub async fn client(){
    let mut stream = prépare().await.unwrap();
    let mut joueur = crée_joueur();
    let affichage  = AffichageTerminal;
    println!("On attend que l'hote choisisse le nombre de manche…");
    let donnée_initialisation: (usize,Vec<String>) = récupérer_info_initialisation(&mut stream).await;
    let nb_manche = donnée_initialisation.0;
    let liste = donnée_initialisation.1;
    // Lance la partie
    jouer(&mut joueur, &affichage, &liste, nb_manche);
}

async fn récupérer_info_initialisation(stream: &mut TcpStream) -> (usize,Vec<String>) {
    let donnée_initialisation_string = lis_message(stream).await.expect("erreur lecture stream");
    let mut donnée_initialisation_string:Vec<String> = donnée_initialisation_string
        .split(";")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    let nb_manche = donnée_initialisation_string[0].parse::<usize>().unwrap();
    donnée_initialisation_string = donnée_initialisation_string.split_off(1);
    (nb_manche,donnée_initialisation_string)
}


async fn envoie_a_l_hote(stream : &mut TcpStream, message:String) -> std::io::Result<()>{
    let message_bytes = message.as_bytes();
    stream.write_all(message_bytes).await?;
    Ok(())
}


async fn lis_message(stream : &mut TcpStream) -> Result<String,Box<dyn std::error::Error>>{
    let mut buffer = vec![0; 1024];
    let n = stream.read(&mut buffer).await?;
    if n == 0 {
        return Err("Le serveur a fermé la connexion".into());
    }
    let message = String::from_utf8_lossy(&buffer[..n]).to_string();
    Ok(message)
}


async fn prépare() -> Result<TcpStream, Box<dyn std::error::Error>> {
    connection().await
}


fn demande_nom() -> String{
    println!("Quel est ton nom ?");
    let mut nom = String::new();

    io::stdin()
        .read_line(&mut nom)
        .expect("Erreur lors de l'entrer du nom du joueur'");

    nom = nom.trim().to_string();
    nom

}


async fn connection() -> Result<TcpStream,Box<dyn std::error::Error>> {
    println!("Quelle adresse ip ? (\"ip a\" sous linux)");
    let mut ip = String::new();

    io::stdin()
        .read_line(&mut ip)
        .expect("Erreur lors de l'entrer du nom du joueur'");

    ip = ip.trim().to_string();


    // Adresse IP du serveur
    let addr = ip+PORT;

    println!("Connexion au serveur {}...", addr);

    let mut stream = TcpStream::connect(addr).await?;
    println!("Connecté !");

    let nom = demande_nom();


    envoie_a_l_hote(&mut stream, nom).await?;


    Ok(stream)
}