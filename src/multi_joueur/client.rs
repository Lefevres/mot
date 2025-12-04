use tokio::net::TcpStream;
use tokio::io::{AsyncWriteExt, AsyncReadExt};
use crate::jouer::jouer;
use crate::outils::outils::{demander, se_préparer};


const PORT: &str = ":9000";


#[tokio::main]
pub async fn client(){
    let (mut joueur,_,_,nom) = se_préparer("client".to_string());

    let temp = connection().await.unwrap();

    let mut stream = temp;
    envoie_a_l_hote(&mut stream, nom.clone()).await.expect("J'envoie le nom");


    println!("On attend que l'hote choisisse le nombre de manche…");
    let donnée_initialisation: (usize,Vec<String>) = récupérer_info_initialisation(&mut stream).await;
    let nb_manche = donnée_initialisation.0;
    let liste = donnée_initialisation.1;

    // Lance la partie
    let résultat = jouer(&mut joueur, &liste, nb_manche);
    let résultat = résultat.0.to_string() +";"+ &résultat.1.to_string();
    envoie_a_l_hote(&mut stream, résultat).await.expect("on a un soucis");
    let résultats = reçoit_les_résultats(&mut stream,nom).await;
    afficher_résultat(résultats);
}


fn afficher_résultat(résultats:Vec<(String,usize,usize)>)  {
    println!("\n");
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
        println!("{} a eu {} bonne réponse(s) et {} mauvaise(s) pour un ration de {:.1}%\n",nom,bonne_réponse,mauvaise_réponse,ratio);
    }
}


async fn reçoit_les_résultats(socket: &mut TcpStream,mon_nom : String) -> Vec<(String,usize,usize)> {
    let message = lis_message(socket).await.unwrap();
    let préparation_retour = message.split(";")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    let mut résultats:Vec<(String, usize, usize)> = Vec::new();
    for i in (0..préparation_retour.len()).step_by(3) {
        let nom = &préparation_retour[i];
        if *nom == mon_nom{
            continue;
        }
        let bonne_réponse = préparation_retour[i+1].parse().unwrap();
        let mauvaise_réponse = préparation_retour[i+2].parse().unwrap();
        résultats.push((nom.to_string(),bonne_réponse,mauvaise_réponse));
    }
    résultats
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


async fn connection() -> Result<TcpStream,Box<dyn std::error::Error>> {
    println!("Quelle adresse ip ? (\"ip a\" sous linux)");
    let ip = demander(String::new());

    // Adresse IP du serveur
    let addr = ip+PORT;

    println!("Connexion au serveur {}...", addr);

    let  stream = TcpStream::connect(addr).await?;
    println!("Connecté !");

    Ok(stream)
}