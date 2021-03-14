import { parse, tokenize, interpret } from 'wasm';
import { EditorState } from '@codemirror/next/state';
import { cpp } from '@codemirror/next/lang-cpp';
import { EditorView, basicSetup } from '@codemirror/next/basic-setup';
import { tagExtension, EditorSelection } from '@codemirror/next/state';
import code from './code.js';
import { astStringToAst, generateHtmlFromAstNode } from './ast';
const languageTag = Symbol('language');

const editor = new EditorView({
  state: EditorState.create({
    doc: code['fibonacci'],
    extensions: [
      basicSetup,
      tagExtension(languageTag, cpp()),
      // autoLanguage,
    ],
    selection: EditorSelection.create([
      EditorSelection.range(58, 101),
      EditorSelection.cursor(101),
    ]),
  }),
  parent: document.querySelector('#editor'),
});

const parse_button = document.getElementById('parse_button');
const tokenize_button = document.getElementById('tokenize_button');
const interpret_button = document.getElementById('interpret_button');
const codeSelect = document.getElementById('code-select');
const result = document.getElementById('result');
const interactiveAst = document.getElementById('interactive-ast');

parse_button.addEventListener('click', function () {
  controlVisibility('parse');
  const parserResult = parse(editor.state.doc.text.join('\n'));
  interactiveAst.innerHTML = generateHtmlFromAstNode(
    astStringToAst(parserResult),
  );
  bindEventToAstNode();
  result.textContent = parserResult;
});

tokenize_button.addEventListener('click', function () {
  removeEventToAstNode();
  controlVisibility('tokenize');
  const tokenizeResult = tokenize(editor.state.doc.text.join('\n'));
  result.textContent = tokenizeResult;
});
interpret_button.addEventListener('click', function () {
  controlVisibility('interpret');
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

function controlVisibility(mode) {
  if (mode === 'parse') {
    interactiveAst.style.display = 'block';
    result.style.display = 'none';
  } else {
    interactiveAst.style.display = 'none';
    result.style.display = 'inline-block';
  }
}

function onMouseEnterAstNode(event) {
  const parent = event.target.parentElement;
  if (parent && parent.classList.contains('ast-node')) {
    parent.classList.add('highlight');
  }
}

function onMouseLeaveAstNode(event) {
  const parent = event.target.parentElement;
  if (parent && parent.classList.contains('ast-node')) {
    parent.classList.remove('highlight');
  }
}
function onClickAstNode(event) {
  const parent = event.target.parentElement;
  if (parent && parent.classList.contains('ast-node')) {
    const start = parent.dataset.start;
    const end = parent.dataset.end;
    if (start !== 'undefined') {
      editor.dispatch({
        selection: EditorSelection.create([
          EditorSelection.range(+start, +end),
          EditorSelection.cursor(+end),
        ]),
      });
      // editor.dispatch([{
      // }]);
    }
  }
}
function bindEventToAstNode() {
  document.querySelectorAll('.ast-child').forEach((node) => {
    node.addEventListener('mouseenter', onMouseEnterAstNode);
    node.addEventListener('mouseleave', onMouseLeaveAstNode);
    node.addEventListener('click', onClickAstNode);
  });
}
function removeEventToAstNode() {
  document.querySelectorAll('.ast-child').forEach((node) => {
    node.removeEventListener('mouseenter', onMouseEnterAstNode);
    node.removeEventListener('mouseleave', onMouseLeaveAstNode);
    node.removeEventListener('click', onClickAstNode);
  });
}
