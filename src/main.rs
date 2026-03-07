use std::process::{Command, Stdio};
use std::fs;
use dialoguer::{theme::ColorfulTheme, Select, Input};
use console::style;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", style("--- RUST LLM MANAGER PRO ---").bold().magenta());
    println!("{}", style("Prêt à gérer et tester tes modèles locaux ?\n").italic());

    loop {
        let menu_options = vec![
            "1. Installer un modèle (Llama 3, Qwen 2, Mistral)",
            "2. Cloner un modèle depuis GitHub",
            "3. Lister mes modèles installés (Ollama)",
            "4. 💬 Lancer un Chat avec un modèle",
            "5. Désinstaller / Supprimer un modèle",
            "6. Quitter"
        ];

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Menu Principal")
            .items(&menu_options)
            .default(0)
            .interact()?;

        match selection {
            0 => installation_menu(),
            1 => install_from_github(),
            2 => list_models(),
            3 => chat_menu(),
            4 => uninstallation_menu(),
            5 => {
                println!("{}", style("Arrêt du manager. Bon code !").red());
                break;
            }
            _ => unreachable!(),
        }
    }

    Ok(())
}

// --- Nouveau : Menu de Chat ---
fn chat_menu() {
    let model: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Nom du modèle avec lequel discuter (ex: llama3)")
        .interact_text()
        .unwrap();

    println!("\n💬 {} (Tapez '/exit' pour quitter le chat)\n", style("Connexion au LLM").green());

    // Pour le chat, on DOIT hériter de stdin pour pouvoir taper du texte
    let status = Command::new("ollama")
        .arg("run")
        .arg(&model)
        .stdin(Stdio::inherit()) 
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status();

    if let Err(_) = status {
        println!("{}", style("❌ Impossible de lancer le chat. Vérifiez le nom du modèle.").red());
    }
}

// --- Menu d'installation standard ---
fn installation_menu() {
    let models = vec![("Llama 3", "llama3"), ("Qwen 2", "qwen2"), ("Mistral", "mistral")];
    let names: Vec<&str> = models.iter().map(|m| m.0).collect();

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Quel modèle installer ?")
        .items(&names)
        .interact()
        .unwrap();

    let tag = models[selection].1;
    execute_command("ollama", &["pull", tag], &format!("Installation de {}", tag));
}

// --- Menu de désinstallation ---
fn uninstallation_menu() {
    let options = vec!["Supprimer un modèle Ollama", "Supprimer un dossier GitHub", "Retour"];
    
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Action de suppression")
        .items(&options)
        .interact()
        .unwrap();

    match selection {
        0 => {
            let model: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Nom du modèle à supprimer")
                .interact_text()
                .unwrap();
            execute_command("ollama", &["rm", &model], &format!("Suppression de {}", model));
        },
        1 => {
            let path: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Nom du dossier à supprimer")
                .interact_text()
                .unwrap();
            
            match fs::remove_dir_all(&path) {
                Ok(_) => println!("{}", style("✅ Dossier supprimé.").green()),
                Err(e) => println!("{}: {}", style("❌ Erreur").red(), e),
            }
        },
        _ => (),
    }
}

// --- Liste des modèles ---
fn list_models() {
    println!("\n{}", style("--- Modèles disponibles dans Ollama ---").cyan());
    let _ = Command::new("ollama")
        .arg("list")
        .stdout(Stdio::inherit())
        .status();
    println!("");
}

// --- GitHub Clone ---
fn install_from_github() {
    let url: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("URL GitHub")
        .interact_text()
        .unwrap();

    execute_command("git", &["clone", &url], "Clonage du dépôt");
}

// --- Helper pour exécuter les commandes système ---
fn execute_command(cmd: &str, args: &[&str], message: &str) {
    println!("\n🚀 {}...", style(message).yellow());
    let status = Command::new(cmd)
        .args(args)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status();

    match status {
        Ok(s) if s.success() => println!("{}", style("✅ Opération réussie !\n").green().bold()),
        _ => println!("{}", style("❌ Une erreur est survenue.\n").red()),
    }
}