# atualiza_certificados

Ferramenta de linha de comando para processar arquivos JSON de resposta de API de certificados digitais,
extraindo o conteúdo binário (codificado em Base64) e salvando-o como arquivo `.zip`, junto com um
arquivo `.hash` contendo o SHA-512 do arquivo gerado.

## Descrição

A ferramenta lê um diretório contendo arquivos JSON no formato de resposta de uma API de certificados.
Para cada arquivo encontrado, ela:

1. Desserializa o JSON e extrai o campo `result.contentBase64`.
2. Decodifica o conteúdo Base64 para bytes binários.
3. Salva o binário como `<nome-do-arquivo>.zip` no diretório de saída.
4. Calcula o hash SHA-512 do arquivo gerado e o salva como `<nome-do-arquivo>.zip.hash` (formato hexadecimal).

### Formato esperado do JSON de entrada

```json
{
  "status": "...",
  "code": "...",
  "messages": null,
  "result": {
    "contentType": "...",
    "fileName": "...",
    "contentBase64": "<conteúdo binário em Base64>",
    "checksumCodec": "...",
    "publicKey": "...",
    "signatureAlgorithm": "...",
    "signedChecksum": "..."
  }
}
```

## Dependências

| Crate | Versão | Finalidade |
|---|---|---|
| [`clap`](https://docs.rs/clap) | 4.x | Parsing de argumentos de linha de comando com derive macros |
| [`serde`](https://docs.rs/serde) + [`serde_json`](https://docs.rs/serde_json) | 1.x | Desserialização dos arquivos JSON de entrada |
| [`base64`](https://docs.rs/base64) | 0.22 | Decodificação do conteúdo do certificado em Base64 |
| [`sha2`](https://docs.rs/sha2) | 0.11 | Cálculo do hash SHA-512 dos arquivos gerados |
| [`hex`](https://docs.rs/hex) | 0.4 | Codificação do hash em formato hexadecimal para escrita no arquivo `.hash` |
| [`anyhow`](https://docs.rs/anyhow) | 1.x | Tratamento de erros ergonômico na função `main` |
| [`thiserror`](https://docs.rs/thiserror) | 2.x | Definição de tipos de erro estruturados no módulo `certificados` |
| [`log`](https://docs.rs/log) + [`env_logger`](https://docs.rs/env_logger) | 0.4 / 0.11 | Logging configurável via variável de ambiente `RUST_LOG` |

## Instalação

Requer [Rust](https://www.rust-lang.org/tools/install) (edição 2024, rustc ≥ 1.85).

```bash
git clone <url-do-repositorio>
cd atualiza_certificados
cargo build --release
```

O binário compilado estará em `target/release/atualiza_certificados`.

## Utilização

```
atualiza_certificados <DIRETORIO_ENTRADA> [DIRETORIO_SAIDA]
```

| Argumento | Obrigatório | Descrição |
|---|---|---|
| `DIRETORIO_ENTRADA` | Sim | Diretório contendo os arquivos `.json` a processar |
| `DIRETORIO_SAIDA` | Não | Diretório onde os arquivos gerados serão salvos (padrão: `./gerados`) |

### Exemplos

Processar os JSONs de `./entrada` e salvar os resultados em `./gerados` (padrão):

```bash
./atualiza_certificados ./entrada
```

Especificar um diretório de saída diferente:

```bash
./atualiza_certificados ./entrada ./saida
```

Aumentar o nível de log para depuração:

```bash
RUST_LOG=debug ./atualiza_certificados ./entrada ./saida
```

## Resultado

Para cada arquivo `<nome>.json` processado com sucesso, dois arquivos são criados no diretório de saída:

- `<nome>.zip` — conteúdo binário do certificado decodificado.
- `<nome>.zip.hash` — hash SHA-512 do arquivo `.zip` em formato hexadecimal.

Arquivos com extensão diferente de `.json` encontrados no diretório de entrada são ignorados.
Erros em arquivos individuais são registrados no `stderr` sem interromper o processamento dos demais.

### Exemplo de saída no terminal

```
[INFO] Obtendo arquivos de certificados a partir do diretório "./entrada":

[INFO] Processando arquivo "./entrada/certificado_abc.json"
[INFO] Arquivos "gerados/certificado_abc.zip" e "gerados/certificado_abc.zip.hash" gerados com sucesso.

[INFO] Ignorando arquivo "./entrada/notas.txt"

[INFO] Processando arquivo "./entrada/certificado_xyz.json"
[INFO] Arquivos "gerados/certificado_xyz.zip" e "gerados/certificado_xyz.zip.hash" gerados com sucesso.
```
