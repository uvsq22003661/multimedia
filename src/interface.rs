use std::io::stdin;
use crate::{scan::scan, search::search, write2md::write,writeplaylist::writeplay};

/// Fonction qui permet à l'utilisateur d'intéragir avec le terminal si aucune commande n'est donnée.
pub fn interaction() {
    let mut path = String::new();
    let mut arguments = String::new();
    let mut demande = String::new();
    let mut option = String::new();
    let mut meta = String::new();

    println!("\nVous n'avez insérer aucune commande. Laissez-moi vous guider.\n");

    println!("Scan : Récupère les métadonnées des fichiers d'un dossier donné par un chemin, et les 
    stocke dans un fichier JSON.\n");

    println!("Search : Vous renvoie toutes les métadonnées d'une chanson que vous voulez, vous devrez
    simplement insérer une métadonnée telle que l'artiste par exemple, ou 2 métadonnées
    grâce au connecteur AND.\n");

    println!("Write2md : Créer un fichier où le JSON sera plus lisible par un humain, grâce à un chemin donné.\n");

    println!("Writeplaylist : Créer une playlist permettant d'écouter les musiques contenues dans le fichier
    JSON à la suite.\n");

    println!("Quelle fonction souhaitez-vous utiliser?\n");
   
    'go: loop {
        let _ = stdin().read_line(&mut demande);
        match demande.as_str().trim() {
            "scan" => {
                println!("Donnez le chemin du dossier à scanner.");
                stdin().read_line(&mut path).expect("Chemin inexistant.");
                let path = std::path::Path::new(&path[0..(path.len()-2)]);            
                let music_files = scan(path);
                for i in &music_files {
                    println!("{:?}", i);
                };
                break 'go;},

            "search" => {
                let mut vec1 : Vec<String> = Vec::new();
                'search : loop {
                    println!("Donnez le type de métadonnée que vous souhaitez modifier entre artist, title, album, year et duration.");
                    let _ = stdin().read_line(&mut meta);
                    println!("Que souhaitez-vous inscrire?");
                    let _ = stdin().read_line(&mut arguments);
                    vec1.push(meta.as_str().trim().to_string().clone());
                    let arg : Vec<&str> = arguments.trim().split(' ').collect();
                    for elt in arg {
                        vec1.push(elt.to_string());
                    }
                    println!("Souhaitez vous affiner la recherche?");
                    let _ = stdin().read_line(&mut option);
                    match option.as_str().trim() {
                        "oui" =>{
                            vec1.push("AND".to_string());
                            arguments.clear();
                            meta.clear();
                            option.clear();
                        },
                        "non" => {
                                println!("Donnez le chemin pour effectuer la recherche.");
                                stdin().read_line(&mut path).expect("Chemin non existant.");
                                let path = path.trim();
                                let path2 = std::path::Path::new(&path);
                                let resscan = scan(path2);
                                println!("{:?}",vec1);

                                let music_files = search(vec1, resscan);
                                for i in &music_files {
                                    println!("{:?}", i);
                                };
                                break 'search;
                                }
                        _ => {break 'go;}
                        }
                    }break 'go;
                }
                "write2md" => {
                    println!("Donnez le chemin afin de créer le fichier markdown (Nommez un nouveau fichier à créer).");
                    stdin().read_line(&mut path).expect("Chemin non existant.");
                    let path = path.trim();
                    let path2 = std::path::Path::new(&path);            
                    write(path2);
            break 'go;}
            "writeplaylist" => {
                            println!("Donnez le chemin afin de créer le fichier playlist (Nommez un nouveau fichier à créer).");
                            stdin().read_line(&mut path).expect("Chemin non reconnu");
                            let path = path.trim();
                            let path2 = std::path::Path::new(&path);
                            writeplay(path2);
            break 'go;},
            _ => {},
        }
        demande.clear();
    }
}
        