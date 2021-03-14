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
  if (hasParsingError(astString)) {
    return null;
  }
  const astList = astString
    .split('\n')
    .filter(Boolean)
    .map((string) => deSerializeAst(string));
  if (astList.length) {
    const program = astList.shift();
    const stack = [program];
    while (astList.length) {
      if (astList[0].level > top(stack).level) {
        const cur = astList.shift();
        top(stack).children.push(cur);
        stack.push(cur);
      } else {
        stack.pop();
      }
    }
    // console.log(JSON.stringify(program, null, 2));
    return program;
  }
  return null;
}

function top(stack) {
  return stack[stack.length - 1];
}
/**
 *
 *
 * @param {*} astString
 * @returns AstNode
 */
function deSerializeAst(astString) {
  try {
    const index = firstIndexNotEmpty(astString);
    const [type, codespan] = astString.split('@');
    if (!codespan) {
      return new AstNode(astString.trim(), index / 2);
    }
    const [start, end] = codespan.split('..');
    return new AstNode(type.trim(), index / 2, start, end);
  } catch (err) {
    return null;
  }
}

function firstIndexNotEmpty(string) {
  let i = 0;
  while (string[i] === ' ') {
    i++;
  }
  return i;
}

function hasParsingError(astString) {
  return !astString.startsWith('Program');
}
/**
 *
 * @param {AstNode} astNode
 */
export function generateHtmlFromAstNode(astNode) {
  const type = astNode.type;
  const codespan =
    astNode.start !== undefined ? `@${astNode.start}..${astNode.end}` : '';
  const children = astNode.children
    .map((child) => {
      return generateHtmlFromAstNode(child);
    })
    .join('');
  return `<ul class="ast-node" data-start="${astNode.start}" data-end="${astNode.end}"><li class="ast-child">${type} ${codespan}</li> ${children}</ul>`;
}
