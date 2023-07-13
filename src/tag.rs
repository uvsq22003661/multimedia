use std::{fs::read_to_string};
use id3::{Error, ErrorKind,Tag,TagLike,Version};
use crate::{musicfile::MusicFile};

/// Fonction permettant de remplacer la valeur des métadonnées correspondant à la recherche, par exemple on
/// peut modifier tous les artistes par x.
 
 

pub fn tag(elements : Vec<String>){
    let contenu = read_to_string("save.json").expect("Impossible de lire le contenu du fichier.");
    let music_file : Vec<MusicFile> = serde_json::from_str(&contenu).expect("Impossible de créer le nouveau MusicFile.") ;
    for elt in &music_file{
        let mut add = match Tag::read_from_path(elt.path.clone()){
            Ok(x)=>x,
            Err(Error{kind:ErrorKind::NoTag, ..})=>Tag::new(),
            Err(_err9) => panic!("Erreur au tag")};
            match elements[0].as_str(){
                "artist" => add.set_artist(elements[1].clone()),
                "album" => add.set_album(elements[1].clone()),
                "title" => add.set_title(elements[1].clone()),
                "year" => add.set_year(elements[1].clone().parse::<i32>().expect("Problème pour la valeur de l'année.")),
                "duration" => add.set_duration(elements[1].clone().parse::<u32>().expect("Problème pour la valeur de la durée.")),
                &_ => {}
            }
            add.write_to_path(elt.path.clone(),Version::Id3v24).expect("erreur")
    
    }
}
