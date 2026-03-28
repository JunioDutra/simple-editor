# Status de Implementação

## Roadmap (baseado em `prd.md`)

### V0 — Editor funcional (MVP)
- [x] Estrutura inicial frontend/backend criada
- [x] Backend com leitura/escrita de arquivo (`read_file`/`write_file`)
- [x] Captura de arquivo inicial por argumento de CLI
- [x] Editor CodeMirror renderizando
- [x] Interceptação de `Ctrl/Cmd+S`
- [ ] Execução ponta-a-ponta validada em ambiente com dependências de build disponíveis

### Infra/Entrega
- [x] Pipeline GitHub Actions para build/release dos binários por tag (`v*`)
- [x] Publicação automática de GitHub Release com artefatos Linux/macOS/Windows

### V1 — Navegação básica (filesystem)
- [ ] Detectar diretório do arquivo
- [ ] Listar arquivos
- [ ] Tree simples
- [ ] Abrir/trocar arquivo no editor

### V2 — Sistema de log
- [ ] Painel de log
- [ ] Mensagens do backend
- [ ] Histórico simples

### V3 — IA básica
- [ ] Botão “analisar arquivo”
- [ ] Chamada IA no backend
- [ ] Exibir sugestões textuais

### V4 — Diff/Patch
- [ ] Receber diff estilo git
- [ ] Preview aceitar/rejeitar
- [ ] Aplicar patch

### V5 — IA em background
- [ ] Análise automática ao abrir
- [ ] Sugestões contínuas
- [ ] Debounce
