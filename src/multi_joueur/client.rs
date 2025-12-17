
use tokio::net::TcpStream;
use tokio::io::{AsyncWriteExt, BufReader, AsyncBufReadExt, AsyncReadExt};
use crate::jeux::{Jeux, Mode};
use crate::outils::outils::{demande_nom, demander};
use crate::outils::terminal::{afficher, afficher_str};


const PORT: &str = ":9000";


#[tokio::main]
pub async fn client(){

    
    let nom = demande_nom();

    let temp = connection().await.unwrap();

    let mut stream = temp;

    envoie_a_l_hote(&mut stream, nom.clone()).await.expect("J'envoie le nom");


    afficher_str("On attend que l'hote règle les paramètres…");

    let mut jeux = récupéré_jeux(&mut stream).await.unwrap();



    // Lance la partie

    let résultat = jeux.jouer();

    //let résultat = jouer(&mut joueur, &liste, nb_manche);
    let résultat = résultat.0.to_string() +";"+ &résultat.1.to_string();

    envoie_a_l_hote(&mut stream, résultat).await.expect("on a un soucis");

    let résultats = reçoit_les_résultats(&mut stream,nom).await;

    afficher_résultat(résultats);

}


async  fn récupéré_jeux(socket: &mut TcpStream) -> Option<Jeux> {



    let mut reader = BufReader::new(socket);

    let mut jeux = String::new();

    reader.read_line(&mut jeux).await.expect("TODO: panic message2");

    if jeux.is_empty() {
        afficher_str("Le serveur a fermé la connexion2");
        return None
    }



    let jeux = serde_json::from_str::<Jeux>(&jeux);
    match jeux {
        Ok(jeux) => { Some(jeux) },
        Err(e) => {eprintln!("Erreur de désérialisation : {}", e);None},
    }


}


fn afficher_résultat(résultats:Vec<(String,usize,usize)>)  {
    afficher_str("\n");
    for résultat in résultats {
        let nom = résultat.0;
        let bonne_réponse = résultat.1;
        let mauvaise_réponse = résultat.2;
        let total = bonne_réponse + mauvaise_réponse;
        let ratio = if total > 0 {
            (bonne_réponse as f32 / total as f32) * 100.0
        } else {
            0.0
        };
        afficher(format!("{} a eu {} bonne réponse(s) et {} mauvaise(s) pour un ration de {:.1}%\n",nom,bonne_réponse,mauvaise_réponse,ratio));
    }
}


async fn reçoit_les_résultats(socket: &mut TcpStream,mon_nom : String) -> Vec<(String,usize,usize)> {
    let message = lis_message(socket).await.unwrap();
    let message = message.trim().to_string();
    let préparation_retour = message.split(";")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    let mut résultats:Vec<(String, usize, usize)> = Vec::new();
    for i in (0..préparation_retour.len()).step_by(3) {
        let nom = &préparation_retour[i];
        if *nom == mon_nom{
            continue;
        }

        match préparation_retour.get(i+1) {
            Some(_) => (),
            None => eprintln!("Erreur de conversion"),
        }
        match préparation_retour.get(i+2) {
            Some(_) => (),
            None => eprintln!("Erreur de conversion"),
        }

        let bonne_réponse = préparation_retour[i+1].parse().unwrap();
        let mauvaise_réponse = préparation_retour[i+2].parse().unwrap();
        résultats.push((nom.to_string(),bonne_réponse,mauvaise_réponse));
    }
    résultats
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


async fn connection() -> Result<TcpStream,Box<dyn std::error::Error>> {
    afficher_str("Quelle adresse ip ? (\"ip a\" sous linux)");
    let ip = demander();

    // Adresse IP du serveur
    let addr = ip + PORT;

    afficher(format!("Connexion au serveur {}...", addr));

    let stream = TcpStream::connect(addr).await?;
    afficher_str("Connecté !");

    Ok(stream)
}