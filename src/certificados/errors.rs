use std::io;
use std::path::PathBuf;

use base64::DecodeError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CertificadoError {
    #[error("Erro ao ler arquivo {caminho}")]
    LeituraArquivo {
        #[source]
        source: io::Error,
        caminho: PathBuf,
    },
    #[error("Erro ao deserializar arquivo {caminho}")]
    DeserializacaoJson {
        #[source]
        source: serde_json::Error,
        caminho: PathBuf,
    },
    #[error("Erro ao decodificar conteúdo.")]
    DecodificacaoBase64(#[from] DecodeError),
    #[error("Nome de arquivo inválido")]
    NomeInvalidoArquivo,
    #[error("Erro ao escrever arquivo {caminho}")]
    EscritaArquivo {
        #[source]
        source: io::Error,
        caminho: PathBuf,
    },
}

pub type Result<T> = core::result::Result<T, CertificadoError>;
