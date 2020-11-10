import { parse } from 'wasm'
const confirm = document.getElementById('confirm')
const editor = document.getElementById('editor')
const result = document.getElementById('result')


confirm.addEventListener('click', function () {
  const parserResult = parse(editor.value)
  result.textContent = parserResult
})