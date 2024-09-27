use std::io::{self, Write};
use rand::seq::SliceRandom;
use std::fs::File;
use chrono::Local;
use colored::*;
use std::process::Command;

fn get_number_of_participants() -> usize {
    loop {
        let mut input = String::new();
        println!("Veuillez entrer le nombre de participants :");
        io::stdin().read_line(&mut input).expect("Erreur de lecture");
        
        match input.trim().parse::<usize>() {
            Ok(num) => return num,
            Err(_) => println!("Veuillez entrer un nombre valide !"), 
        }
    }
}

fn clear_screen() {
    if cfg!(target_os = "windows") {
        Command::new("cmd").args(&["/C", "cls"]).status().expect("Erreur lors de l'exécution de la commande");
    } else {
        Command::new("clear").status().expect("Erreur lors de l'exécution de la commande");
    }
}

fn shuffle_participants(participants: &mut Vec<String>) {
    let mut rng = rand::thread_rng();
    participants.shuffle(&mut rng);
}

fn ask_confirmation() -> bool {
    loop {
        let mut input = String::new();
        println!("{}", "=========> Voulez-vous passer à la question suivante ? (Y/n)".blue().bold());
        io::stdin().read_line(&mut input).expect("Erreur de lecture");
        match input.trim().to_lowercase().as_str() {
            "y" | "" => return true,
            "n" => return false,
            _ => println!("Veuillez répondre par Y ou n."),
        }
    }
}

fn get_feedback(participant: &str) -> (String, String, String) {
    let mut positive = String::new();
    let mut negative = String::new();
    let mut ameliorate = String::new();

    println!("{} - {}", participant, "Points positifs :".green().bold());
    io::stdin().read_line(&mut positive).expect("Erreur de lecture");
    if !ask_confirmation() {
        println!("Veuillez ressaisir les points positifs :");
        io::stdin().read_line(&mut positive).expect("Erreur de lecture");
    }

    println!("{} - {}", participant, "Points négatifs :".red().bold());
    io::stdin().read_line(&mut negative).expect("Erreur de lecture");
    if !ask_confirmation() {
        println!("Veuillez ressaisir les points négatifs :");
        io::stdin().read_line(&mut negative).expect("Erreur de lecture");
    }
    
    println!("{} - {}", participant, "À améliorer :".yellow().bold());
    io::stdin().read_line(&mut ameliorate).expect("Erreur de lecture");
    if !ask_confirmation() {
        println!("Veuillez ressaisir les points à améliorer :");
        io::stdin().read_line(&mut ameliorate).expect("Erreur de lecture");
    }

    (positive.trim().to_string(), negative.trim().to_string(), ameliorate.trim().to_string())
}

fn export_to_markdown(participants: Vec<(String, String, String, String)>) {
    let _date = Local::now();
    let formatted_date = _date.format("%Y-%m-%d").to_string();
    let file_name = format!("retrospective_{}.md", formatted_date);

    let mut file = File::create(&file_name).expect("Impossible de créer le fichier");
    
    writeln!(file, "# Rétrospective - {}", formatted_date).expect("Erreur d'écriture");
    
    for (participant, positive, negative, ameliorate) in participants {
        writeln!(file, "## {}", participant).expect("Erreur d'écriture");
        
        writeln!(file, "- **Positif**: {}", positive).expect("Erreur d'écriture");
        writeln!(file, "").expect("Erreur d'écriture");

        writeln!(file, "- **Négatif**: {}", negative).expect("Erreur d'écriture");
        writeln!(file, "").expect("Erreur d'écriture");

        writeln!(file, "- **À améliorer**: {}", ameliorate).expect("Erreur d'écriture");
        writeln!(file, "").expect("Erreur d'écriture"); 
    }
}

fn main() {
    
    print!("\x1b]0;RetroNCI\x07");
    println!("{}", "Bienvenue dans RetroNCI!".bold().underline());

    let participants_numb = get_number_of_participants();
    let mut participants = vec![];

    for i in 1..=participants_numb {
        let mut input = String::new();
        println!("Nom du participant {} :", i);
        io::stdin().read_line(&mut input).expect("Erreur de lecture");
        participants.push(input.trim().to_string());
    }

    clear_screen();

    shuffle_participants(&mut participants);
    
    let mut feedbacks = vec![];

    for participant in participants {
        clear_screen();
        let (positif, negatif, ameliorer) = get_feedback(&participant);
        feedbacks.push((participant, positif, negatif, ameliorer));
    }

    println!("{}", "Tous les participants sont passés !".bold().green());

    export_to_markdown(feedbacks);
    println!("Rétrospective enregistrée");
}