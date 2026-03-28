export function createEditor(target, initialText) {
  const editor = window.CodeMirror(target, {
    value: initialText,
    mode: 'text/x-java',
    theme: 'material-darker',
    lineNumbers: true,
    lineWrapping: false,
    tabSize: 2,
  });

  return editor;
}
