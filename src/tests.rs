mod tests {
    use std::fs::read_to_string;
    use std::path::{Path, PathBuf};
    use crate::musicfile::MusicFile;
    use crate::scan::scan;
    use crate::search::search;
    use crate::write2md::write;
    use crate::writeplaylist::writeplay;
    #[test]
    /// Fonction test de la fonction scan : on compare des données prédéfinies avec celles d'un fichier qui devraient
    /// être identiques si la fonction scan est correcte.
    fn test_scan(){
        let chemin = PathBuf::from("src\\test_file");
        let _artiste = String::from("KOSMORIDER");
        let _album = String::from("GEOMETRY");
        let _title = String::from("NIGHT");
        let pathy = Path::new("src\\test_file\\Kosmorider-Night.mp3");
        let temoin = MusicFile::new(pathy, Some(_artiste), Some(_album), 
        Some(_title), None, None);
        let test = &scan(&chemin)[0];
        assert_eq!(*test, temoin);
    }
    #[test]
    fn test_search(){
        let mut filtre : Vec<String> = Vec::new();
        filtre.push("artist".to_string());
        filtre.push("KOSMORIDER".to_string());
        filtre.push("AND".to_string());
        filtre.push("title".to_string());
        filtre.push("NIGHT".to_string());
        let chemin = PathBuf::from("src\\test_file");
        let _artiste = String::from("KOSMORIDER");
        let _album = String::from("GEOMETRY");
        let _title = String::from("NIGHT");
        let pathy = Path::new("src\\test_file\\Kosmorider-Night.mp3");
        let temoin = MusicFile::new(pathy, Some(_artiste), Some(_album), 
        Some(_title), None, None);
        let docu = scan(&chemin);
        let test = search(filtre,docu);
        assert_eq!(test[0], temoin);
    }
    #[test]
    fn test_write2md(){
        let chemin = PathBuf::from("src\\test_md\\testmd1");
        let mut temoin = String::new();
        temoin.push_str("# **Métadonnées du fichier n°1**\n\n\n\n**Titre** : NIGHT\n\n**Artiste** : KOSMORIDER\n\n**Album** : GEOMETRY\n\n**Année** : Aucune donnée\n\n**Durée** : Aucune donnée\n\n**Chemin** : src\\\\test\\_file\\\\Kosmorider\\-Night\\.mp3\n\n");
        write(&chemin);
        let test = read_to_string(chemin.with_extension("md")).expect("Lecture du fichier test impossible.");
        assert_eq!(test, temoin);
    }
    #[test]
    fn test_writeplaylist(){
        let chemin = PathBuf::from("src\\test_playlist\\testplay");
        let mut temoin = String::new();
        temoin.push_str("C:\\Users\\sawey\\Documents\\Ecole\\L3\\Rust\\Projet\\projet-medman-uvsq22003661-main\\src\\test_file\\Kosmorider-Night.mp3\n");
        writeplay(&chemin);
        let test = read_to_string(chemin.with_extension("m3u")).expect("Lecture du fichier test impossible.");
        assert_eq!(test, temoin);
    }
}