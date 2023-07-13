use medman::cli::CliArguments;
use medman::interface::interaction;
use medman::scan::scan;
use medman::search::search;
use medman::write2md::write;
use medman::writeplaylist::writeplay;
use medman::tag::tag;


/// La fonction main est à la base de tout : Elle identifie la commande entrée et sélectionne en conséquent
/// la fonction à executer. Si rien n'est saisi par l'utilisateur, une interface interactive apparait.

fn main() {
    let args = CliArguments::new();
    match args.command{
        Some(_) =>{
            if args.commande() == "Scan" {
                let music_files = scan(args.path());
                for music_file in music_files {
                    println!("{:?}", music_file);
                }
            }
            if args.commande() == "Search" {
                let musicfile = scan(args.path());
                let res = search(args.filtre(), musicfile);
                for music_file in res {
                    println!("{:?}", music_file);
                }
            }
            if args.commande() == "Write2md" {
                write(args.path());
            }
            if args.commande() == "Writeplay" {
                writeplay(args.path());
            }
            if args.commande() == "Tag" {
                tag(args.filtre());
            }
        },
        None => {interaction()},
    }

}
