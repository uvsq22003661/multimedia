use crate::musicfile::MusicFile;

/// Fonction permettant de renvoyer les métadonnées d'un fichier mp3 voulu. A partir de l'une des métadonnées, 
/// on renvoie les autres correspondantes. 
/// Filtre : comprend la recherche et l'élément. Music_file : résultat d'un scan.
/// /// #Example
/// ```ignore
/// search(["artist","x"], music_file)
/// ```

pub fn search(filtre : Vec<String>, music_file : Vec<MusicFile>) -> Vec<MusicFile>{
    let mut res : Vec<MusicFile> = Vec::new();
    let recherche = filtre[0].clone();
    let element = filtre[1].clone();
    for i in &music_file{
        if recherche == "artist"{
            if element == match i.artist.clone() {
                Some(x) => {x},
                None => {continue;},
            }{
                res.push(i.clone());
            }
        }
        else if recherche == "album"{
            if element == match i.album.clone() {
                Some(x) => {x},
                None => {continue},
            }{
                res.push(i.clone());
            }
        }
        else if recherche == "title"{
            if element == match i.title.clone() {
                Some(x) => {x},
                None => {continue},
            }{
                res.push(i.clone());
            }
        }
        else if recherche == "year"{
            if element == match i.year {
                Some(x) => {x},
                None => {continue},
            }.to_string(){
                res.push(i.clone());
            }
        }
        else if recherche == "duration" && element == match i.duration{
                Some(x) => {x},
                None => {continue},
            }.to_string(){
                res.push(i.clone());
        }
    }
    if filtre.len() >= 3{
        let mut rec : Vec<String> = Vec::new();
        rec.extend_from_slice(&filtre[3..]);
        res = search(rec, res);
    }
    res
}
