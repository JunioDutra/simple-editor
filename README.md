# Simple Editor

Editor desktop leve construído com **Tauri + HTML/JS** para evoluir por versões (V0, V1, ...), conforme o roadmap em `prd.md`.

## Objetivo do projeto
O foco atual é o **V0 (MVP)**:
- abrir um arquivo passado por argumento de linha de comando;
- editar conteúdo com CodeMirror;
- salvar com `Ctrl+S` / `Cmd+S`.

## Estrutura
- `frontend/`: interface (HTML/CSS/JS)
- `src-tauri/`: backend Tauri em Rust
- `src-tauri/src/commands/`: comandos de leitura/escrita de arquivos
- `STATUS.md`: status oficial de implementação
- `prd.md`: requisitos e roadmap por versão

## Como baixar
### Opção 1 — Clonar com Git
```bash
git clone <URL_DO_REPOSITORIO>
cd simple-editor
```

### Opção 2 — Download ZIP
1. Baixe o ZIP do repositório no GitHub.
2. Extraia os arquivos.
3. Abra a pasta do projeto no terminal.

## Como usar (desenvolvimento)
### Pré-requisitos
- Rust toolchain
- Dependências do Tauri para seu sistema operacional

### Executar em modo dev
```bash
cargo tauri dev --manifest-path src-tauri/Cargo.toml -- ./caminho/para/arquivo.java
```

> Dica: no Windows, você pode usar caminho relativo ou absoluto para o arquivo inicial.

## Como usar (binário empacotado)
Após baixar o executável da release:

### Windows (PowerShell)
```powershell
.\daemon.exe .\pasta\arquivo.java
```

### Linux/macOS
```bash
./daemon ./pasta/arquivo.java
```

Se o arquivo for aberto corretamente, o caminho aparece no canto superior direito do editor.

## Roadmap rápido
- **V0**: editor funcional (MVP)
- **V1**: navegação básica de filesystem
- **V2**: painel de logs
- **V3**: integração IA básica
- **V4**: diff/patch
- **V5**: IA em background

Consulte `STATUS.md` para o progresso atualizado.
