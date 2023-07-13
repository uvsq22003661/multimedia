use std::path::{Path, PathBuf};
use serde::{Serialize, Deserialize};

/// Implémente une strucute MusicFile qui nous permettra de manipuler plus aisément les métadonnées dans toutes
/// les prochaines fonctions. On utilisera le chemin, l'artiste, l'album, le titre, l'année et la durée en métadonnées.

#[derive(Serialize, Deserialize, Debug, Clone)]
#[derive(PartialEq, Eq)]
pub struct MusicFile {
    pub path: PathBuf,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub title: Option<String>,
    pub year: Option<i32>,
    pub duration : Option<u32>,
}

impl MusicFile {
    pub fn new(path: &Path, artist : Option<String>, album: Option<String>, title: Option<String>, year : Option<i32>,
        duration : Option<u32>) -> MusicFile {
        MusicFile {
            path: path.to_path_buf(),
            artist,
            album, 
            title,
            year,
            duration,
        }
    }
}
