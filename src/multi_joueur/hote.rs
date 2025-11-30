use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener,TcpStream};
use crate::affichage::affichage::Affichage;
use crate::joueur::Joueur;
use crate::jouer::jouer;
use crate::outils::outils::{demander, se_préparer};


#[tokio::main]
pub async fn hote(){
    let nb_client:usize= demander_nb_joueur();
    let (mut joueur,liste,nb_manche,affichage,mon_nom) = se_préparer("hote".to_string());
    let clients = connextion_au_client(nb_client).await.unwrap();
    let mut noms = clients.0;
    let mut sockets = clients.1;

    message_initialisation(&mut sockets, nb_manche, liste[0..nb_manche*2].to_vec()).await;  //fois deux pour question réponse
    let mut résultats:Vec<(String,String)> = Vec::new();
    noms.insert(0,mon_nom.clone());

    partie(&mut joueur, &affichage, &liste, nb_manche, &mut sockets, &mut résultats, &noms).await;
    afficher_résultat(nb_client,&noms,&mon_nom,&résultats);
}

async fn partie(joueur: &mut Joueur, affichage: &dyn Affichage, liste: &Vec<String>, nb_manche: usize, sockets: &mut Vec<TcpStream>, résultats: &mut Vec<(String, String)>, noms : &Vec<String>) -> (usize, usize){
    let mut stop = false;
    while !joueur.fin(nb_manche) && !stop {
        stop = manche(joueur, affichage, liste,sockets,résultats,noms,noms[0].clone()).await;
        met_a_jour_les_résultats(sockets, joueur, résultats).await;
        partage_résultat(sockets,résultats,&noms).await;
    }
    affichage.afficher_score(joueur);
    (joueur.bonne_reponse(),joueur.mauvaise_reponse())

}


async fn manche(joueur: &mut Joueur, affichage: &dyn Affichage, liste: &Vec<String>, sockets: &mut Vec<TcpStream>, résultats: &mut Vec<(String, String)>, noms : &Vec<String>,mon_nom : String) -> bool {
    let mut essai = false;
    affichage.afficher_en_tete();
    affichage.afficher_score(joueur);
    let mot = affichage.afficher_question(joueur.question(), &liste);
    while !essai { //syncroniser les résultats pour le multi ?
        let reponse = demander(String::new());
        let reaction = crate::jouer::réagir(joueur, affichage, &reponse, &mot);
        match reaction.as_str() {
            "stop" => {
                return true;  //on arrete bel et bien
            }

            "suivant" => {
                essai = true;  //l'essai est correcte
            }

            "reposer" => {}

            "affiche" => {
                partage_résultat(sockets,résultats,&noms).await;
                afficher_résultat(sockets.len(),&noms,&mon_nom,&résultats);
            }

            _ => {
                println!("comment on en est arrivé là ?");
            }
        }
    }
    false
}


async fn met_a_jour_les_résultats(sockets :&mut Vec<TcpStream>, moi:&Joueur, résultats: &mut Vec<(String, String)>) {
    résultats.clear();
    résultats.push((moi.bonne_reponse().to_string(),moi.mauvaise_reponse().to_string())); //ajoute mes résultats


    for mut socket in sockets {
        let buffer = lis_buffer(&mut socket).await.unwrap();
        let mut itérateur = buffer.splitn(2,";");
        let bonne_réponse = itérateur.next().unwrap();
        let mauvaise_réponse = itérateur.next().unwrap();
        let résultat = (bonne_réponse.to_string(),mauvaise_réponse.to_string());
        résultats.push(résultat);
    }


}

fn afficher_résultat(nb_client:usize, noms :&Vec<String>, mon_nom :&String, résultats :&Vec<(String,String)>) {
    println!("\n");
    for i in 0..nb_client+1 { //pour l'hote
        let nom = noms[i].clone();
        if *nom == *mon_nom{
            continue;
        }
        let bonne_réponse:usize = résultats[i].0.parse().unwrap();
        let mauvaise_réponse:usize = résultats[i].1.parse().unwrap();
        let total = bonne_réponse+mauvaise_réponse;
        let ratio = if total > 0 {
            (bonne_réponse as f32 / total as f32) * 100.0
        } else {
            0.0
        };
        println!("{} a eu {} bonne réponse(s) pour {} mauvaise(s) pour un ration de {:.1}% \n",nom, résultats[i].0, résultats[i].1,ratio);
    }
}


fn ajoute_mes_résultats(résultats: &mut Vec<(String, String)>, moi:Joueur) -> &mut Vec<(String, String)> {
    let mes_résultats = (moi.bonne_reponse().to_string(),moi.mauvaise_reponse().to_string());
    résultats.insert(0,mes_résultats);
    résultats
}


async fn partage_résultat(sockets: &mut Vec<TcpStream>,résultats: &mut Vec<(String,String)>, noms :&Vec<String>){
    let mut message = "".to_string();
    for i in 0..noms.len() {
        message += &*noms[i]; //nom;br;mr
        message += ";";
        message += &résultats[i].0;
        message += ";";
        message += &résultats[i].1;
        message += ";";
    }
    message.pop();
    for mut socket in sockets {
        envoie_message(&mut socket,&message).await;
    }
}


async fn recevoir_résultat(sockets : &mut Vec<TcpStream>) -> Vec<(String,String)> {
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
        envoie_message(socket,&message_string).await;
    }
}


fn demander_nb_joueur() -> usize {
    println!("Pour combien de joueur ? (hormis toi)");
    loop {
        let nb_joueur = demander(String::new());

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


async fn envoie_message(socket:&mut TcpStream, message:&String){
    let message_bytes = message.as_bytes();
    socket.write_all(message_bytes).await.unwrap();
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