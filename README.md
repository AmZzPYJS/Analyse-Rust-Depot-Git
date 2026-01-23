# Analyse-Rust-Depot-Git

## Objectif
L’objectif de ce projet est d’analyser automatiquement un ensemble de dépôts Git à partir d’un répertoire donné, afin d’en extraire des statistiques pertinentes :
- nombre de commits
- intervalles de temps entre les commits
- volume moyen de modifications par commit
- répartition des contributions par utilisateur

Ces métriques permettent d’obtenir une vision quantitative de l’activité et de l’organisation d’un projet Git.

## Outils et technologies
- **Rust**
- **Cargo**
- **CLI (CLI.rs)** pour le parsing des arguments
- **Librairie git2** pour l’analyse des dépôts Git
- Fichiers CSV pour l’entrée et la sortie des données

Les commandes utilisées incluent notamment :
- `cargo build`
- `cargo run`
- `cargo clippy`
- `cargo doc --open`
- `cargo test`
- `git shortlog -sn --all`

## Fonctionnement général
- Chaque dépôt est parcouru automatiquement et analysé
- Les statistiques sont calculées et agrégées
- Les résultats sont exportés dans des fichiers exploitables (csv, json)

## Contexte académique
Ce projet a été réalisé dans le cadre d’un **travail académique en groupe de 5 personnes**, sur une durée de plus d’un mois.

Le dépôt original a été géré via **GitHub Classroom** et est **privé**.  
Par conséquent, ce dépôt est une **vitrine** présentant :
- le contexte du projet
- ses objectifs techniques
- son architecture
- mes contributions personnelles

## Ce que ce projet démontre
- Maîtrise des bases de **Rust**
- Manipulation de dépôts Git de manière programmatique
- Conception d’un outil d’analyse automatisé
- Travail en équipe avec Git (branches, commits, push)

## Limites et pistes d’amélioration
- Pas d’interface graphique (outil en ligne de commande uniquement)
- Analyse limitée à certaines métriques

## Contributions personnelles
Voir le fichier **CONTRIBUTIONS.md** pour le détail de mes interventions.