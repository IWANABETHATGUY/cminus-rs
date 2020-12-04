import { parse, tokenize, interpret } from 'wasm'

const code = 
`int fibonacci(int a) {
  if (a < 2) {
    return a;
  }
  return fibonacci(a - 1) + fibonacci(a - 2);
}
void main() {
  print(fibonacci(10));
}
`
import { EditorState } from "@codemirror/next/state"
import { cpp } from "@codemirror/next/lang-cpp"
import { EditorView, basicSetup } from '@codemirror/next/basic-setup'
import { tagExtension } from "@codemirror/next/state"
const languageTag = Symbol("language")

const editor = new EditorView({
  state: EditorState.create({
    doc: code,
    extensions: [
      basicSetup,
      tagExtension(languageTag, cpp()),
      // autoLanguage,
    ],
  }),
  parent: document.querySelector("#editor"),
})


const parse_button = document.getElementById('parse_button')
const tokenize_button = document.getElementById('tokenize_button')
const interpret_button = document.getElementById('interpret_button')
const result = document.getElementById('result')


parse_button.addEventListener('click', function () {
  const parserResult = parse(editor.state.doc.text.join('\n'))
  result.textContent = parserResult
})

tokenize_button.addEventListener('click', function () {
  const tokenizeResult = tokenize(editor.state.doc.text.join('\n'))
  result.textContent = tokenizeResult
})
interpret_button.addEventListener('click', function () {
  const interpretResult = interpret(editor.state.doc.text.join('\n'))
  result.textContent = interpretResult
})