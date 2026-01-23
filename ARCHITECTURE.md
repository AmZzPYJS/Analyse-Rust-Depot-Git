├── src/                      # Dossier des modules Rust 
│   ├── main.rs               # Module principal
│   ├── cli.rs                # Parsing des arguments CLI
│   ├── config.rs             # Lecture de config.toml
│   ├── git_stats.rs          # Statistiques Git (commits, auteurs, intervalles, etc.)
│   ├── command_exec.rs       # Exécution de commandes (cargo, etc.)
│   ├── command_report.rs     # Génération des sorties (CSV/rapport)
│   └── report.rs             # Structuration des résultats
│
├── Cargo.toml                # Dépendances + métadonnées du projet
├── Cargo.lock               
├── config.toml               # Paramètres des commandes à executer dans un dépot
├── README.adoc               # Documentation d’origine
│
├── input.csv                 # Liste des étudiants d'un dépot à analyser
├── output.csv                # Résultats principaux
├── commands_output.csv       # sorties des commandes exécutées
├── rapport.json              # Rapport exporté en JSON
│
└──────────────────────