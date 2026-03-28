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
   - A publicação oficial acontece via GitHub Actions em `.github/workflows/release.yml`.
   - Todo merge para `main` deve disparar build e publicação de release automaticamente.
   - A release deve conter os binários compilados da versão para Linux, macOS e Windows.
4. **Checklist mínimo antes de finalizar**.
   - Código atualizado.
   - `STATUS.md` atualizado.
   - Commit realizado.
   - PR pronto para merge em `main` (o merge dispara a release).

## Convenções rápidas
- Manter frontend em `frontend/`.
- Manter backend em `src-tauri/`.
- Manter comandos Rust em `src-tauri/src/commands/`.
