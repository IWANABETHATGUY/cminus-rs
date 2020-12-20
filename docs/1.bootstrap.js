(window["webpackJsonp"] = window["webpackJsonp"] || []).push([[1],{

/***/ "../pkg/wasm.js":
/*!**********************!*\
  !*** ../pkg/wasm.js ***!
  \**********************/
/*! exports provided: parse, tokenize, interpret */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony import */ var _wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./wasm_bg.wasm */ \"../pkg/wasm_bg.wasm\");\n/* harmony import */ var _wasm_bg_js__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! ./wasm_bg.js */ \"../pkg/wasm_bg.js\");\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"parse\", function() { return _wasm_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"parse\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"tokenize\", function() { return _wasm_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"tokenize\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"interpret\", function() { return _wasm_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"interpret\"]; });\n\n\n\n\n//# sourceURL=webpack:///../pkg/wasm.js?");

/***/ }),

/***/ "../pkg/wasm_bg.js":
/*!*************************!*\
  !*** ../pkg/wasm_bg.js ***!
  \*************************/
/*! exports provided: parse, tokenize, interpret */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* WEBPACK VAR INJECTION */(function(module) {/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"parse\", function() { return parse; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"tokenize\", function() { return tokenize; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"interpret\", function() { return interpret; });\n/* harmony import */ var _wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./wasm_bg.wasm */ \"../pkg/wasm_bg.wasm\");\n\n\nlet WASM_VECTOR_LEN = 0;\n\nlet cachegetUint8Memory0 = null;\nfunction getUint8Memory0() {\n    if (cachegetUint8Memory0 === null || cachegetUint8Memory0.buffer !== _wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer) {\n        cachegetUint8Memory0 = new Uint8Array(_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer);\n    }\n    return cachegetUint8Memory0;\n}\n\nconst lTextEncoder = typeof TextEncoder === 'undefined' ? (0, module.require)('util').TextEncoder : TextEncoder;\n\nlet cachedTextEncoder = new lTextEncoder('utf-8');\n\nconst encodeString = (typeof cachedTextEncoder.encodeInto === 'function'\n    ? function (arg, view) {\n    return cachedTextEncoder.encodeInto(arg, view);\n}\n    : function (arg, view) {\n    const buf = cachedTextEncoder.encode(arg);\n    view.set(buf);\n    return {\n        read: arg.length,\n        written: buf.length\n    };\n});\n\nfunction passStringToWasm0(arg, malloc, realloc) {\n\n    if (realloc === undefined) {\n        const buf = cachedTextEncoder.encode(arg);\n        const ptr = malloc(buf.length);\n        getUint8Memory0().subarray(ptr, ptr + buf.length).set(buf);\n        WASM_VECTOR_LEN = buf.length;\n        return ptr;\n    }\n\n    let len = arg.length;\n    let ptr = malloc(len);\n\n    const mem = getUint8Memory0();\n\n    let offset = 0;\n\n    for (; offset < len; offset++) {\n        const code = arg.charCodeAt(offset);\n        if (code > 0x7F) break;\n        mem[ptr + offset] = code;\n    }\n\n    if (offset !== len) {\n        if (offset !== 0) {\n            arg = arg.slice(offset);\n        }\n        ptr = realloc(ptr, len, len = offset + arg.length * 3);\n        const view = getUint8Memory0().subarray(ptr + offset, ptr + len);\n        const ret = encodeString(arg, view);\n\n        offset += ret.written;\n    }\n\n    WASM_VECTOR_LEN = offset;\n    return ptr;\n}\n\nlet cachegetInt32Memory0 = null;\nfunction getInt32Memory0() {\n    if (cachegetInt32Memory0 === null || cachegetInt32Memory0.buffer !== _wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer) {\n        cachegetInt32Memory0 = new Int32Array(_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer);\n    }\n    return cachegetInt32Memory0;\n}\n\nconst lTextDecoder = typeof TextDecoder === 'undefined' ? (0, module.require)('util').TextDecoder : TextDecoder;\n\nlet cachedTextDecoder = new lTextDecoder('utf-8', { ignoreBOM: true, fatal: true });\n\ncachedTextDecoder.decode();\n\nfunction getStringFromWasm0(ptr, len) {\n    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));\n}\n/**\n* @param {string} source_code\n* @returns {string}\n*/\nfunction parse(source_code) {\n    try {\n        const retptr = _wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_export_0\"].value - 16;\n        _wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_export_0\"].value = retptr;\n        var ptr0 = passStringToWasm0(source_code, _wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_malloc\"], _wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_realloc\"]);\n        var len0 = WASM_VECTOR_LEN;\n        _wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"parse\"](retptr, ptr0, len0);\n        var r0 = getInt32Memory0()[retptr / 4 + 0];\n        var r1 = getInt32Memory0()[retptr / 4 + 1];\n        return getStringFromWasm0(r0, r1);\n    } finally {\n        _wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_export_0\"].value += 16;\n        _wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_free\"](r0, r1);\n    }\n}\n\n/**\n* @param {string} source_code\n* @returns {string}\n*/\nfunction tokenize(source_code) {\n    try {\n        const retptr = _wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_export_0\"].value - 16;\n        _wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_export_0\"].value = retptr;\n        var ptr0 = passStringToWasm0(source_code, _wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_malloc\"], _wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_realloc\"]);\n        var len0 = WASM_VECTOR_LEN;\n        _wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"tokenize\"](retptr, ptr0, len0);\n        var r0 = getInt32Memory0()[retptr / 4 + 0];\n        var r1 = getInt32Memory0()[retptr / 4 + 1];\n        return getStringFromWasm0(r0, r1);\n    } finally {\n        _wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_export_0\"].value += 16;\n        _wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_free\"](r0, r1);\n    }\n}\n\n/**\n* @param {string} source_code\n* @returns {string}\n*/\nfunction interpret(source_code) {\n    try {\n        const retptr = _wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_export_0\"].value - 16;\n        _wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_export_0\"].value = retptr;\n        var ptr0 = passStringToWasm0(source_code, _wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_malloc\"], _wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_realloc\"]);\n        var len0 = WASM_VECTOR_LEN;\n        _wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"interpret\"](retptr, ptr0, len0);\n        var r0 = getInt32Memory0()[retptr / 4 + 0];\n        var r1 = getInt32Memory0()[retptr / 4 + 1];\n        return getStringFromWasm0(r0, r1);\n    } finally {\n        _wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_export_0\"].value += 16;\n        _wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_free\"](r0, r1);\n    }\n}\n\n\n/* WEBPACK VAR INJECTION */}.call(this, __webpack_require__(/*! ./../www/node_modules/webpack/buildin/harmony-module.js */ \"./node_modules/webpack/buildin/harmony-module.js\")(module)))\n\n//# sourceURL=webpack:///../pkg/wasm_bg.js?");

/***/ }),

