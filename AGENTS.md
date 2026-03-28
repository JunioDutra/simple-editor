# AGENTS.md

## Objetivo
Este repositório segue o PRD em `prd.md` e prioriza evolução incremental por versões (V0, V1, ...).

## Diretrizes para próximas implementações
1. **Sempre atualizar o status de implementação** antes de abrir PR.
   - O status oficial fica em `STATUS.md`.
   - Cada entrega deve marcar o que foi concluído e o que ainda está pendente.
2. **Escopo enxuto por PR**.
   - Implementar apenas o necessário para o checkpoint atual.
   - Evitar adicionar funcionalidades fora do roadmap ativo.
3. **Fluxo obrigatório de release ao final de cada entrega**.
   - Toda implementação finalizada deve terminar com uma release Git.
   - A publicação oficial deve acontecer via GitHub Actions em `.github/workflows/release.yml`.
   - A release deve conter os binários compilados da versão para Linux, macOS e Windows.
   - Nome sugerido de tag: `vMAJOR.MINOR.PATCH`.
4. **Checklist mínimo antes de finalizar**.
   - Código atualizado.
   - `STATUS.md` atualizado.
   - Commit realizado.
   - Tag de versão criada (ex.: `v0.1.0`) para disparar a pipeline de release.

## Convenções rápidas
- Manter frontend em `frontend/`.
- Manter backend em `src-tauri/`.
- Manter comandos Rust em `src-tauri/src/commands/`.
