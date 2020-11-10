import { parse, tokenize } from 'wasm'
const parse_button = document.getElementById('parse_button')
const tokenize_button = document.getElementById('tokenize_button')
const editor = document.getElementById('editor')
const result = document.getElementById('result')


parse_button.addEventListener('click', function () {
  const parserResult = parse(editor.value)
  result.textContent = parserResult
})

tokenize_button.addEventListener('click', function () {
  const tokenizeResult = tokenize(editor.value)
  result.textContent = tokenizeResult
})