/***/ "../pkg/wasm_bg.wasm":
/*!***************************!*\
  !*** ../pkg/wasm_bg.wasm ***!
  \***************************/
/*! exports provided: memory, parse, tokenize, interpret, __wbindgen_export_0, __wbindgen_malloc, __wbindgen_realloc, __wbindgen_free */
/***/ (function(module, exports, __webpack_require__) {

eval("\"use strict\";\n// Instantiate WebAssembly module\nvar wasmExports = __webpack_require__.w[module.i];\n__webpack_require__.r(exports);\n// export exports from WebAssembly module\nfor(var name in wasmExports) if(name != \"__webpack_init__\") exports[name] = wasmExports[name];\n// exec imports from WebAssembly module (for esm order)\n\n\n// exec wasm module\nwasmExports[\"__webpack_init__\"]()\n\n//# sourceURL=webpack:///../pkg/wasm_bg.wasm?");

/***/ }),

/***/ "./code.js":
/*!*****************!*\
  !*** ./code.js ***!
  \*****************/
/*! exports provided: default */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\nconst fibonacci = `int fibonacci(int a) {\n  if (a < 2) {\n    return a;\n  }\n  return fibonacci(a - 1) + fibonacci(a - 2);\n}\nvoid main() {\n  print(fibonacci(10));\n}\n`;\n\nconst bubbleSort = `void bubbleSort(int a[], int len) {\n   int i = len - 1;\n   while (i > 0) {\n      int j = 0;\n      while (j < i) {\n         if (a[j] > a[j + 1]) {\n            int tem = a[j];\n            a[j] = a[j + 1];\n            a[j + 1] = tem;\n         }\n         j = j + 1;\n      }\n      i = i- 1;\n   }\n}\nvoid main() {\n   int a[5] = {4, 10, 1, 7, 2};\n   bubbleSort(a, 5);\n   print(a);\n}\n\n`;\n\n/* harmony default export */ __webpack_exports__[\"default\"] = ({\n    fibonacci,\n    bubbleSort\n});\n\n\n//# sourceURL=webpack:///./code.js?");

