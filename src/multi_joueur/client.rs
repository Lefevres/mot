use std::io;
use tokio::net::TcpStream;
use tokio::io::{AsyncWriteExt, AsyncReadExt};

const port: &str = ":9000";


pub fn client(){
    prépare();
}


fn prépare(){
    tokio::runtime::Runtime::new().unwrap().block_on(async {
        connection().await.unwrap();
    });
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

async fn connection() -> Result<(), Box<dyn std::error::Error>> {
    println!("Quelle adresse ip ? (\"ip a sous\") linux");
    let mut ip = String::new();

    io::stdin()
        .read_line(&mut ip)
        .expect("Erreur lors de l'entrer du nom du joueur'");

    ip = ip.trim().to_string();


    // Adresse IP du serveur
    let addr = ip+port;

    println!("Connexion au serveur {}...", addr);

    let mut stream = TcpStream::connect(addr).await?;
    println!("Connecté !");

    let nom = demande_nom();
    // envoyer un message

    stream.write_all(nom.as_bytes()).await?;

    /*// lire la réponse
    let mut buffer = vec![0; 1024];
    let n = stream.read(&mut buffer).await?;

    println!("Reçu: {}", String::from_utf8_lossy(&buffer[..n]));*/

    Ok(())
}
