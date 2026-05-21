use anyhow::{Context, Result};
use env_logger::Env;
use std::fs;
use std::path::PathBuf;

use atualiza_certificados::certificados::Certificado;
use clap::Parser;

#[derive(Debug, Parser)]
#[command(version, about = "Atualizador de cerfificados")]
struct Cli {
    diretorio_entrada: PathBuf,
    diretorio_saida: Option<PathBuf>,
}

fn main() -> Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let cli = Cli::parse();
    log::info!(
        "Obtendo arquivos de certificados a partir do diretório \"{}\":",
        cli.diretorio_entrada.display()
    );
    let diretorio_saida = cli
        .diretorio_saida
        .unwrap_or_else(|| PathBuf::from("gerados"));
    println!();

    let read_dir = fs::read_dir(&cli.diretorio_entrada).with_context(|| {
        format!(
            "Erro ao ler diretório \"{}\"",
            cli.diretorio_entrada.display()
        )
    })?;

    read_dir
        .filter_map(|e| {
            let arquivo = match e {
                Ok(arquivo) => arquivo.path(),
                Err(e) => {
                    log::info!("Erro ao ler diretório: {e:#}.");
                    return None;
                }
            };
            if arquivo.extension().is_some_and(|ext| ext == "json") {
                Some(arquivo)
            } else {
                log::info!("Ignorando arquivo \"{}\"", arquivo.display());
                println!();
                None
            }
        })
        .for_each(|arquivo| {
            log::info!("Processando arquivo \"{}\"", arquivo.display());
            match Certificado::processar(&arquivo, &diretorio_saida) {
                Ok(saida) => log::info!(
                    "Arquivos \"{0}\" e \"{0}.hash\" gerados com sucesso.",
                    saida.display()
                ),
                Err(erro) => eprintln!(
                    "Erro ao processar arquivo \"{}\": {erro:#}",
                    arquivo.display()
                ),
            }
            println!();
        });
    Ok(())
}
