use tokio::net::TcpStream;
use tokio::io::{AsyncWriteExt, AsyncReadExt};




pub fn client(){
    println!("Je suis un client");
    tokio::runtime::Runtime::new().unwrap().block_on(async {
        connection().await.unwrap();
    });
}



async fn connection() -> Result<(), Box<dyn std::error::Error>> {
    // Adresse IP du serveur (met celle de l'autre PC !)
    let addr = "172.16.20.134:9000";

    println!("Connexion au serveur {}...", addr);

    let mut stream = TcpStream::connect(addr).await?;
    println!("Connecté !");

    // envoyer un message
    stream.write_all(b"Hello serveur !").await?;

    // lire la réponse
    let mut buffer = vec![0; 1024];
    let n = stream.read(&mut buffer).await?;

    println!("Reçu: {}", String::from_utf8_lossy(&buffer[..n]));

    Ok(())
}
