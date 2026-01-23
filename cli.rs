// Le fichier seul que j'avais fait dans ce projet sinon j'en ai fait d'autre avec les autres mais je préfère ne pas les afficher car elles ne résultent pas entièrement de mon travail.


//! Module CLI : gestion des arguments de la ligne de commande
//! (chemins du dépôt, des fichiers CSV, JSON, etc.).

use clap::Parser;

/// Arguments de la ligne de commande du programme.
///
/// Exemple d'utilisation :
///
/// ```bash
/// cargo run -- \
///   --repo-path mon_repo \
///   --input-csv input.csv \
///   --output-csv rapport.csv 
///   ou/et
///   --json-output rapport.json
/// ```

#[derive(Parser, Debug)]
#[command(author, version, about = "Analyse d'un dépôt Git étudiant et génération de rapports")]
pub struct Cli {
    /// Chemin du dépôt Git à analyser.
    #[arg(short, long)]
    pub repo_path: String,

    /// Chemin du fichier CSV des étudiants (input).
    #[arg(short, long)]
    pub input_csv: String,

    /// Chemin du fichier CSV de sortie (rapport final).
    #[arg(short, long)]
    pub output_csv: String,

    /// Chemin du fichier JSON de sortie (facultatif).
    #[arg(long)]
    pub json_output: Option<String>,
}

/// Parse les arguments de la ligne de commande et retourne une instance de `Cli`.
pub fn parse_cli() -> Cli {
    Cli::parse()
}