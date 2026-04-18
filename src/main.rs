use clap::Parser;
use colored::*;
use inquire::Password;
use zxcvbn::zxcvbn;
use sha1::{Sha1, Digest};
use rand::seq::SliceRandom;

#[derive(Parser)]
struct Cli {
    /// Ativa a verificação online de senhas vazadas (HIBP)
    #[arg(short, long)]
    leak_check: bool,

    /// Sugere uma frase-senha segura baseada em palavras
    #[arg(short, long)]
    suggest: bool,
}

fn main() {
    let cli = Cli::parse();

    println!("{}", "=== PASS-AUDIT RUST ===".bold().bright_green());

    // 1. INPUT OCULTO
    let password = Password::new("Digite a senha para auditoria:")
        .without_confirmation()
        .prompt()
        .expect("Falha ao ler entrada");

    if password.is_empty() {
        println!("{}", "Erro: A senha não pode ser vazia.".red());
        return;
    }

    // 2. ANÁLISE DE ENTROPIA
    let estimate = zxcvbn(&password, &[]).expect("Falha ao analisar a senha");
    
    println!("\n{}", "--- RELATÓRIO DE FORÇA ---".bold().cyan());
    render_score_bar(estimate.score());
    
    // LINHA CORRIGIDA AQUI (Sem duplicidade e sem erro de sintaxe)
        println!("Tempo estimado para quebra: {}", 
        estimate.crack_times().offline_fast_hashing_1e10_per_second().to_string().bright_magenta());


    // 3. VERIFICAÇÃO DE VAZAMENTO REAL
    if cli.leak_check {
        check_pwned_api(&password);
    }

    // 4. GERADOR DE SUGESTÃO
    if cli.suggest || estimate.score() < 3 {
        generate_passphrase();
    }
}


fn render_score_bar(score: u8) {
    let blocks = "█".repeat(score as usize + 1);
    let empty = "░".repeat(4 - score as usize);
    let color = match score {
        0..=1 => "red",
        2..=3 => "yellow",
        _ => "green",
    };
    println!("Força: [ {}{} ] {}/4", blocks.color(color), empty, score);
}

fn check_pwned_api(password: &str) {
    println!("\n{}", "Consultando base de dados de vazamentos (K-Anonymity)...".dimmed());
    
    let mut hasher = Sha1::new();
    hasher.update(password.as_bytes());
    let hash = format!("{:X}", hasher.finalize());
    
    let prefix = &hash[..5];
    let suffix = &hash[5..];

    let url = format!("https://api.pwnedpasswords.com/range/{}", prefix);
    
    match reqwest::blocking::get(url) {
        Ok(response) => {
            let body = response.text().unwrap_or_default();
            // A API retorna uma lista de sufixos; verificamos se o nosso está lá
            if body.to_uppercase().contains(&suffix.to_uppercase()) {
                println!("{}", "⚠️ ATENÇÃO: Esta senha já foi encontrada em vazamentos públicos!".on_red().white());
            } else {
                println!("{}", "✅ Seguro: Nenhuma ocorrência encontrada nos vazamentos conhecidos.".green());
            }
        }
        Err(_) => println!("{}", "Erro ao conectar na API de vazamentos.".red()),
    }
}

fn generate_passphrase() {
    let words = vec![
        "alfa", "bravo", "codigo", "dados", "eco", "ferro", "pixel", 
        "nuvem", "rede", "seguro", "terra", "vortex", "zero", "rust",
        "linux", "etec", "mogi", "arca", "cyber", "sentinel"
    ];
    let mut rng = rand::thread_rng();
    
    let chosen: Vec<_> = words.choose_multiple(&mut rng, 4).cloned().collect();
    
    println!("\n{}", "Dica: Use uma 'Passphrase' como esta:".yellow().bold());
    println!("> {}", chosen.join("-").cyan());
    println!("{}", "Frases longas são mais fáceis de lembrar e muito mais seguras.".dimmed());
}
