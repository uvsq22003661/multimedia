use structopt::StructOpt;

/// Enumère les différentes commandes possible en entrée et les arguments qu'elles doivent prendre en compte.

#[derive(StructOpt,Debug)]

pub enum Commandes {
    Scan{
        /// Chemin où trouver les fichiers à analyser.
        #[structopt(parse(from_os_str))]
        path: std::path::PathBuf,
    },
    Search{
        /// Chemin où trouver les fichiers à analyser.
        #[structopt(parse(from_os_str))]
        path: std::path::PathBuf,
        /// Argument pour filtrer la recherche tel que par exemple : artist x AND title y.
        filtre: Vec<String>
    },
    Write2md{
        /// Chemin où l'on créer le nouveau fichier md.
        #[structopt(parse(from_os_str))]
        path : std::path::PathBuf,
    },
    Writeplay{
        /// Chemin où l'on créer la nouvelle playlist.
        #[structopt(parse(from_os_str))]
        path : std::path::PathBuf,
    },
    Tag{
        /// Argument pour savoir quel tag ajouter ou modifier.
        elements : Vec<String>,
    }
}

/// Représente les arguments en paramètres de ligne de commande.
#[derive(Debug)]
#[derive(StructOpt)]


pub struct CliArguments {
    /// Commande à exécuter
    #[structopt(subcommand)]
    pub command: Option<Commandes>,
}

impl CliArguments {
    pub fn new() -> CliArguments {
        CliArguments::from_args()
    }

    pub fn path(&self) -> &std::path::Path {
        match match &self.command {
            Some(x) => x,
            None => panic!("Aucune commande saisie.")
                } {
                    Commandes :: Scan{path} => path,
                    Commandes :: Search{path, filtre:_}  => path,
                    Commandes :: Write2md {path} => path,
                    Commandes :: Writeplay { path } => path,
                    Commandes :: Tag { elements:_ } => panic!("Aucun chemin dans tag."),
        }
    }

    pub fn commande(&self) -> &str {
        match match &self.command {
            Some(x) => x,
            None => panic!("Aucune commande saisie.")
                } {
                    Commandes :: Scan{path:_} =>  "Scan",
                    Commandes :: Search{path :_,filtre: _}  =>  "Search",
                    Commandes :: Write2md {path:_} => "Write2md",
                    Commandes :: Writeplay { path: _ } => "Writeplay",
                    Commandes :: Tag { elements:_ } => "Tag",
        }
    }

    pub fn filtre(&self) -> Vec<String> {
        match match &self.command {
            Some(x) => x,
            None => panic!("Aucune commande saisie.")
                } {
                    Commandes :: Scan{path:_} => panic!("Rien à chercher dans Scan."),
                    Commandes :: Search{path:_,filtre}  =>  filtre.to_vec(),
                    Commandes :: Write2md {path:_} => panic!("Rien à chercher dans write."),
                    Commandes :: Writeplay { path: _ } => panic!("Rien à chercher dans writeplay."),
                    Commandes :: Tag { elements } => elements.to_vec(),
        }
    }

}

impl Default for CliArguments {
    fn default() -> Self {
        Self::new()
    }
}