/***/ }),

/***/ "./index.js":
/*!******************!*\
  !*** ./index.js ***!
  \******************/
/*! no exports provided */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony import */ var _input__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./input */ \"./input.js\");\n\n\n//# sourceURL=webpack:///./index.js?");

/***/ }),

/***/ "./input.js":
/*!******************!*\
  !*** ./input.js ***!
  \******************/
/*! no exports provided */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony import */ var wasm__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! wasm */ \"../pkg/wasm.js\");\n/* harmony import */ var _codemirror_next_state__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! @codemirror/next/state */ \"./node_modules/@codemirror/next/state/dist/index.js\");\n/* harmony import */ var _codemirror_next_lang_cpp__WEBPACK_IMPORTED_MODULE_2__ = __webpack_require__(/*! @codemirror/next/lang-cpp */ \"./node_modules/@codemirror/next/lang-cpp/dist/index.js\");\n/* harmony import */ var _codemirror_next_basic_setup__WEBPACK_IMPORTED_MODULE_3__ = __webpack_require__(/*! @codemirror/next/basic-setup */ \"./node_modules/@codemirror/next/basic-setup/dist/index.js\");\n/* harmony import */ var _code_js__WEBPACK_IMPORTED_MODULE_4__ = __webpack_require__(/*! ./code.js */ \"./code.js\");\n\n\n\n\n\n\nconst languageTag = Symbol('language');\n\nconst editor = new _codemirror_next_basic_setup__WEBPACK_IMPORTED_MODULE_3__[\"EditorView\"]({\n  state: _codemirror_next_state__WEBPACK_IMPORTED_MODULE_1__[\"EditorState\"].create({\n    doc: _code_js__WEBPACK_IMPORTED_MODULE_4__[\"default\"]['fibonacci'],\n    extensions: [\n      _codemirror_next_basic_setup__WEBPACK_IMPORTED_MODULE_3__[\"basicSetup\"],\n      Object(_codemirror_next_state__WEBPACK_IMPORTED_MODULE_1__[\"tagExtension\"])(languageTag, Object(_codemirror_next_lang_cpp__WEBPACK_IMPORTED_MODULE_2__[\"cpp\"])()),\n      // autoLanguage,\n    ],\n  }),\n  parent: document.querySelector('#editor'),\n});\n\nconst parse_button = document.getElementById('parse_button');\nconst tokenize_button = document.getElementById('tokenize_button');\nconst interpret_button = document.getElementById('interpret_button');\nconst codeSelect = document.getElementById('code-select');\nconst result = document.getElementById('result');\n\nparse_button.addEventListener('click', function () {\n  const parserResult = Object(wasm__WEBPACK_IMPORTED_MODULE_0__[\"parse\"])(editor.state.doc.text.join('\\n'));\n  result.textContent = parserResult;\n});\n\ntokenize_button.addEventListener('click', function () {\n  const tokenizeResult = Object(wasm__WEBPACK_IMPORTED_MODULE_0__[\"tokenize\"])(editor.state.doc.text.join('\\n'));\n  result.textContent = tokenizeResult;\n});\ninterpret_button.addEventListener('click', function () {\n  const interpretResult = Object(wasm__WEBPACK_IMPORTED_MODULE_0__[\"interpret\"])(editor.state.doc.text.join('\\n'));\n  result.textContent = interpretResult;\n});\n\ncodeSelect.addEventListener('change', function () {\n  const len = editor.state.doc.toString().length;\n  const selection = codeSelect.value;\n  editor.dispatch({\n    changes: [\n      {\n        from: 0,\n        to: len,\n        insert: _code_js__WEBPACK_IMPORTED_MODULE_4__[\"default\"][convertKebabCaseToCamelCase(selection)],\n      },\n    ],\n  });\n});\n\nfunction convertKebabCaseToCamelCase(string) {\n  const reg = /\\-(\\w+)/g;\n  return string.replace(reg, function (m1, m2) {\n    return m2.slice(0, 1).toUpperCase() + m2.slice(1);\n  });\n}\n\n\n//# sourceURL=webpack:///./input.js?");

/***/ })

}]);