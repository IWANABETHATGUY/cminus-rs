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
var editor = CodeMirror(document.getElementById('editor'), {
  value: code,
  lineNumbers: true,
  gutters: ["CodeMirror-linenumbers", "CodeMirror-foldgutter"],
  matchBrackets: true,
  mode: "text/x-csrc",
  autoCloseBrackets: true,
  lineWrapping: true,
})

const parse_button = document.getElementById('parse_button')
const tokenize_button = document.getElementById('tokenize_button')
const interpret_button = document.getElementById('interpret_button')
const result = document.getElementById('result')


parse_button.addEventListener('click', function () {
  const parserResult = parse(editor.getValue())
  result.textContent = parserResult
})

tokenize_button.addEventListener('click', function () {
  const tokenizeResult = tokenize(editor.getValue())
  result.textContent = tokenizeResult
})
interpret_button.addEventListener('click', function () {
  const interpretResult = interpret(editor.getValue())
  console.log(interpretResult);
  result.textContent = interpretResult
})