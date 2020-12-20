import { parse, tokenize, interpret } from 'wasm';
import { EditorState } from '@codemirror/next/state';
import { cpp } from '@codemirror/next/lang-cpp';
import { EditorView, basicSetup } from '@codemirror/next/basic-setup';
import { tagExtension } from '@codemirror/next/state';
import code from './code.js';
const languageTag = Symbol('language');

const editor = new EditorView({
  state: EditorState.create({
    doc: code['fibonacci'],
    extensions: [
      basicSetup,
      tagExtension(languageTag, cpp()),
      // autoLanguage,
    ],
  }),
  parent: document.querySelector('#editor'),
});

const parse_button = document.getElementById('parse_button');
const tokenize_button = document.getElementById('tokenize_button');
const interpret_button = document.getElementById('interpret_button');
const codeSelect = document.getElementById('code-select');
const result = document.getElementById('result');

parse_button.addEventListener('click', function () {
  const parserResult = parse(editor.state.doc.text.join('\n'));
  result.textContent = parserResult;
});

tokenize_button.addEventListener('click', function () {
  const tokenizeResult = tokenize(editor.state.doc.text.join('\n'));
  result.textContent = tokenizeResult;
});
interpret_button.addEventListener('click', function () {
  const interpretResult = interpret(editor.state.doc.text.join('\n'));
  result.textContent = interpretResult;
});

codeSelect.addEventListener('change', function () {
  const len = editor.state.doc.toString().length;
  const selection = codeSelect.value;
  editor.dispatch({
    changes: [
      {
        from: 0,
        to: len,
        insert: code[convertKebabCaseToCamelCase(selection)],
      },
    ],
  });
});

function convertKebabCaseToCamelCase(string) {
  const reg = /\-(\w+)/g;
  return string.replace(reg, function (m1, m2) {
    return m2.slice(0, 1).toUpperCase() + m2.slice(1);
  });
}
