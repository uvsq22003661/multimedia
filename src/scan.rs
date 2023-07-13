use std::{path::Path, fs::File, io::Write};
use walkdir::{DirEntry, WalkDir};
use id3::{Tag,TagLike};



use crate::musicfile::MusicFile;

const SUPPORTED_EXTENSIONS: [&str; 1] = ["mp3"];

///Recupère les métadonnées et les stock dans la variable music_files. Parcours le dossier et analyse
///son contenu pour le manipuler.
///Utilise la structure MusicFile créée précédemment. 
/// 
/// #Example
/// ```ignore
/// scan(path)
/// ```

pub fn scan(path: &Path) -> Vec<MusicFile> {
    let mut music_files: Vec<MusicFile> = Vec::new();
    let walker = WalkDir::new(path).into_iter();
    for entry in walker {
        let entry = match entry {
            Ok(entry) => entry,
            Err(ref _err) => {println!("Erreur fichier pour {:?}", entry); break},
        };
        if is_supported(&entry) {
            let tag = match Tag::read_from_path(entry.path()) {
                Ok(tag) => tag,
                Err(_err2) => {println!("Pas de métadonnées pour {:?}", entry); break},
            };

            music_files.push(MusicFile::new(entry.path(),tag.artist().map(|s|s.to_owned()),
            tag.album().map(|s|s.to_owned()),tag.title().map(|s|s.to_owned()),
            tag.year().map(|s|s.to_owned()),tag.duration().map(|s|s.to_owned())));
        }
    };
    let mut chemin = File::create("save.json").expect("Création du fichier json impossible.");
    let serialized = serde_json::to_string_pretty(&music_files).expect("Serialize impossible.");
    writeln!(chemin, "{}", serialized).expect("Ecriture impossible.");
    music_files
}

fn is_supported(entry: &DirEntry) -> bool {
    entry.path().is_file() &&
    SUPPORTED_EXTENSIONS.contains(&entry.path().extension().unwrap().to_str().unwrap())
}