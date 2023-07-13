use std::{path::Path, fs::{read_to_string, File}};
use crate::musicfile::MusicFile;
use m3u::{Entry, path_entry};
use std::env::current_dir;

/// Fonction permettant de créer une playlist : Elle ajoute tous les fichiers mp3 du JSON dans un même fichier renvoyant
/// à une playlist permettant de les écouter les uns à la suite des autres.
/// 
/// #Example
/// ```ignore
    /// writeplay("src\\test_playlist\\testplay1")
/// ```

pub fn writeplay(path : &Path){
    let contenu = read_to_string("save.json").expect("Pas de fichier.");
    let music_file : Vec<MusicFile> =  serde_json::from_str(&contenu).expect("Mauvais format.");
    let mut playlist : Vec<Entry> = Vec::new();
    for i in music_file{
        let chemin = path_entry(match current_dir() {
            Ok(x) => {x},
            Err(_err7) => {panic!("Pas de chemin.")},
        }.join(i.path.clone()));
        playlist.push(chemin);
    }

    let mut playlistfile = File::create(path.with_extension("m3u")).expect("Création du fichier impossible.");
    let mut writer = m3u::Writer::new(&mut playlistfile);
        for entry in &playlist {
            writer.write_entry(entry).expect("Pas d'écriture possible.");
        }
}