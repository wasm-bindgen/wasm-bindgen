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

/***/ "./index.js"
/*!******************!*\
  !*** ./index.js ***!
  \******************/
(__unused_webpack_module, __unused_webpack_exports, __webpack_require__) {

eval("{// For more comments about what's going on here, check out the `hello_world`\n// example\n__webpack_require__.e(/*! import() */ \"pkg_index_js\").then(() => (__webpack_require__(/*! ./pkg */ \"./pkg/index.js\")))\n    .catch(console.error);\n\n\n//# sourceURL=webpack:///./index.js?\n}");

/***/ }

/******/ 	});
/************************************************************************/
/******/ 	// The module cache
/******/ 	const __webpack_module_cache__ = {};
/******/ 	
/******/ 	// The require function
/******/ 	function __webpack_require__(moduleId) {
/******/ 		// Check if module is in cache
/******/ 		const cachedModule = __webpack_module_cache__[moduleId];
/******/ 		if (cachedModule !== undefined) {
/******/ 			return cachedModule.exports;
/******/ 		}
/******/ 		// Create a new module (and put it into the cache)
/******/ 		const module = __webpack_module_cache__[moduleId] = {
/******/ 			id: moduleId,
/******/ 			// no module.loaded needed
/******/ 			exports: {}
/******/ 		};
/******/ 	
/******/ 		// Execute the module function
/******/ 		if (!(moduleId in __webpack_modules__)) {
/******/ 			delete __webpack_module_cache__[moduleId];
/******/ 			const e = new Error("Cannot find module '" + moduleId + "'");
/******/ 			e.code = 'MODULE_NOT_FOUND';
/******/ 			throw e;
/******/ 		}
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
/******/ 	// define getter/value functions for harmony exports
/******/ 	__webpack_require__.d = (exports, definition) => {
/******/ 		for(var key in definition) {
/******/ 			if(__webpack_require__.o(definition, key) && !__webpack_require__.o(exports, key)) {
/******/ 				Object.defineProperty(exports, key, { enumerable: true, get: definition[key] });
/******/ 			}
/******/ 		}
/******/ 	};
/******/ 	
/******/ 	/* webpack/runtime/ensure chunk */
/******/ 	__webpack_require__.f = {};
/******/ 	// This file contains only the entry chunk.
/******/ 	// The chunk loading function for additional chunks
/******/ 	__webpack_require__.e = (chunkId) => {
/******/ 		return Promise.all(Object.keys(__webpack_require__.f).reduce((promises, key) => {
/******/ 			__webpack_require__.f[key](chunkId, promises);
/******/ 			return promises;
/******/ 		}, []));
/******/ 	};
/******/ 	
/******/ 	/* webpack/runtime/get javascript chunk filename */
/******/ 	// This function allow to reference async chunks
/******/ 	__webpack_require__.u = (chunkId) => (chunkId + ".index.js");
/******/ 	
/******/ 	/* webpack/runtime/global */
/******/ 	__webpack_require__.g = (function() {
/******/ 		if (typeof globalThis === 'object') return globalThis;
/******/ 		try {
/******/ 			return this || new Function('return this')();
/******/ 		} catch (e) {
/******/ 			if (typeof window === 'object') return window;
/******/ 		}
/******/ 	})();
/******/ 	
/******/ 	/* webpack/runtime/hasOwnProperty shorthand */
/******/ 	__webpack_require__.o = (obj, prop) => (Object.prototype.hasOwnProperty.call(obj, prop));
/******/ 	
/******/ 	/* webpack/runtime/load script */
/******/ 	(() => {
/******/ 		const inProgress = {};
/******/ 		// data-webpack is not used as build has no uniqueName
/******/ 		// loadScript function to load a script via script tag
/******/ 		__webpack_require__.l = (url, done, key, chunkId) => {
/******/ 			if(inProgress[url]) { inProgress[url].push(done); return; }
/******/ 			let script, needAttach;
/******/ 			if(key !== undefined) {
/******/ 				const scripts = document.getElementsByTagName("script");
/******/ 				for(var i = 0; i < scripts.length; i++) {
/******/ 					const s = scripts[i];
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
/******/ 			const onScriptComplete = (prev, event) => {
/******/ 				// avoid mem leaks in IE.
/******/ 				script.onerror = script.onload = null;
/******/ 				clearTimeout(timeout);
/******/ 				const doneFns = inProgress[url];
/******/ 				delete inProgress[url];
/******/ 				script.parentNode?.removeChild(script);
/******/ 				doneFns?.forEach((fn) => (fn(event)));
/******/ 				if(prev) return prev(event);
/******/ 			}
/******/ 			const timeout = setTimeout(onScriptComplete.bind(null, undefined, { type: 'timeout', target: script }), 120000);
/******/ 			script.onerror = onScriptComplete.bind(null, script.onerror);
/******/ 			script.onload = onScriptComplete.bind(null, script.onload);
/******/ 			needAttach && document.head.appendChild(script);
/******/ 		};
/******/ 	})();
/******/ 	
/******/ 	/* webpack/runtime/make namespace object */
/******/ 	// define __esModule on exports
/******/ 	__webpack_require__.r = (exports) => {
/******/ 		Object.defineProperty(exports, Symbol.toStringTag, { value: 'Module' });
/******/ 		Object.defineProperty(exports, '__esModule', { value: true });
/******/ 	};
/******/ 	
/******/ 	/* webpack/runtime/publicPath */
/******/ 	(() => {
/******/ 		let scriptUrl;
/******/ 		if (__webpack_require__.g.importScripts) scriptUrl = __webpack_require__.g.location + "";
/******/ 		const document = __webpack_require__.g.document;
/******/ 		if (!scriptUrl && document) {
/******/ 			if (document.currentScript?.tagName.toUpperCase() === 'SCRIPT')
/******/ 				scriptUrl = document.currentScript.src;
/******/ 			if (!scriptUrl) {
/******/ 				const scripts = document.getElementsByTagName("script");
/******/ 				if(scripts.length) {
/******/ 					let i = scripts.length - 1;
/******/ 					while (i > -1 && (!scriptUrl || !/^https?:/.test(scriptUrl))) scriptUrl = scripts[i--].src;
/******/ 				}
/******/ 			}
/******/ 		}
/******/ 		// When supporting browsers where an automatic publicPath is not supported you must specify an output.publicPath manually via configuration
/******/ 		// or pass an empty string ("") and set the __webpack_public_path__ variable from your code to use your own logic.
/******/ 		if (!scriptUrl) throw new Error("Automatic publicPath is not supported in this browser");
/******/ 		scriptUrl = scriptUrl.replace(/^blob:|[?#].*$/g, "").replace(/\/[^/]+$/, "/");
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
/******/ 		const installedChunks = {
/******/ 			"main": 0
/******/ 		};
/******/ 		
/******/ 		__webpack_require__.f.j = (chunkId, promises) => {
/******/ 				// JSONP chunk loading for javascript
/******/ 				let installedChunkData = __webpack_require__.o(installedChunks, chunkId) ? installedChunks[chunkId] : undefined;
/******/ 				if(installedChunkData !== 0) { // 0 means "already installed".
/******/ 		
/******/ 					// a Promise means "currently loading".
/******/ 					if(installedChunkData) {
/******/ 						promises.push(installedChunkData[2]);
/******/ 					} else {
/******/ 						if(true) { // all chunks have JS
/******/ 							// setup Promise in chunk cache
/******/ 							const promise = new Promise((resolve, reject) => (installedChunkData = installedChunks[chunkId] = [resolve, reject]));
/******/ 							promises.push(installedChunkData[2] = promise);
/******/ 		
/******/ 							// create error before stack unwound to get useful stacktrace later
/******/ 							const error = new Error();
/******/ 							const loadingEnded = (event) => {
/******/ 								if(__webpack_require__.o(installedChunks, chunkId)) {
/******/ 									installedChunkData = installedChunks[chunkId];
/******/ 									if(installedChunkData !== 0) installedChunks[chunkId] = undefined;
/******/ 									if(installedChunkData) {
/******/ 										const errorType = event && (event.type === 'load' ? 'missing' : event.type);
/******/ 										const realSrc = event && event.target && event.target.src;
/******/ 										error.message = 'Loading chunk ' + chunkId + ' failed.\n(' + errorType + ': ' + realSrc + ')';
/******/ 										error.name = 'ChunkLoadError';
/******/ 										error.type = errorType;
/******/ 										error.request = realSrc;
/******/ 										error.event = event;
/******/ 										installedChunkData[1](error);
/******/ 									}
/******/ 								}
/******/ 							};
/******/ 							__webpack_require__.l(__webpack_require__.p + __webpack_require__.u(chunkId), loadingEnded, "chunk-" + chunkId, chunkId);
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
/******/ 		const webpackJsonpCallback = (parentChunkLoadingFunction, data) => {
/******/ 			let [chunkIds, moreModules, runtime] = data;
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
/******/ 		const chunkLoadingGlobal = self["webpackChunk"] = self["webpackChunk"] || [];
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
/******/ 						"__wbg_then_c949d5a25a4e78f8": function(p0externref,p1externref,p2externref) {
/******/ 							if(wasmImportedFuncCache1 === undefined) wasmImportedFuncCache1 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache1["__wbg_then_c949d5a25a4e78f8"](p0externref,p1externref,p2externref);
/******/ 						},
/******/ 						"__wbg_get_6c896e0571ddae51": function(p0externref,p1i32) {
/******/ 							if(wasmImportedFuncCache2 === undefined) wasmImportedFuncCache2 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache2["__wbg_get_6c896e0571ddae51"](p0externref,p1i32);
/******/ 						},
/******/ 						"__wbg_call_187d372bd5fdd4aa": function(p0externref,p1externref,p2externref) {
/******/ 							if(wasmImportedFuncCache3 === undefined) wasmImportedFuncCache3 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache3["__wbg_call_187d372bd5fdd4aa"](p0externref,p1externref,p2externref);
/******/ 						},
/******/ 						"__wbg_next_f4aac29c42af995c": function(p0externref) {
/******/ 							if(wasmImportedFuncCache4 === undefined) wasmImportedFuncCache4 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache4["__wbg_next_f4aac29c42af995c"](p0externref);
/******/ 						},
/******/ 						"__wbg_done_b41a1d26cdb37fb6": function(p0externref) {
/******/ 							if(wasmImportedFuncCache5 === undefined) wasmImportedFuncCache5 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache5["__wbg_done_b41a1d26cdb37fb6"](p0externref);
/******/ 						},
/******/ 						"__wbg_value_f3c585ee8f5ba40c": function(p0externref) {
/******/ 							if(wasmImportedFuncCache6 === undefined) wasmImportedFuncCache6 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache6["__wbg_value_f3c585ee8f5ba40c"](p0externref);
/******/ 						},
/******/ 						"__wbg_setTimeout_b4a9096784635514": function(p0externref,p1i32) {
/******/ 							if(wasmImportedFuncCache7 === undefined) wasmImportedFuncCache7 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache7["__wbg_setTimeout_b4a9096784635514"](p0externref,p1i32);
/******/ 						},
/******/ 						"__wbg_clearTimeout_1daad5c6ee4fcd3b": function(p0externref) {
/******/ 							if(wasmImportedFuncCache8 === undefined) wasmImportedFuncCache8 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache8["__wbg_clearTimeout_1daad5c6ee4fcd3b"](p0externref);
/******/ 						},
/******/ 						"__wbg_fetch_7a37768ab67c5bd8": function(p0externref) {
/******/ 							if(wasmImportedFuncCache9 === undefined) wasmImportedFuncCache9 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache9["__wbg_fetch_7a37768ab67c5bd8"](p0externref);
/******/ 						},
/******/ 						"__wbg_document_9854e03c05fc8834": function(p0externref) {
/******/ 							if(wasmImportedFuncCache10 === undefined) wasmImportedFuncCache10 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache10["__wbg_document_9854e03c05fc8834"](p0externref);
/******/ 						},
/******/ 						"__wbg_instanceof_Window_82d71df4eddf88bc": function(p0externref) {
/******/ 							if(wasmImportedFuncCache11 === undefined) wasmImportedFuncCache11 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache11["__wbg_instanceof_Window_82d71df4eddf88bc"](p0externref);
/******/ 						},
/******/ 						"__wbg_createElement_74049073a11f9c31": function(p0externref,p1i32,p2i32) {
/******/ 							if(wasmImportedFuncCache12 === undefined) wasmImportedFuncCache12 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache12["__wbg_createElement_74049073a11f9c31"](p0externref,p1i32,p2i32);
/******/ 						},
/******/ 						"__wbg_getElementById_cc94972b404e4eaa": function(p0externref,p1i32,p2i32) {
/******/ 							if(wasmImportedFuncCache13 === undefined) wasmImportedFuncCache13 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache13["__wbg_getElementById_cc94972b404e4eaa"](p0externref,p1i32,p2i32);
/******/ 						},
/******/ 						"__wbg_body_e549239eaff082e1": function(p0externref) {
/******/ 							if(wasmImportedFuncCache14 === undefined) wasmImportedFuncCache14 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache14["__wbg_body_e549239eaff082e1"](p0externref);
/******/ 						},
/******/ 						"__wbg_setAttribute_9e7d603908f63705": function(p0externref,p1i32,p2i32,p3i32,p4i32) {
/******/ 							if(wasmImportedFuncCache15 === undefined) wasmImportedFuncCache15 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache15["__wbg_setAttribute_9e7d603908f63705"](p0externref,p1i32,p2i32,p3i32,p4i32);
/******/ 						},
/******/ 						"__wbg_set_innerHTML_7af59a832a09a074": function(p0externref,p1i32,p2i32) {
/******/ 							if(wasmImportedFuncCache16 === undefined) wasmImportedFuncCache16 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache16["__wbg_set_innerHTML_7af59a832a09a074"](p0externref,p1i32,p2i32);
/******/ 						},
/******/ 						"__wbg_set_id_60955e6018d03b26": function(p0externref,p1i32,p2i32) {
/******/ 							if(wasmImportedFuncCache17 === undefined) wasmImportedFuncCache17 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache17["__wbg_set_id_60955e6018d03b26"](p0externref,p1i32,p2i32);
/******/ 						},
/******/ 						"__wbg_set_className_541fce5cd31918aa": function(p0externref,p1i32,p2i32) {
/******/ 							if(wasmImportedFuncCache18 === undefined) wasmImportedFuncCache18 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache18["__wbg_set_className_541fce5cd31918aa"](p0externref,p1i32,p2i32);
/******/ 						},
/******/ 						"__wbg_value_05305a761dfa3e0e": function(p0i32,p1externref) {
/******/ 							if(wasmImportedFuncCache19 === undefined) wasmImportedFuncCache19 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache19["__wbg_value_05305a761dfa3e0e"](p0i32,p1externref);
/******/ 						},
/******/ 						"__wbg_instanceof_HtmlInputElement_5c33d1de59c09c49": function(p0externref) {
/******/ 							if(wasmImportedFuncCache20 === undefined) wasmImportedFuncCache20 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache20["__wbg_instanceof_HtmlInputElement_5c33d1de59c09c49"](p0externref);
/******/ 						},
/******/ 						"__wbg_appendChild_fb8c52e7dd8484ea": function(p0externref,p1externref) {
/******/ 							if(wasmImportedFuncCache21 === undefined) wasmImportedFuncCache21 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache21["__wbg_appendChild_fb8c52e7dd8484ea"](p0externref,p1externref);
/******/ 						},
/******/ 						"__wbg_new_with_str_and_init_4618dee4e950224f": function(p0i32,p1i32,p2externref) {
/******/ 							if(wasmImportedFuncCache22 === undefined) wasmImportedFuncCache22 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache22["__wbg_new_with_str_and_init_4618dee4e950224f"](p0i32,p1i32,p2externref);
/******/ 						},
/******/ 						"__wbg_text_ff3f476b3d6b1246": function(p0externref) {
/******/ 							if(wasmImportedFuncCache23 === undefined) wasmImportedFuncCache23 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache23["__wbg_text_ff3f476b3d6b1246"](p0externref);
/******/ 						},
/******/ 						"__wbg_new_83ca405eae412a62": function() {
/******/ 							if(wasmImportedFuncCache24 === undefined) wasmImportedFuncCache24 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache24["__wbg_new_83ca405eae412a62"]();
/******/ 						},
/******/ 						"__wbg_signal_e5a49a2d9c65b7f6": function(p0externref) {
/******/ 							if(wasmImportedFuncCache25 === undefined) wasmImportedFuncCache25 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache25["__wbg_signal_e5a49a2d9c65b7f6"](p0externref);
/******/ 						},
/******/ 						"__wbg_url_8e28edeab4cd6bed": function(p0i32,p1externref) {
/******/ 							if(wasmImportedFuncCache26 === undefined) wasmImportedFuncCache26 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache26["__wbg_url_8e28edeab4cd6bed"](p0i32,p1externref);
/******/ 						},
/******/ 						"__wbg_new_2e41357094e99294": function() {
/******/ 							if(wasmImportedFuncCache27 === undefined) wasmImportedFuncCache27 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache27["__wbg_new_2e41357094e99294"]();
/******/ 						},
/******/ 						"__wbg_addEventListener_8d3235978b9c69a8": function(p0externref,p1i32,p2i32,p3externref,p4externref) {
/******/ 							if(wasmImportedFuncCache28 === undefined) wasmImportedFuncCache28 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache28["__wbg_addEventListener_8d3235978b9c69a8"](p0externref,p1i32,p2i32,p3externref,p4externref);
/******/ 						},
/******/ 						"__wbg_abort_f78b99d80248bacb": function(p0externref) {
/******/ 							if(wasmImportedFuncCache29 === undefined) wasmImportedFuncCache29 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache29["__wbg_abort_f78b99d80248bacb"](p0externref);
/******/ 						},
/******/ 						"__wbg_status_ce0a98d3c57125f3": function(p0externref) {
/******/ 							if(wasmImportedFuncCache30 === undefined) wasmImportedFuncCache30 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache30["__wbg_status_ce0a98d3c57125f3"](p0externref);
/******/ 						},
/******/ 						"__wbg_abort_334e7f918b7c5047": function(p0externref,p1externref) {
/******/ 							if(wasmImportedFuncCache31 === undefined) wasmImportedFuncCache31 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache31["__wbg_abort_334e7f918b7c5047"](p0externref,p1externref);
/******/ 						},
/******/ 						"__wbg_append_1938ed58fb5613b8": function(p0externref,p1i32,p2i32,p3i32,p4i32) {
/******/ 							if(wasmImportedFuncCache32 === undefined) wasmImportedFuncCache32 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache32["__wbg_append_1938ed58fb5613b8"](p0externref,p1i32,p2i32,p3i32,p4i32);
/******/ 						},
/******/ 						"__wbg_headers_eba93595f8944c2f": function(p0externref) {
/******/ 							if(wasmImportedFuncCache33 === undefined) wasmImportedFuncCache33 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache33["__wbg_headers_eba93595f8944c2f"](p0externref);
/******/ 						},
/******/ 						"__wbg_removeEventListener_261678bfcbd90907": function(p0externref,p1i32,p2i32,p3externref,p4i32) {
/******/ 							if(wasmImportedFuncCache34 === undefined) wasmImportedFuncCache34 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache34["__wbg_removeEventListener_261678bfcbd90907"](p0externref,p1i32,p2i32,p3externref,p4i32);
/******/ 						},
/******/ 						"__wbg_entries_b55ec7e0dc443355": function(p0externref) {
/******/ 							if(wasmImportedFuncCache35 === undefined) wasmImportedFuncCache35 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache35["__wbg_entries_b55ec7e0dc443355"](p0externref);
/******/ 						},
/******/ 						"__wbg_instanceof_Response_b8758567269c30b2": function(p0externref) {
/******/ 							if(wasmImportedFuncCache36 === undefined) wasmImportedFuncCache36 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache36["__wbg_instanceof_Response_b8758567269c30b2"](p0externref);
/******/ 						},
/******/ 						"__wbg_set_mode_74e772635acb96ec": function(p0externref,p1i32) {
/******/ 							if(wasmImportedFuncCache37 === undefined) wasmImportedFuncCache37 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache37["__wbg_set_mode_74e772635acb96ec"](p0externref,p1i32);
/******/ 						},
/******/ 						"__wbg_set_referrer_efd0fb956d6b70b2": function(p0externref,p1i32,p2i32) {
/******/ 							if(wasmImportedFuncCache38 === undefined) wasmImportedFuncCache38 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache38["__wbg_set_referrer_efd0fb956d6b70b2"](p0externref,p1i32,p2i32);
/******/ 						},
/******/ 						"__wbg_set_referrer_policy_465382f52c1a03d3": function(p0externref,p1i32) {
/******/ 							if(wasmImportedFuncCache39 === undefined) wasmImportedFuncCache39 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache39["__wbg_set_referrer_policy_465382f52c1a03d3"](p0externref,p1i32);
/******/ 						},
/******/ 						"__wbg_set_signal_2dbd377604627f26": function(p0externref,p1externref) {
/******/ 							if(wasmImportedFuncCache40 === undefined) wasmImportedFuncCache40 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache40["__wbg_set_signal_2dbd377604627f26"](p0externref,p1externref);
/******/ 						},
/******/ 						"__wbg_set_body_1fb0f1008bfc7df6": function(p0externref,p1externref) {
/******/ 							if(wasmImportedFuncCache41 === undefined) wasmImportedFuncCache41 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache41["__wbg_set_body_1fb0f1008bfc7df6"](p0externref,p1externref);
/******/ 						},
/******/ 						"__wbg_set_capture_5beff5b8a7562085": function(p0externref,p1i32) {
/******/ 							if(wasmImportedFuncCache42 === undefined) wasmImportedFuncCache42 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache42["__wbg_set_capture_5beff5b8a7562085"](p0externref,p1i32);
/******/ 						},
/******/ 						"__wbg_set_once_8c45bbbfb71d1b34": function(p0externref,p1i32) {
/******/ 							if(wasmImportedFuncCache43 === undefined) wasmImportedFuncCache43 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache43["__wbg_set_once_8c45bbbfb71d1b34"](p0externref,p1i32);
/******/ 						},
/******/ 						"__wbg_set_passive_31e9f895d2ac0046": function(p0externref,p1i32) {
/******/ 							if(wasmImportedFuncCache44 === undefined) wasmImportedFuncCache44 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache44["__wbg_set_passive_31e9f895d2ac0046"](p0externref,p1i32);
/******/ 						},
/******/ 						"__wbg_set_cache_7507c4cc5938ccc7": function(p0externref,p1i32) {
/******/ 							if(wasmImportedFuncCache45 === undefined) wasmImportedFuncCache45 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache45["__wbg_set_cache_7507c4cc5938ccc7"](p0externref,p1i32);
/******/ 						},
/******/ 						"__wbg_set_credentials_e0dcce31ca532b7d": function(p0externref,p1i32) {
/******/ 							if(wasmImportedFuncCache46 === undefined) wasmImportedFuncCache46 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache46["__wbg_set_credentials_e0dcce31ca532b7d"](p0externref,p1i32);
/******/ 						},
/******/ 						"__wbg_set_headers_40ad33dcf016613f": function(p0externref,p1externref) {
/******/ 							if(wasmImportedFuncCache47 === undefined) wasmImportedFuncCache47 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache47["__wbg_set_headers_40ad33dcf016613f"](p0externref,p1externref);
/******/ 						},
/******/ 						"__wbg_set_method_dcb32343247ec427": function(p0externref,p1i32,p2i32) {
/******/ 							if(wasmImportedFuncCache48 === undefined) wasmImportedFuncCache48 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache48["__wbg_set_method_dcb32343247ec427"](p0externref,p1i32,p2i32);
/******/ 						},
/******/ 						"__wbg_fetch_efc64baf325b084d": function(p0externref,p1externref) {
/******/ 							if(wasmImportedFuncCache49 === undefined) wasmImportedFuncCache49 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache49["__wbg_fetch_efc64baf325b084d"](p0externref,p1externref);
/******/ 						},
/******/ 						"__wbg_then_e71170d78fcf8954": function(p0externref,p1externref) {
/******/ 							if(wasmImportedFuncCache50 === undefined) wasmImportedFuncCache50 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache50["__wbg_then_e71170d78fcf8954"](p0externref,p1externref);
/******/ 						},
/******/ 						"__wbg_resolve_0076e10020304ede": function(p0externref) {
/******/ 							if(wasmImportedFuncCache51 === undefined) wasmImportedFuncCache51 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache51["__wbg_resolve_0076e10020304ede"](p0externref);
/******/ 						},
/******/ 						"__wbg_isArray_e15a2ff68ffdbef2": function(p0externref) {
/******/ 							if(wasmImportedFuncCache52 === undefined) wasmImportedFuncCache52 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache52["__wbg_isArray_e15a2ff68ffdbef2"](p0externref);
/******/ 						},
/******/ 						"__wbg_new_617a8cdb8bb1130e": function() {
/******/ 							if(wasmImportedFuncCache53 === undefined) wasmImportedFuncCache53 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache53["__wbg_new_617a8cdb8bb1130e"]();
/******/ 						},
/******/ 						"__wbg_new_from_slice_9a868026ffa4208a": function(p0i32,p1i32) {
/******/ 							if(wasmImportedFuncCache54 === undefined) wasmImportedFuncCache54 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache54["__wbg_new_from_slice_9a868026ffa4208a"](p0i32,p1i32);
/******/ 						},
/******/ 						"__wbg_static_accessor_GLOBAL_THIS_10fb7dc1ae063179": function() {
/******/ 							if(wasmImportedFuncCache55 === undefined) wasmImportedFuncCache55 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache55["__wbg_static_accessor_GLOBAL_THIS_10fb7dc1ae063179"]();
/******/ 						},
/******/ 						"__wbg_static_accessor_SELF_0b583911f537483a": function() {
/******/ 							if(wasmImportedFuncCache56 === undefined) wasmImportedFuncCache56 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache56["__wbg_static_accessor_SELF_0b583911f537483a"]();
/******/ 						},
/******/ 						"__wbg_static_accessor_GLOBAL_266715b9d96ba635": function() {
/******/ 							if(wasmImportedFuncCache57 === undefined) wasmImportedFuncCache57 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache57["__wbg_static_accessor_GLOBAL_266715b9d96ba635"]();
/******/ 						},
/******/ 						"__wbg_static_accessor_WINDOW_d7f903d1508cbdc4": function() {
/******/ 							if(wasmImportedFuncCache58 === undefined) wasmImportedFuncCache58 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache58["__wbg_static_accessor_WINDOW_d7f903d1508cbdc4"]();
/******/ 						},
/******/ 						"__wbg_queueMicrotask_9833f9a49df95a49": function(p0externref) {
/******/ 							if(wasmImportedFuncCache59 === undefined) wasmImportedFuncCache59 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache59["__wbg_queueMicrotask_9833f9a49df95a49"](p0externref);
/******/ 						},
/******/ 						"__wbg_has_5d6706e5209576c1": function(p0externref,p1externref) {
/******/ 							if(wasmImportedFuncCache60 === undefined) wasmImportedFuncCache60 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache60["__wbg_has_5d6706e5209576c1"](p0externref,p1externref);
/******/ 						},
/******/ 						"__wbg_queueMicrotask_a72f977e97f23c5f": function(p0externref) {
/******/ 							if(wasmImportedFuncCache61 === undefined) wasmImportedFuncCache61 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache61["__wbg_queueMicrotask_a72f977e97f23c5f"](p0externref);
/******/ 						},
/******/ 						"__wbg_run_6e1f787af721bfe6": function(p0externref,p1i32,p2i32) {
/******/ 							if(wasmImportedFuncCache62 === undefined) wasmImportedFuncCache62 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache62["__wbg_run_6e1f787af721bfe6"](p0externref,p1i32,p2i32);
/******/ 						},
/******/ 						"__wbg_static_accessor_CREATE_TASK_7e310091636beacc": function() {
/******/ 							if(wasmImportedFuncCache63 === undefined) wasmImportedFuncCache63 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache63["__wbg_static_accessor_CREATE_TASK_7e310091636beacc"]();
/******/ 						},
/******/ 						"__wbg___wbindgen_throw_41e9ee4f547fc59a": function(p0i32,p1i32) {
/******/ 							if(wasmImportedFuncCache64 === undefined) wasmImportedFuncCache64 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache64["__wbg___wbindgen_throw_41e9ee4f547fc59a"](p0i32,p1i32);
/******/ 						},
/******/ 						"__wbg___wbindgen_rethrow_cb2e88c6b2a16733": function(p0externref) {
/******/ 							if(wasmImportedFuncCache65 === undefined) wasmImportedFuncCache65 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache65["__wbg___wbindgen_rethrow_cb2e88c6b2a16733"](p0externref);
/******/ 						},
/******/ 						"__wbg___wbindgen_string_get_0380ccaa2f57f0d9": function(p0i32,p1externref) {
/******/ 							if(wasmImportedFuncCache66 === undefined) wasmImportedFuncCache66 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache66["__wbg___wbindgen_string_get_0380ccaa2f57f0d9"](p0i32,p1externref);
/******/ 						},
/******/ 						"__wbg___wbindgen_is_function_1f9d30630b8b1d3d": function(p0externref) {
/******/ 							if(wasmImportedFuncCache67 === undefined) wasmImportedFuncCache67 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache67["__wbg___wbindgen_is_function_1f9d30630b8b1d3d"](p0externref);
/******/ 						},
/******/ 						"__wbg___wbindgen_debug_string_4687d8d8c2017d52": function(p0i32,p1externref) {
/******/ 							if(wasmImportedFuncCache68 === undefined) wasmImportedFuncCache68 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache68["__wbg___wbindgen_debug_string_4687d8d8c2017d52"](p0i32,p1externref);
/******/ 						},
/******/ 						"__wbg___wbindgen_is_undefined_8865fb403f8fe9d8": function(p0externref) {
/******/ 							if(wasmImportedFuncCache69 === undefined) wasmImportedFuncCache69 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache69["__wbg___wbindgen_is_undefined_8865fb403f8fe9d8"](p0externref);
/******/ 						},
/******/ 						"__wbg__wbg_cb_unref_dcc1a90847f04c41": function(p0externref) {
/******/ 							if(wasmImportedFuncCache70 === undefined) wasmImportedFuncCache70 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache70["__wbg__wbg_cb_unref_dcc1a90847f04c41"](p0externref);
/******/ 						},
/******/ 						"__wbindgen_init_externref_table": function() {
/******/ 							if(wasmImportedFuncCache71 === undefined) wasmImportedFuncCache71 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache71["__wbindgen_init_externref_table"]();
/******/ 						},
/******/ 						"__wbindgen_generic_0000000000000001": function(p0i32,p1i32) {
/******/ 							if(wasmImportedFuncCache72 === undefined) wasmImportedFuncCache72 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache72["__wbindgen_generic_0000000000000001"](p0i32,p1i32);
/******/ 						},
/******/ 						"__wbindgen_generic_0000000000000002": function(p0i32,p1i32) {
/******/ 							if(wasmImportedFuncCache73 === undefined) wasmImportedFuncCache73 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache73["__wbindgen_generic_0000000000000002"](p0i32,p1i32);
/******/ 						},
/******/ 						"__wbindgen_generic_0000000000000003": function(p0i32,p1i32) {
/******/ 							if(wasmImportedFuncCache74 === undefined) wasmImportedFuncCache74 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache74["__wbindgen_generic_0000000000000003"](p0i32,p1i32);
/******/ 						},
/******/ 						"__wbindgen_generic_0000000000000004": function(p0i32,p1i32) {
/******/ 							if(wasmImportedFuncCache75 === undefined) wasmImportedFuncCache75 = __webpack_require__.c["./pkg/index_bg.js"].exports;
/******/ 							return wasmImportedFuncCache75["__wbindgen_generic_0000000000000004"](p0i32,p1i32);
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
/******/ 					var req = fetch(__webpack_require__.p + "" + {"pkg_index_js":{"./pkg/index_bg.wasm":"665cce77ad259a059ab3"}}[chunkId][wasmModuleId] + ".module.wasm");
/******/ 					var promise;
/******/ 					if(importObject && typeof importObject.then === 'function' && typeof WebAssembly.compileStreaming === 'function') {
/******/ 						promise = req.then(function(res) {
/******/ 							return Promise.all([WebAssembly.compileStreaming(res), importObject]).then(function(items) {
/******/ 								return WebAssembly.instantiate(items[0], items[1]);
/******/ 							}, function(e) {
/******/ 								if(res.headers.get("Content-Type") !== "application/wasm") {
/******/ 									console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);
/******/ 									return Promise.all([res.arrayBuffer().then(function(bytes) { return WebAssembly.compile(bytes); }), importObject]).then(function(items) {
/******/ 										return WebAssembly.instantiate(items[0], items[1]);
/******/ 									});
/******/ 								}
/******/ 								throw e;
/******/ 							});
/******/ 						});
/******/ 					} else if(typeof WebAssembly.instantiateStreaming === 'function') {
/******/ 						promise = req.then(function(res) {
/******/ 							return WebAssembly.instantiateStreaming(res, importObject).then(undefined, function(e) {
/******/ 								if(res.headers.get("Content-Type") !== "application/wasm") {
/******/ 									console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);
/******/ 									return res.arrayBuffer().then(function(bytes) {
/******/ 										return WebAssembly.instantiate(bytes, importObject);
/******/ 									});
/******/ 								}
/******/ 								throw e;
/******/ 							});
/******/ 						});
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
/******/ 	let __webpack_exports__ = __webpack_require__("./index.js");
/******/ 	
/******/ })()
;