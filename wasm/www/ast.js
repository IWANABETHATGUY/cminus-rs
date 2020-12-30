class AstNode {
  constructor(type, level, start, end, children = []) {
    this.level = level;
    this.type = type;

    start !== undefined && (this.start = +start);
    end !== undefined && (this.end = +end);
    this.children = children;
  }
}
export function astStringToAst(astString) {
  const astList = astString
    .split('\n')
    .filter(Boolean)
    .map((string) => deSerializeAst(string));
  if (astList.length) {
    const program = astList.shift()
    const stack = [program];
    while (astList.length) {
      if (astList[0].level > top(stack).level) {
        const cur = astList.shift()
        top(stack).children.push(cur)
        stack.push(cur)
      } else {
        stack.pop();
      }
    }
    console.log(JSON.stringify(program, null, 2))
    return program
  }
  return null
}

function top(stack) {
  return stack[stack.length - 1]
}
/**
 *
 *
 * @param {*} astString
 * @returns AstNode
 */
function deSerializeAst(astString) {
  const index = firstIndexNotEmpty(astString);
  const [type, codespan] = astString.split('@');
  if (!codespan) {
    return new AstNode(astString.trim(), index / 2);
  }
  const [start, end] = codespan.split('..');
  return new AstNode(type.trim(), index / 2, start, end);
}

function firstIndexNotEmpty(string) {
  let i = 0;
  while (string[i] === ' ') {
    i++;
  }
  return i;
}
