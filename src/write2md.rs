use crate::musicfile::MusicFile;
use markdown_gen::markdown::{Markdown, AsMarkdown};
use std::{fs::{File, read_to_string}, path::Path};

/// Fonction permettant de rendre le fichier JSON plus lisible pour un humain. Il rend la présentation
/// plus agréable, en manipulant les mots pour les rendre plus gros, en gras ou en italique par exemple.
/// 
///  #Example
/// ```ignore
/// write("src\\test_md\\testmd1")
/// ```

pub fn write(path : &Path){
    let modif = File::create(path.with_extension("md")).expect("Création du fichier impossible.");
    let mut markdown = Markdown::new(modif);
    let contenu = read_to_string("save.json").expect("Pas de fichier.");
    let music_file : Vec<MusicFile> =  serde_json::from_str(&contenu).expect("Mauvais format.");
    for (i, item) in music_file.iter().enumerate(){
        let header = "Métadonnées du fichier n°".to_string() + &(i+1).to_string();
        markdown.write(header.bold().heading(1)).expect("Impossible d'écrire.");
        markdown.write("\n").expect("Saut de ligne impossible.");
        markdown.write("Titre".bold().paragraph().append(" : ").append(match item.title.clone() {
            Some(x) => {x},
            None => {"Aucune donnée".to_string()},
        }
        .as_str())).expect("Titre non inséré.");
        markdown.write("Artiste".bold().paragraph().append(" : ").append(match item.artist.clone() {
            Some(x) => {x},
            None => {"Aucune donnée".to_string()},
        }
        .as_str())).expect("Artiste non inséré.");
        markdown.write("Album".bold().paragraph().append(" : ").append(match item.album.clone() {
            Some(x) => {x},
            None => {"Aucune donnée".to_string()},
        }
        .as_str())).expect("Album non inséré.");
        markdown.write("Année".bold().paragraph().append(" : ").append(match item.year {
            Some(x) => {x.to_string()},
            None => {"Aucune donnée".to_string()},
        }
        .as_str())).expect("Année non insérée.");
        markdown.write("Durée".bold().paragraph().append(" : ").append(match item.duration {
            Some(x) => {x.to_string()},
            None => {"Aucune donnée".to_string()},
        }
        .to_string().as_str())).expect("Durée non insérée.");
        markdown.write("Chemin".bold().paragraph().append(" : ").append(item.path.clone()
        .into_os_string().to_str().expect("Conversion impossible."))).expect("Aucun chemin trouvé.");
        }
}


