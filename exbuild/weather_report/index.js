/*
 * ATTENTION: The "eval" devtool has been used (maybe by default in mode: "development").
 * This devtool is neither made for production nor for readable output files.
 * It uses "eval()" calls to create a separate source file in the browser devtools.
 * If you are trying to read the output file, select a different devtool (https://webpack.js.org/configuration/devtool/)
 * or disable the default devtool with "devtool: false".
 * If you are looking for production-ready output files, see mode: "production" (https://webpack.js.org/configuration/mode/).
 */
/******/ (() => { // webpackBootstrap
/******/ 	var __webpack_modules__ = ({

/***/ "./index.js":
/*!******************!*\
  !*** ./index.js ***!
  \******************/
/***/ ((__unused_webpack_module, __unused_webpack_exports, __webpack_require__) => {

eval("{// For more comments about what's going on here, check out the `hello_world`\n// example\n__webpack_require__.e(/*! import() */ \"pkg_index_js\").then(__webpack_require__.bind(__webpack_require__, /*! ./pkg */ \"./pkg/index.js\"))\n    .catch(console.error);\n\n\n//# sourceURL=webpack:///./index.js?\n}");

/***/ })

/******/ 	});
/************************************************************************/
/******/ 	// The module cache
/******/ 	var __webpack_module_cache__ = {};
/******/ 	
/******/ 	// The require function
/******/ 	function __webpack_require__(moduleId) {
/******/ 		// Check if module is in cache
/******/ 		var cachedModule = __webpack_module_cache__[moduleId];
/******/ 		if (cachedModule !== undefined) {
/******/ 			return cachedModule.exports;
/******/ 		}
/******/ 		// Create a new module (and put it into the cache)
/******/ 		var module = __webpack_module_cache__[moduleId] = {
/******/ 			id: moduleId,
/******/ 			// no module.loaded needed
/******/ 			exports: {}
/******/ 		};
/******/ 	
/******/ 		// Execute the module function
/******/ 		__webpack_modules__[moduleId](module, module.exports, __webpack_require__);
/******/ 	
/******/ 		// Return the exports of the module
/******/ 		return module.exports;
/******/ 	}
/******/ 	
/******/ 	// expose the modules object (__webpack_modules__)
/******/ 	__webpack_require__.m = __webpack_modules__;
/******/ 	
/******/ 	// expose the module cache
/******/ 	__webpack_require__.c = __webpack_module_cache__;
/******/ 	
/************************************************************************/
/******/ 	/* webpack/runtime/define property getters */
/******/ 	(() => {
/******/ 		// define getter functions for harmony exports
/******/ 		__webpack_require__.d = (exports, definition) => {
/******/ 			for(var key in definition) {
/******/ 				if(__webpack_require__.o(definition, key) && !__webpack_require__.o(exports, key)) {
/******/ 					Object.defineProperty(exports, key, { enumerable: true, get: definition[key] });
/******/ 				}
/******/ 			}
/******/ 		};
/******/ 	})();
/******/ 	
/******/ 	/* webpack/runtime/ensure chunk */
/******/ 	(() => {
/******/ 		__webpack_require__.f = {};
/******/ 		// This file contains only the entry chunk.
/******/ 		// The chunk loading function for additional chunks
/******/ 		__webpack_require__.e = (chunkId) => {
/******/ 			return Promise.all(Object.keys(__webpack_require__.f).reduce((promises, key) => {
/******/ 				__webpack_require__.f[key](chunkId, promises);
/******/ 				return promises;
/******/ 			}, []));
/******/ 		};
/******/ 	})();
/******/ 	
/******/ 	/* webpack/runtime/get javascript chunk filename */
/******/ 	(() => {
/******/ 		// This function allow to reference async chunks
/******/ 		__webpack_require__.u = (chunkId) => {
/******/ 			// return url for filenames based on template
/******/ 			return "" + chunkId + ".index.js";
/******/ 		};
/******/ 	})();
/******/ 	
/******/ 	/* webpack/runtime/global */
/******/ 	(() => {
/******/ 		__webpack_require__.g = (function() {
/******/ 			if (typeof globalThis === 'object') return globalThis;
/******/ 			try {
/******/ 				return this || new Function('return this')();
/******/ 			} catch (e) {
/******/ 				if (typeof window === 'object') return window;
/******/ 			}
/******/ 		})();
/******/ 	})();
/******/ 	
/******/ 	/* webpack/runtime/hasOwnProperty shorthand */
/******/ 	(() => {
/******/ 		__webpack_require__.o = (obj, prop) => (Object.prototype.hasOwnProperty.call(obj, prop))
/******/ 	})();
/******/ 	
/******/ 	/* webpack/runtime/load script */
/******/ 	(() => {
/******/ 		var inProgress = {};
/******/ 		// data-webpack is not used as build has no uniqueName
/******/ 		// loadScript function to load a script via script tag
/******/ 		__webpack_require__.l = (url, done, key, chunkId) => {
/******/ 			if(inProgress[url]) { inProgress[url].push(done); return; }
/******/ 			var script, needAttach;
/******/ 			if(key !== undefined) {
/******/ 				var scripts = document.getElementsByTagName("script");
/******/ 				for(var i = 0; i < scripts.length; i++) {
/******/ 					var s = scripts[i];
/******/ 					if(s.getAttribute("src") == url) { script = s; break; }
/******/ 				}
/******/ 			}
/******/ 			if(!script) {
/******/ 				needAttach = true;
/******/ 				script = document.createElement('script');
/******/ 		
/******/ 				script.charset = 'utf-8';
/******/ 				if (__webpack_require__.nc) {
/******/ 					script.setAttribute("nonce", __webpack_require__.nc);
/******/ 				}
/******/ 		
/******/ 		
/******/ 				script.src = url;
/******/ 			}
/******/ 			inProgress[url] = [done];
/******/ 			var onScriptComplete = (prev, event) => {
/******/ 				// avoid mem leaks in IE.
/******/ 				script.onerror = script.onload = null;
/******/ 				clearTimeout(timeout);
/******/ 				var doneFns = inProgress[url];
/******/ 				delete inProgress[url];
/******/ 				script.parentNode && script.parentNode.removeChild(script);
/******/ 				doneFns && doneFns.forEach((fn) => (fn(event)));
/******/ 				if(prev) return prev(event);
/******/ 			}
/******/ 			var timeout = setTimeout(onScriptComplete.bind(null, undefined, { type: 'timeout', target: script }), 120000);
/******/ 			script.onerror = onScriptComplete.bind(null, script.onerror);
/******/ 			script.onload = onScriptComplete.bind(null, script.onload);
/******/ 			needAttach && document.head.appendChild(script);
/******/ 		};
/******/ 	})();
/******/ 	
/******/ 	/* webpack/runtime/make namespace object */
/******/ 	(() => {
/******/ 		// define __esModule on exports
/******/ 		__webpack_require__.r = (exports) => {
/******/ 			if(typeof Symbol !== 'undefined' && Symbol.toStringTag) {
/******/ 				Object.defineProperty(exports, Symbol.toStringTag, { value: 'Module' });
/******/ 			}
/******/ 			Object.defineProperty(exports, '__esModule', { value: true });
/******/ 		};
/******/ 	})();
/******/ 	
/******/ 	/* webpack/runtime/publicPath */
/******/ 	(() => {
/******/ 		var scriptUrl;
/******/ 		if (__webpack_require__.g.importScripts) scriptUrl = __webpack_require__.g.location + "";
/******/ 		var document = __webpack_require__.g.document;
/******/ 		if (!scriptUrl && document) {
/******/ 			if (document.currentScript && document.currentScript.tagName.toUpperCase() === 'SCRIPT')
/******/ 				scriptUrl = document.currentScript.src;
/******/ 			if (!scriptUrl) {
/******/ 				var scripts = document.getElementsByTagName("script");
/******/ 				if(scripts.length) {
/******/ 					var i = scripts.length - 1;
/******/ 					while (i > -1 && (!scriptUrl || !/^http(s?):/.test(scriptUrl))) scriptUrl = scripts[i--].src;
/******/ 				}
/******/ 			}
/******/ 		}
/******/ 		// When supporting browsers where an automatic publicPath is not supported you must specify an output.publicPath manually via configuration
/******/ 		// or pass an empty string ("") and set the __webpack_public_path__ variable from your code to use your own logic.
/******/ 		if (!scriptUrl) throw new Error("Automatic publicPath is not supported in this browser");
/******/ 		scriptUrl = scriptUrl.replace(/^blob:/, "").replace(/#.*$/, "").replace(/\?.*$/, "").replace(/\/[^\/]+$/, "/");
/******/ 		__webpack_require__.p = scriptUrl;
/******/ 	})();
/******/ 	
/******/ 	/* webpack/runtime/jsonp chunk loading */
/******/ 	(() => {
/******/ 		// no baseURI
/******/ 		
/******/ 		// object to store loaded and loading chunks
/******/ 		// undefined = chunk not loaded, null = chunk preloaded/prefetched
/******/ 		// [resolve, reject, Promise] = chunk loading, 0 = chunk loaded
/******/ 		var installedChunks = {
/******/ 			"main": 0
/******/ 		};
/******/ 		
/******/ 		__webpack_require__.f.j = (chunkId, promises) => {
/******/ 				// JSONP chunk loading for javascript
/******/ 				var installedChunkData = __webpack_require__.o(installedChunks, chunkId) ? installedChunks[chunkId] : undefined;
/******/ 				if(installedChunkData !== 0) { // 0 means "already installed".
/******/ 		
/******/ 					// a Promise means "currently loading".
/******/ 					if(installedChunkData) {
/******/ 						promises.push(installedChunkData[2]);
/******/ 					} else {
/******/ 						if(true) { // all chunks have JS
/******/ 							// setup Promise in chunk cache
/******/ 							var promise = new Promise((resolve, reject) => (installedChunkData = installedChunks[chunkId] = [resolve, reject]));
/******/ 							promises.push(installedChunkData[2] = promise);
/******/ 		
/******/ 							// start chunk loading
/******/ 							var url = __webpack_require__.p + __webpack_require__.u(chunkId);
/******/ 							// create error before stack unwound to get useful stacktrace later
/******/ 							var error = new Error();
/******/ 							var loadingEnded = (event) => {
/******/ 								if(__webpack_require__.o(installedChunks, chunkId)) {
/******/ 									installedChunkData = installedChunks[chunkId];
/******/ 									if(installedChunkData !== 0) installedChunks[chunkId] = undefined;
/******/ 									if(installedChunkData) {
/******/ 										var errorType = event && (event.type === 'load' ? 'missing' : event.type);
/******/ 										var realSrc = event && event.target && event.target.src;
/******/ 										error.message = 'Loading chunk ' + chunkId + ' failed.\n(' + errorType + ': ' + realSrc + ')';
/******/ 										error.name = 'ChunkLoadError';
/******/ 										error.type = errorType;
/******/ 										error.request = realSrc;
/******/ 										installedChunkData[1](error);
/******/ 									}
/******/ 								}
/******/ 							};
/******/ 							__webpack_require__.l(url, loadingEnded, "chunk-" + chunkId, chunkId);
/******/ 						}
/******/ 					}
/******/ 				}
/******/ 		};
/******/ 		
/******/ 		// no prefetching
/******/ 		
/******/ 		// no preloaded
/******/ 		
/******/ 		// no HMR
/******/ 		
/******/ 		// no HMR manifest
/******/ 		
/******/ 		// no on chunks loaded
/******/ 		
/******/ 		// install a JSONP callback for chunk loading
/******/ 		var webpackJsonpCallback = (parentChunkLoadingFunction, data) => {
/******/ 			var [chunkIds, moreModules, runtime] = data;
/******/ 			// add "moreModules" to the modules object,
/******/ 			// then flag all "chunkIds" as loaded and fire callback
/******/ 			var moduleId, chunkId, i = 0;
/******/ 			if(chunkIds.some((id) => (installedChunks[id] !== 0))) {
/******/ 				for(moduleId in moreModules) {
/******/ 					if(__webpack_require__.o(moreModules, moduleId)) {
/******/ 						__webpack_require__.m[moduleId] = moreModules[moduleId];
/******/ 					}
/******/ 				}
/******/ 				if(runtime) var result = runtime(__webpack_require__);
/******/ 			}
/******/ 			if(parentChunkLoadingFunction) parentChunkLoadingFunction(data);
/******/ 			for(;i < chunkIds.length; i++) {
/******/ 				chunkId = chunkIds[i];
/******/ 				if(__webpack_require__.o(installedChunks, chunkId) && installedChunks[chunkId]) {
/******/ 					installedChunks[chunkId][0]();
/******/ 				}
/******/ 				installedChunks[chunkId] = 0;
/******/ 			}
/******/ 		
/******/ 		}
/******/ 		
/******/ 		var chunkLoadingGlobal = self["webpackChunk"] = self["webpackChunk"] || [];
/******/ 		chunkLoadingGlobal.forEach(webpackJsonpCallback.bind(null, 0));
/******/ 		chunkLoadingGlobal.push = webpackJsonpCallback.bind(null, chunkLoadingGlobal.push.bind(chunkLoadingGlobal));
/******/ 	})();
/******/ 	
/******/ 	/* webpack/runtime/wasm chunk loading */
/******/ 	(() => {
/******/ 		// object to store loaded and loading wasm modules
/******/ 		var installedWasmModules = {};
/******/ 		
/******/ 		function promiseResolve() { return Promise.resolve(); }
/******/ 		
/******/ 		var wasmImportedFuncCache0;
/******/ 		var wasmImportedFuncCache1;
/******/ 		var wasmImportedFuncCache2;
/******/ 		var wasmImportedFuncCache3;
/******/ 		var wasmImportedFuncCache4;
/******/ 		var wasmImportedFuncCache5;
/******/ 		var wasmImportedFuncCache6;
/******/ 		var wasmImportedFuncCache7;
/******/ 		var wasmImportedFuncCache8;
/******/ 		var wasmImportedFuncCache9;
/******/ 		var wasmImportedFuncCache10;
/******/ 		var wasmImportedFuncCache11;
/******/ 		var wasmImportedFuncCache12;
/******/ 		var wasmImportedFuncCache13;
/******/ 		var wasmImportedFuncCache14;
/******/ 		var wasmImportedFuncCache15;
/******/ 		var wasmImportedFuncCache16;
/******/ 		var wasmImportedFuncCache17;
/******/ 		var wasmImportedFuncCache18;
/******/ 		var wasmImportedFuncCache19;
/******/ 		var wasmImportedFuncCache20;
/******/ 		var wasmImportedFuncCache21;
/******/ 		var wasmImportedFuncCache22;
/******/ 		var wasmImportedFuncCache23;
/******/ 		var wasmImportedFuncCache24;
/******/ 		var wasmImportedFuncCache25;
/******/ 		var wasmImportedFuncCache26;
/******/ 		var wasmImportedFuncCache27;
/******/ 		var wasmImportedFuncCache28;
/******/ 		var wasmImportedFuncCache29;
/******/ 		var wasmImportedFuncCache30;
/******/ 		var wasmImportedFuncCache31;
/******/ 		var wasmImportedFuncCache32;
/******/ 		var wasmImportedFuncCache33;
/******/ 		var wasmImportedFuncCache34;
/******/ 		var wasmImportedFuncCache35;
/******/ 		var wasmImportedFuncCache36;
/******/ 		var wasmImportedFuncCache37;
/******/ 		var wasmImportedFuncCache38;
/******/ 		var wasmImportedFuncCache39;
/******/ 		var wasmImportedFuncCache40;
/******/ 		var wasmImportedFuncCache41;
/******/ 		var wasmImportedFuncCache42;
/******/ 		var wasmImportedFuncCache43;
/******/ 		var wasmImportedFuncCache44;
/******/ 		var wasmImportedFuncCache45;
/******/ 		var wasmImportedFuncCache46;
/******/ 		var wasmImportedFuncCache47;
/******/ 		var wasmImportedFuncCache48;
/******/ 		var wasmImportedFuncCache49;
/******/ 		var wasmImportedFuncCache50;
/******/ 		var wasmImportedFuncCache51;
/******/ 		var wasmImportedFuncCache52;
/******/ 		var wasmImportedFuncCache53;
/******/ 		var wasmImportedFuncCache54;
/******/ 		var wasmImportedFuncCache55;
/******/ 		var wasmImportedFuncCache56;
/******/ 		var wasmImportedFuncCache57;
/******/ 		var wasmImportedFuncCache58;
/******/ 		var wasmImportedFuncCache59;
/******/ 		var wasmImportedFuncCache60;
/******/ 		var wasmImportedFuncCache61;
/******/ 		var wasmImportedFuncCache62;
/******/ 		var wasmImportedFuncCache63;
/******/ 		var wasmImportedFuncCache64;
/******/ 		var wasmImportedFuncCache65;
/******/ 		var wasmImportedFuncCache66;
/******/ 		var wasmImportedFuncCache67;
/******/ 		var wasmImportedFuncCache68;
/******/ 		var wasmImportedFuncCache69;
/******/ 		var wasmImportedFuncCache70;
/******/ 		var wasmImportedFuncCache71;
/******/ 		var wasmImportedFuncCache72;
/******/ 		var wasmImportedFuncCache73;
/******/ 		var wasmImportedFuncCache74;
/******/ 		var wasmImportedFuncCache75;
/******/ 		var wasmImportedFuncCache76;
/******/ 		var wasmImportObjects = {
/******/ 			"./pkg/index_bg.wasm": function() {
/******/ 				return {
/******/ 					"./snippets/rust-webassembly-weather-reports-7e815c9c0f487b6a/util.js": {
/******/ 						"initialize": function(p0f64,p1f64) {
/******/ 							if(wasmImportedFuncCache0 === undefined) wasmImportedFuncCache0 = __webpack_require__.c["./pkg/snippets/rust-webassembly-weather-reports-7e815c9c0f487b6a/util.js"].exports;
/******/ 							return wasmImportedFuncCache0["initialize"](p0f64,p1f64);
/******/ 						}
/******/ 					},
/******/ 					"./index_bg.js": {
/******/ 						"__wbg_setTimeout_7bb3429662ab1e70": function(p0externref,p1i32) {
/******/ 							if(wasmImportedFuncCache1 === undefined) wasmImportedFuncCache1 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache1["__wbg_setTimeout_7bb3429662ab1e70"](p0externref,p1i32);
/******/ 						},
/******/ 						"__wbg_clearTimeout_7a42b49784aea641": function(p0externref) {
/******/ 							if(wasmImportedFuncCache2 === undefined) wasmImportedFuncCache2 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache2["__wbg_clearTimeout_7a42b49784aea641"](p0externref);
/******/ 						},
/******/ 						"__wbg_fetch_74a3e84ebd2c9a0e": function(p0externref) {
/******/ 							if(wasmImportedFuncCache3 === undefined) wasmImportedFuncCache3 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache3["__wbg_fetch_74a3e84ebd2c9a0e"](p0externref);
/******/ 						},
/******/ 						"__wbg_queueMicrotask_9d76cacb20c84d58": function(p0externref) {
/******/ 							if(wasmImportedFuncCache4 === undefined) wasmImportedFuncCache4 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache4["__wbg_queueMicrotask_9d76cacb20c84d58"](p0externref);
/******/ 						},
/******/ 						"__wbg_queueMicrotask_34d692c25c47d05b": function(p0externref) {
/******/ 							if(wasmImportedFuncCache5 === undefined) wasmImportedFuncCache5 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache5["__wbg_queueMicrotask_34d692c25c47d05b"](p0externref);
/******/ 						},
/******/ 						"__wbg_createTask_9ac11a42c24ef284": function(p0i32,p1i32) {
/******/ 							if(wasmImportedFuncCache6 === undefined) wasmImportedFuncCache6 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache6["__wbg_createTask_9ac11a42c24ef284"](p0i32,p1i32);
/******/ 						},
/******/ 						"__wbg_run_e5e1ecccf06974b2": function(p0externref,p1i32,p2i32) {
/******/ 							if(wasmImportedFuncCache7 === undefined) wasmImportedFuncCache7 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache7["__wbg_run_e5e1ecccf06974b2"](p0externref,p1i32,p2i32);
/******/ 						},
/******/ 						"__wbg_instanceof_Window_4846dbb3de56c84c": function(p0externref) {
/******/ 							if(wasmImportedFuncCache8 === undefined) wasmImportedFuncCache8 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache8["__wbg_instanceof_Window_4846dbb3de56c84c"](p0externref);
/******/ 						},
/******/ 						"__wbg_document_725ae06eb442a6db": function(p0externref) {
/******/ 							if(wasmImportedFuncCache9 === undefined) wasmImportedFuncCache9 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache9["__wbg_document_725ae06eb442a6db"](p0externref);
/******/ 						},
/******/ 						"__wbg_body_8c26b54829a0c4cb": function(p0externref) {
/******/ 							if(wasmImportedFuncCache10 === undefined) wasmImportedFuncCache10 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache10["__wbg_body_8c26b54829a0c4cb"](p0externref);
/******/ 						},
/******/ 						"__wbg_createElement_964ab674a0176cd8": function(p0externref,p1i32,p2i32) {
/******/ 							if(wasmImportedFuncCache11 === undefined) wasmImportedFuncCache11 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache11["__wbg_createElement_964ab674a0176cd8"](p0externref,p1i32,p2i32);
/******/ 						},
/******/ 						"__wbg_getElementById_c365dd703c4a88c3": function(p0externref,p1i32,p2i32) {
/******/ 							if(wasmImportedFuncCache12 === undefined) wasmImportedFuncCache12 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache12["__wbg_getElementById_c365dd703c4a88c3"](p0externref,p1i32,p2i32);
/******/ 						},
/******/ 						"__wbg_set_id_6d16897f248a4f75": function(p0externref,p1i32,p2i32) {
/******/ 							if(wasmImportedFuncCache13 === undefined) wasmImportedFuncCache13 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache13["__wbg_set_id_6d16897f248a4f75"](p0externref,p1i32,p2i32);
/******/ 						},
/******/ 						"__wbg_set_className_30cca9952180bfd1": function(p0externref,p1i32,p2i32) {
/******/ 							if(wasmImportedFuncCache14 === undefined) wasmImportedFuncCache14 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache14["__wbg_set_className_30cca9952180bfd1"](p0externref,p1i32,p2i32);
/******/ 						},
/******/ 						"__wbg_set_innerHTML_fb5a7e25198fc344": function(p0externref,p1i32,p2i32) {
/******/ 							if(wasmImportedFuncCache15 === undefined) wasmImportedFuncCache15 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache15["__wbg_set_innerHTML_fb5a7e25198fc344"](p0externref,p1i32,p2i32);
/******/ 						},
/******/ 						"__wbg_setAttribute_9bad76f39609daac": function(p0externref,p1i32,p2i32,p3i32,p4i32) {
/******/ 							if(wasmImportedFuncCache16 === undefined) wasmImportedFuncCache16 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache16["__wbg_setAttribute_9bad76f39609daac"](p0externref,p1i32,p2i32,p3i32,p4i32);
/******/ 						},
/******/ 						"__wbg_signal_4db5aa055bf9eb9a": function(p0externref) {
/******/ 							if(wasmImportedFuncCache17 === undefined) wasmImportedFuncCache17 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache17["__wbg_signal_4db5aa055bf9eb9a"](p0externref);
/******/ 						},
/******/ 						"__wbg_new_2531773dac38ebb3": function() {
/******/ 							if(wasmImportedFuncCache18 === undefined) wasmImportedFuncCache18 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache18["__wbg_new_2531773dac38ebb3"]();
/******/ 						},
/******/ 						"__wbg_abort_e7eb059f72f9ed0c": function(p0externref) {
/******/ 							if(wasmImportedFuncCache19 === undefined) wasmImportedFuncCache19 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache19["__wbg_abort_e7eb059f72f9ed0c"](p0externref);
/******/ 						},
/******/ 						"__wbg_abort_28ad55c5825b004d": function(p0externref,p1externref) {
/******/ 							if(wasmImportedFuncCache20 === undefined) wasmImportedFuncCache20 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache20["__wbg_abort_28ad55c5825b004d"](p0externref,p1externref);
/******/ 						},
/******/ 						"__wbg_new_9edf9838a2def39c": function() {
/******/ 							if(wasmImportedFuncCache21 === undefined) wasmImportedFuncCache21 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache21["__wbg_new_9edf9838a2def39c"]();
/******/ 						},
/******/ 						"__wbg_append_b577eb3a177bc0fa": function(p0externref,p1i32,p2i32,p3i32,p4i32) {
/******/ 							if(wasmImportedFuncCache22 === undefined) wasmImportedFuncCache22 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache22["__wbg_append_b577eb3a177bc0fa"](p0externref,p1i32,p2i32,p3i32,p4i32);
/******/ 						},
/******/ 						"__wbg_instanceof_HtmlInputElement_b8672abb32fe4ab7": function(p0externref) {
/******/ 							if(wasmImportedFuncCache23 === undefined) wasmImportedFuncCache23 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache23["__wbg_instanceof_HtmlInputElement_b8672abb32fe4ab7"](p0externref);
/******/ 						},
/******/ 						"__wbg_value_f470db44e5a60ad8": function(p0i32,p1externref) {
/******/ 							if(wasmImportedFuncCache24 === undefined) wasmImportedFuncCache24 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache24["__wbg_value_f470db44e5a60ad8"](p0i32,p1externref);
/******/ 						},
/******/ 						"__wbg_set_capture_baeff7292df46e64": function(p0externref,p1i32) {
/******/ 							if(wasmImportedFuncCache25 === undefined) wasmImportedFuncCache25 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache25["__wbg_set_capture_baeff7292df46e64"](p0externref,p1i32);
/******/ 						},
/******/ 						"__wbg_set_once_6faa794a6bcd7d25": function(p0externref,p1i32) {
/******/ 							if(wasmImportedFuncCache26 === undefined) wasmImportedFuncCache26 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache26["__wbg_set_once_6faa794a6bcd7d25"](p0externref,p1i32);
/******/ 						},
/******/ 						"__wbg_set_passive_c4c6f6a4ddd1a789": function(p0externref,p1i32) {
/******/ 							if(wasmImportedFuncCache27 === undefined) wasmImportedFuncCache27 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache27["__wbg_set_passive_c4c6f6a4ddd1a789"](p0externref,p1i32);
/******/ 						},
/******/ 						"__wbg_set_body_3c365989753d61f4": function(p0externref,p1externref) {
/******/ 							if(wasmImportedFuncCache28 === undefined) wasmImportedFuncCache28 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache28["__wbg_set_body_3c365989753d61f4"](p0externref,p1externref);
/******/ 						},
/******/ 						"__wbg_set_cache_2f9deb19b92b81e3": function(p0externref,p1i32) {
/******/ 							if(wasmImportedFuncCache29 === undefined) wasmImportedFuncCache29 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache29["__wbg_set_cache_2f9deb19b92b81e3"](p0externref,p1i32);
/******/ 						},
/******/ 						"__wbg_set_credentials_f621cd2d85c0c228": function(p0externref,p1i32) {
/******/ 							if(wasmImportedFuncCache30 === undefined) wasmImportedFuncCache30 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache30["__wbg_set_credentials_f621cd2d85c0c228"](p0externref,p1i32);
/******/ 						},
/******/ 						"__wbg_set_headers_6926da238cd32ee4": function(p0externref,p1externref) {
/******/ 							if(wasmImportedFuncCache31 === undefined) wasmImportedFuncCache31 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache31["__wbg_set_headers_6926da238cd32ee4"](p0externref,p1externref);
/******/ 						},
/******/ 						"__wbg_set_method_c02d8cbbe204ac2d": function(p0externref,p1i32,p2i32) {
/******/ 							if(wasmImportedFuncCache32 === undefined) wasmImportedFuncCache32 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache32["__wbg_set_method_c02d8cbbe204ac2d"](p0externref,p1i32,p2i32);
/******/ 						},
/******/ 						"__wbg_set_mode_52ef73cfa79639cb": function(p0externref,p1i32) {
/******/ 							if(wasmImportedFuncCache33 === undefined) wasmImportedFuncCache33 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache33["__wbg_set_mode_52ef73cfa79639cb"](p0externref,p1i32);
/******/ 						},
/******/ 						"__wbg_set_signal_dda2cf7ccb6bee0f": function(p0externref,p1externref) {
/******/ 							if(wasmImportedFuncCache34 === undefined) wasmImportedFuncCache34 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache34["__wbg_set_signal_dda2cf7ccb6bee0f"](p0externref,p1externref);
/******/ 						},
/******/ 						"__wbg_appendChild_aec7a8a4bd6cac61": function(p0externref,p1externref) {
/******/ 							if(wasmImportedFuncCache35 === undefined) wasmImportedFuncCache35 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache35["__wbg_appendChild_aec7a8a4bd6cac61"](p0externref,p1externref);
/******/ 						},
/******/ 						"__wbg_new_with_str_and_init_0ae7728b6ec367b1": function(p0i32,p1i32,p2externref) {
/******/ 							if(wasmImportedFuncCache36 === undefined) wasmImportedFuncCache36 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache36["__wbg_new_with_str_and_init_0ae7728b6ec367b1"](p0i32,p1i32,p2externref);
/******/ 						},
/******/ 						"__wbg_addEventListener_534b9f715f44517f": function(p0externref,p1i32,p2i32,p3externref,p4externref) {
/******/ 							if(wasmImportedFuncCache37 === undefined) wasmImportedFuncCache37 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache37["__wbg_addEventListener_534b9f715f44517f"](p0externref,p1i32,p2i32,p3externref,p4externref);
/******/ 						},
/******/ 						"__wbg_removeEventListener_7f805799d8d1e552": function(p0externref,p1i32,p2i32,p3externref,p4i32) {
/******/ 							if(wasmImportedFuncCache38 === undefined) wasmImportedFuncCache38 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache38["__wbg_removeEventListener_7f805799d8d1e552"](p0externref,p1i32,p2i32,p3externref,p4i32);
/******/ 						},
/******/ 						"__wbg_instanceof_Response_f4f3e87e07f3135c": function(p0externref) {
/******/ 							if(wasmImportedFuncCache39 === undefined) wasmImportedFuncCache39 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache39["__wbg_instanceof_Response_f4f3e87e07f3135c"](p0externref);
/******/ 						},
/******/ 						"__wbg_url_b36d2a5008eb056f": function(p0i32,p1externref) {
/******/ 							if(wasmImportedFuncCache40 === undefined) wasmImportedFuncCache40 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache40["__wbg_url_b36d2a5008eb056f"](p0i32,p1externref);
/******/ 						},
/******/ 						"__wbg_status_de7eed5a7a5bfd5d": function(p0externref) {
/******/ 							if(wasmImportedFuncCache41 === undefined) wasmImportedFuncCache41 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache41["__wbg_status_de7eed5a7a5bfd5d"](p0externref);
/******/ 						},
/******/ 						"__wbg_headers_b87d7eaba61c3278": function(p0externref) {
/******/ 							if(wasmImportedFuncCache42 === undefined) wasmImportedFuncCache42 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache42["__wbg_headers_b87d7eaba61c3278"](p0externref);
/******/ 						},
/******/ 						"__wbg_text_dc33c15c17bdfb52": function(p0externref) {
/******/ 							if(wasmImportedFuncCache43 === undefined) wasmImportedFuncCache43 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache43["__wbg_text_dc33c15c17bdfb52"](p0externref);
/******/ 						},
/******/ 						"__wbg_fetch_f8ba0e29a9d6de0d": function(p0externref,p1externref) {
/******/ 							if(wasmImportedFuncCache44 === undefined) wasmImportedFuncCache44 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache44["__wbg_fetch_f8ba0e29a9d6de0d"](p0externref,p1externref);
/******/ 						},
/******/ 						"__wbg_new_no_args_ee98eee5275000a4": function(p0i32,p1i32) {
/******/ 							if(wasmImportedFuncCache45 === undefined) wasmImportedFuncCache45 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache45["__wbg_new_no_args_ee98eee5275000a4"](p0i32,p1i32);
/******/ 						},
/******/ 						"__wbg_call_e762c39fa8ea36bf": function(p0externref,p1externref) {
/******/ 							if(wasmImportedFuncCache46 === undefined) wasmImportedFuncCache46 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache46["__wbg_call_e762c39fa8ea36bf"](p0externref,p1externref);
/******/ 						},
/******/ 						"__wbg_next_020810e0ae8ebcb0": function(p0externref) {
/******/ 							if(wasmImportedFuncCache47 === undefined) wasmImportedFuncCache47 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache47["__wbg_next_020810e0ae8ebcb0"](p0externref);
/******/ 						},
/******/ 						"__wbg_next_2c826fe5dfec6b6a": function(p0externref) {
/******/ 							if(wasmImportedFuncCache48 === undefined) wasmImportedFuncCache48 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache48["__wbg_next_2c826fe5dfec6b6a"](p0externref);
/******/ 						},
/******/ 						"__wbg_done_2042aa2670fb1db1": function(p0externref) {
/******/ 							if(wasmImportedFuncCache49 === undefined) wasmImportedFuncCache49 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache49["__wbg_done_2042aa2670fb1db1"](p0externref);
/******/ 						},
/******/ 						"__wbg_value_692627309814bb8c": function(p0externref) {
/******/ 							if(wasmImportedFuncCache50 === undefined) wasmImportedFuncCache50 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache50["__wbg_value_692627309814bb8c"](p0externref);
/******/ 						},
/******/ 						"__wbg_new_1acc0b6eea89d040": function() {
/******/ 							if(wasmImportedFuncCache51 === undefined) wasmImportedFuncCache51 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache51["__wbg_new_1acc0b6eea89d040"]();
/******/ 						},
/******/ 						"__wbg_iterator_e5822695327a3c39": function() {
/******/ 							if(wasmImportedFuncCache52 === undefined) wasmImportedFuncCache52 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache52["__wbg_iterator_e5822695327a3c39"]();
/******/ 						},
/******/ 						"__wbg_resolve_caf97c30b83f7053": function(p0externref) {
/******/ 							if(wasmImportedFuncCache53 === undefined) wasmImportedFuncCache53 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache53["__wbg_resolve_caf97c30b83f7053"](p0externref);
/******/ 						},
/******/ 						"__wbg_then_4f46f6544e6b4a28": function(p0externref,p1externref) {
/******/ 							if(wasmImportedFuncCache54 === undefined) wasmImportedFuncCache54 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache54["__wbg_then_4f46f6544e6b4a28"](p0externref,p1externref);
/******/ 						},
/******/ 						"__wbg_then_70d05cf780a18d77": function(p0externref,p1externref,p2externref) {
/******/ 							if(wasmImportedFuncCache55 === undefined) wasmImportedFuncCache55 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache55["__wbg_then_70d05cf780a18d77"](p0externref,p1externref,p2externref);
/******/ 						},
/******/ 						"__wbg_static_accessor_GLOBAL_THIS_8b530f326a9e48ac": function() {
/******/ 							if(wasmImportedFuncCache56 === undefined) wasmImportedFuncCache56 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache56["__wbg_static_accessor_GLOBAL_THIS_8b530f326a9e48ac"]();
/******/ 						},
/******/ 						"__wbg_static_accessor_SELF_6fdf4b64710cc91b": function() {
/******/ 							if(wasmImportedFuncCache57 === undefined) wasmImportedFuncCache57 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache57["__wbg_static_accessor_SELF_6fdf4b64710cc91b"]();
/******/ 						},
/******/ 						"__wbg_static_accessor_WINDOW_b45bfc5a37f6cfa2": function() {
/******/ 							if(wasmImportedFuncCache58 === undefined) wasmImportedFuncCache58 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache58["__wbg_static_accessor_WINDOW_b45bfc5a37f6cfa2"]();
/******/ 						},
/******/ 						"__wbg_static_accessor_GLOBAL_89e1d9ac6a1b250e": function() {
/******/ 							if(wasmImportedFuncCache59 === undefined) wasmImportedFuncCache59 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache59["__wbg_static_accessor_GLOBAL_89e1d9ac6a1b250e"]();
/******/ 						},
/******/ 						"__wbg_new_from_slice_92f4d78ca282a2d2": function(p0i32,p1i32) {
/******/ 							if(wasmImportedFuncCache60 === undefined) wasmImportedFuncCache60 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache60["__wbg_new_from_slice_92f4d78ca282a2d2"](p0i32,p1i32);
/******/ 						},
/******/ 						"__wbg_stringify_b5fb28f6465d9c3e": function(p0externref) {
/******/ 							if(wasmImportedFuncCache61 === undefined) wasmImportedFuncCache61 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache61["__wbg_stringify_b5fb28f6465d9c3e"](p0externref);
/******/ 						},
/******/ 						"__wbg_get_efcb449f58ec27c2": function(p0externref,p1externref) {
/******/ 							if(wasmImportedFuncCache62 === undefined) wasmImportedFuncCache62 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache62["__wbg_get_efcb449f58ec27c2"](p0externref,p1externref);
/******/ 						},
/******/ 						"__wbg_has_787fafc980c3ccdb": function(p0externref,p1externref) {
/******/ 							if(wasmImportedFuncCache63 === undefined) wasmImportedFuncCache63 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache63["__wbg_has_787fafc980c3ccdb"](p0externref,p1externref);
/******/ 						},
/******/ 						"__wbg__wbg_cb_unref_2454a539ea5790d9": function(p0externref) {
/******/ 							if(wasmImportedFuncCache64 === undefined) wasmImportedFuncCache64 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache64["__wbg__wbg_cb_unref_2454a539ea5790d9"](p0externref);
/******/ 						},
/******/ 						"__wbg___wbindgen_is_undefined_2d472862bd29a478": function(p0externref) {
/******/ 							if(wasmImportedFuncCache65 === undefined) wasmImportedFuncCache65 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache65["__wbg___wbindgen_is_undefined_2d472862bd29a478"](p0externref);
/******/ 						},
/******/ 						"__wbg___wbindgen_is_object_c818261d21f283a4": function(p0externref) {
/******/ 							if(wasmImportedFuncCache66 === undefined) wasmImportedFuncCache66 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache66["__wbg___wbindgen_is_object_c818261d21f283a4"](p0externref);
/******/ 						},
/******/ 						"__wbg___wbindgen_is_function_ee8a6c5833c90377": function(p0externref) {
/******/ 							if(wasmImportedFuncCache67 === undefined) wasmImportedFuncCache67 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache67["__wbg___wbindgen_is_function_ee8a6c5833c90377"](p0externref);
/******/ 						},
/******/ 						"__wbg___wbindgen_string_get_e4f06c90489ad01b": function(p0i32,p1externref) {
/******/ 							if(wasmImportedFuncCache68 === undefined) wasmImportedFuncCache68 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache68["__wbg___wbindgen_string_get_e4f06c90489ad01b"](p0i32,p1externref);
/******/ 						},
/******/ 						"__wbg___wbindgen_debug_string_df47ffb5e35e6763": function(p0i32,p1externref) {
/******/ 							if(wasmImportedFuncCache69 === undefined) wasmImportedFuncCache69 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache69["__wbg___wbindgen_debug_string_df47ffb5e35e6763"](p0i32,p1externref);
/******/ 						},
/******/ 						"__wbg___wbindgen_throw_b855445ff6a94295": function(p0i32,p1i32) {
/******/ 							if(wasmImportedFuncCache70 === undefined) wasmImportedFuncCache70 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache70["__wbg___wbindgen_throw_b855445ff6a94295"](p0i32,p1i32);
/******/ 						},
/******/ 						"__wbg___wbindgen_rethrow_ea38273dafc473e6": function(p0externref) {
/******/ 							if(wasmImportedFuncCache71 === undefined) wasmImportedFuncCache71 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache71["__wbg___wbindgen_rethrow_ea38273dafc473e6"](p0externref);
/******/ 						},
/******/ 						"__wbindgen_init_externref_table": function() {
/******/ 							if(wasmImportedFuncCache72 === undefined) wasmImportedFuncCache72 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache72["__wbindgen_init_externref_table"]();
/******/ 						},
/******/ 						"__wbindgen_cast_da7e93c27d321d1d": function(p0i32,p1i32) {
/******/ 							if(wasmImportedFuncCache73 === undefined) wasmImportedFuncCache73 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache73["__wbindgen_cast_da7e93c27d321d1d"](p0i32,p1i32);
/******/ 						},
/******/ 						"__wbindgen_cast_57c0269cd9046987": function(p0i32,p1i32) {
/******/ 							if(wasmImportedFuncCache74 === undefined) wasmImportedFuncCache74 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache74["__wbindgen_cast_57c0269cd9046987"](p0i32,p1i32);
/******/ 						},
/******/ 						"__wbindgen_cast_146fd35f906c65cd": function(p0i32,p1i32) {
/******/ 							if(wasmImportedFuncCache75 === undefined) wasmImportedFuncCache75 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache75["__wbindgen_cast_146fd35f906c65cd"](p0i32,p1i32);
/******/ 						},
/******/ 						"__wbindgen_cast_2241b6af4c4b2941": function(p0i32,p1i32) {
/******/ 							if(wasmImportedFuncCache76 === undefined) wasmImportedFuncCache76 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache76["__wbindgen_cast_2241b6af4c4b2941"](p0i32,p1i32);
/******/ 						}
/******/ 					}
/******/ 				};
/******/ 			},
/******/ 		};
/******/ 		
/******/ 		var wasmModuleMap = {
/******/ 			"pkg_index_js": [
/******/ 				"./pkg/index_bg.wasm"
/******/ 			]
/******/ 		};
/******/ 		
/******/ 		// object with all WebAssembly.instance exports
/******/ 		__webpack_require__.w = {};
/******/ 		
/******/ 		// Fetch + compile chunk loading for webassembly
/******/ 		__webpack_require__.f.wasm = function(chunkId, promises) {
/******/ 		
/******/ 			var wasmModules = wasmModuleMap[chunkId] || [];
/******/ 		
/******/ 			wasmModules.forEach(function(wasmModuleId, idx) {
/******/ 				var installedWasmModuleData = installedWasmModules[wasmModuleId];
/******/ 		
/******/ 				// a Promise means "currently loading" or "already loaded".
/******/ 				if(installedWasmModuleData)
/******/ 					promises.push(installedWasmModuleData);
/******/ 				else {
/******/ 					var importObject = wasmImportObjects[wasmModuleId]();
/******/ 					var req = fetch(__webpack_require__.p + "" + {"pkg_index_js":{"./pkg/index_bg.wasm":"e2631e57c364a48cc9bf"}}[chunkId][wasmModuleId] + ".module.wasm");
/******/ 					var promise;
/******/ 					if(importObject && typeof importObject.then === 'function' && typeof WebAssembly.compileStreaming === 'function') {
/******/ 						promise = Promise.all([WebAssembly.compileStreaming(req), importObject]).then(function(items) {
/******/ 							return WebAssembly.instantiate(items[0], items[1]);
/******/ 						});
/******/ 					} else if(typeof WebAssembly.instantiateStreaming === 'function') {
/******/ 						promise = WebAssembly.instantiateStreaming(req, importObject);
/******/ 					} else {
/******/ 						var bytesPromise = req.then(function(x) { return x.arrayBuffer(); });
/******/ 						promise = bytesPromise.then(function(bytes) {
/******/ 							return WebAssembly.instantiate(bytes, importObject);
/******/ 						});
/******/ 					}
/******/ 					promises.push(installedWasmModules[wasmModuleId] = promise.then(function(res) {
/******/ 						return __webpack_require__.w[wasmModuleId] = (res.instance || res).exports;
/******/ 					}));
/******/ 				}
/******/ 			});
/******/ 		};
/******/ 	})();
/******/ 	
/************************************************************************/
/******/ 	
/******/ 	// module cache are used so entry inlining is disabled
/******/ 	// startup
/******/ 	// Load entry module and return exports
/******/ 	var __webpack_exports__ = __webpack_require__("./index.js");
/******/ 	
/******/ })()
;