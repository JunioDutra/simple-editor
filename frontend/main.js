import { createEditor } from './editor.js';

const fallbackText = `// Simple Editor V0\n// Abra com: daemon arquivo.java\n\npublic class Main {\n  public static void main(String[] args) {\n    System.out.println(\"Hello from V0\");\n  }\n}`;

const tauriInvoke = window.__TAURI__?.core?.invoke;
const fileLabel = document.querySelector('#filename');
const editorRoot = document.querySelector('#editor-root');

const editor = createEditor(editorRoot, fallbackText);
let currentFile = null;

async function loadInitialFile() {
  if (!tauriInvoke) {
    return;
  }

  currentFile = await tauriInvoke('get_current_file');

  if (!currentFile) {
    return;
  }

  try {
    const content = await tauriInvoke('read_file', { path: currentFile });
    fileLabel.textContent = currentFile;
    editor.setValue(content);
  } catch (error) {
    currentFile = null;
    fileLabel.textContent = 'Falha ao abrir arquivo';
    console.error('Falha ao abrir arquivo inicial', error);
  }
}

async function saveCurrentFile() {
  if (!tauriInvoke || !currentFile) {
    return;
  }

  await tauriInvoke('write_file', {
    path: currentFile,
    content: editor.getValue(),
  });
}

window.addEventListener('keydown', async (event) => {
  if ((event.ctrlKey || event.metaKey) && event.key.toLowerCase() === 's') {
    event.preventDefault();
    await saveCurrentFile();
  }
});

loadInitialFile().catch((error) => {
  console.error('Falha ao carregar arquivo inicial', error);
});
