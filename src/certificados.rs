pub mod errors;

use crate::certificados::errors::{CertificadoError, Result};
use base64::prelude::*;
use sha2::{Digest, Sha512};
use std::fs;
use std::path::{Path, PathBuf};

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Certificado {
    pub status: String,
    pub code: String,
    pub messages: Option<String>,
    pub result: CertificadoResult,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CertificadoResult {
    pub content_type: String,
    pub file_name: String,
    pub content_base64: String,
    pub checksum_codec: String,
    pub public_key: String,
    pub signature_algorithm: String,
    pub signed_checksum: String,
}

impl Certificado {
    /// # Errors
    /// `CertificadoError::LeituraArquivo` ao ler o arquivo json
    /// `CertificadoError::DeserializacaoJson` ao deserializar o conteúdo do arquivo via serde.
    pub fn deserializar(caminho: &Path) -> Result<Self> {
        let conteudo =
            fs::read_to_string(caminho).map_err(|source| CertificadoError::LeituraArquivo {
                source,
                caminho: caminho.to_path_buf(),
            })?;

        serde_json::from_str(&conteudo).map_err(|source| CertificadoError::DeserializacaoJson {
            source,
            caminho: caminho.to_path_buf(),
        })
    }

    /// # Errors
    /// `Certificado::DecodificacaoBase64` ao decodificar o conteúdo base64 do certificado.
    fn decodificar(&self) -> Result<Vec<u8>> {
        let bytes = BASE64_STANDARD.decode(&self.result.content_base64)?;
        Ok(bytes)
    }

    /// # Errors
    /// `CertificadoError::NomeInvalidoArquivo` ao obter steam do arquivo origem.
    pub fn caminho_saida(origem: &Path) -> Result<PathBuf> {
        let stem = origem
            .file_stem()
            .ok_or(CertificadoError::NomeInvalidoArquivo)?;
        let mut origem = PathBuf::from(stem);
        origem.set_extension("zip");
        Ok(origem)
    }

    /// # Errors
    /// `CertificadoError::NomeInvalidoArquivo` ao obter steam do arquivo origem.
    pub fn salvar_hash_512(caminho: &Path) -> Result<()> {
        let bytes = fs::read(caminho).map_err(|source| CertificadoError::LeituraArquivo {
            source,
            caminho: caminho.to_path_buf(),
        })?;
        let hash = Sha512::digest(bytes);

        let mut caminho = PathBuf::from(caminho);
        caminho.set_extension("zip.hash");
        fs::write(&caminho, hex::encode(hash)).map_err(|source| CertificadoError::EscritaArquivo {
            source,
            caminho: caminho.clone(),
        })?;
        Ok(())
    }

    /// # Errors
    /// `CertificadoError::LeituraArquivo` ao ler o arquivo json
    /// `CertificadoError::DeserializacaoJson` ao deserializar o conteúdo do arquivo via serde.
    /// `Certificado::DecodificacaoBase64` ao decodificar o conteúdo base64 do certificado.
    /// `CertificadoError::NomeInvalidoArquivo` ao obter steam do arquivo origem.
    pub fn processar(caminho: &Path) -> Result<PathBuf> {
        let certificado = Self::deserializar(caminho)?;
        let bytes = certificado.decodificar()?;
        let caminho_saida = Self::caminho_saida(caminho)?;

        fs::write(&caminho_saida, bytes).map_err(|source| CertificadoError::EscritaArquivo {
            source,
            caminho: caminho_saida.clone(),
        })?;

        Self::salvar_hash_512(&caminho_saida)?;
        Ok(caminho_saida)
    }
}
