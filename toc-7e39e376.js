// Populate the sidebar
//
// This is a script, and not included directly in the page, to control the total size of the book.
// The TOC contains an entry for each page, so if each page includes a copy of the TOC,
// the total size of the page becomes O(n**2).
class MDBookSidebarScrollbox extends HTMLElement {
    constructor() {
        super();
    }
    connectedCallback() {
        this.innerHTML = '<ol class="chapter"><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="introduction.html">Introduction</a></span></li><li class="chapter-item expanded "><li class="spacer"></li></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="examples/index.html"><strong aria-hidden="true">1.</strong> Examples</a></span><ol class="section"><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="examples/hello-world.html"><strong aria-hidden="true">1.1.</strong> Hello, World!</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="examples/console-log.html"><strong aria-hidden="true">1.2.</strong> Using console.log</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="examples/add.html"><strong aria-hidden="true">1.3.</strong> Small Wasm files</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="examples/without-a-bundler.html"><strong aria-hidden="true">1.4.</strong> Without a Bundler</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="examples/synchronous-instantiation.html"><strong aria-hidden="true">1.5.</strong> Synchronous Instantiation</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="examples/import-js.html"><strong aria-hidden="true">1.6.</strong> Importing functions from JS</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="examples/char.html"><strong aria-hidden="true">1.7.</strong> Working with char</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="examples/wasm-in-wasm.html"><strong aria-hidden="true">1.8.</strong> js-sys: WebAssembly in WebAssembly</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="examples/dom.html"><strong aria-hidden="true">1.9.</strong> web-sys: DOM hello world</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="examples/closures.html"><strong aria-hidden="true">1.10.</strong> web-sys: Closures</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="examples/performance.html"><strong aria-hidden="true">1.11.</strong> web-sys: performance.now</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="examples/fetch.html"><strong aria-hidden="true">1.12.</strong> web-sys: using fetch</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="examples/weather_report.html"><strong aria-hidden="true">1.13.</strong> web-sys: Weather report</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="examples/2d-canvas.html"><strong aria-hidden="true">1.14.</strong> web-sys: canvas hello world</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="examples/julia.html"><strong aria-hidden="true">1.15.</strong> web-sys: canvas Julia set</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="examples/web-audio.html"><strong aria-hidden="true">1.16.</strong> web-sys: WebAudio</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="examples/webgl.html"><strong aria-hidden="true">1.17.</strong> web-sys: WebGL</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="examples/websockets.html"><strong aria-hidden="true">1.18.</strong> web-sys: WebSockets</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="examples/webrtc_datachannel.html"><strong aria-hidden="true">1.19.</strong> web-sys: WebRTC DataChannel</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="examples/request-animation-frame.html"><strong aria-hidden="true">1.20.</strong> web-sys: requestAnimationFrame</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="examples/paint.html"><strong aria-hidden="true">1.21.</strong> web-sys: A Simple Paint Program</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="examples/wasm-in-web-worker.html"><strong aria-hidden="true">1.22.</strong> web-sys: Wasm in Web Worker</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="examples/raytrace.html"><strong aria-hidden="true">1.23.</strong> Parallel Raytracing</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="examples/wasm-audio-worklet.html"><strong aria-hidden="true">1.24.</strong> Wasm Audio Worklet</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="examples/todomvc.html"><strong aria-hidden="true">1.25.</strong> web-sys: A TODO MVC App</a></span></li></ol><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="reference/index.html"><strong aria-hidden="true">2.</strong> Reference</a></span><ol class="section"><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="reference/working-with-generics.html"><strong aria-hidden="true">2.1.</strong> Working with Generics</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="reference/deployment.html"><strong aria-hidden="true">2.2.</strong> Deployment</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="reference/community-projects.html"><strong aria-hidden="true">2.3.</strong> Community Projects</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="reference/js-snippets.html"><strong aria-hidden="true">2.4.</strong> JS snippets</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="reference/static-js-objects.html"><strong aria-hidden="true">2.5.</strong> Static JS Objects</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="reference/passing-rust-closures-to-js.html"><strong aria-hidden="true">2.6.</strong> Passing Rust Closures to JS</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="reference/receiving-js-closures-in-rust.html"><strong aria-hidden="true">2.7.</strong> Receiving JS Closures in Rust</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="reference/js-promises-and-rust-futures.html"><strong aria-hidden="true">2.8.</strong> Promises and Futures</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="reference/iterating-over-js-values.html"><strong aria-hidden="true">2.9.</strong> Iterating over JS Values</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="reference/arbitrary-data-with-serde.html"><strong aria-hidden="true">2.10.</strong> Arbitrary Data with Serde</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="reference/accessing-properties-of-untyped-js-values.html"><strong aria-hidden="true">2.11.</strong> Accessing Properties of Untyped JS Values</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="reference/working-with-duck-typed-interfaces.html"><strong aria-hidden="true">2.12.</strong> Working with Duck-Typed Interfaces</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="reference/cli.html"><strong aria-hidden="true">2.13.</strong> Command Line Interface</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="reference/optimize-size.html"><strong aria-hidden="true">2.14.</strong> Optimizing for Size</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="reference/debug-info.html"><strong aria-hidden="true">2.15.</strong> Debug information</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="reference/rust-targets.html"><strong aria-hidden="true">2.16.</strong> Supported Rust Targets</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="reference/browser-support.html"><strong aria-hidden="true">2.17.</strong> Supported Browsers</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="reference/weak-references.html"><strong aria-hidden="true">2.18.</strong> Support for Weak References</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="reference/reference-types.html"><strong aria-hidden="true">2.19.</strong> Support for Reference Types</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="reference/catch-unwind.html"><strong aria-hidden="true">2.20.</strong> Catching Panics</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="reference/handling-aborts.html"><strong aria-hidden="true">2.21.</strong> Handling Aborts</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="reference/types.html"><strong aria-hidden="true">2.22.</strong> Supported Types</a></span><ol class="section"><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="reference/types/imported-js-types.html"><strong aria-hidden="true">2.22.1.</strong> Imported JavaScript Types</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="reference/types/exported-rust-types.html"><strong aria-hidden="true">2.22.2.</strong> Exported Rust Types</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="reference/types/jsvalue.html"><strong aria-hidden="true">2.22.3.</strong> JsValue</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="reference/types/js-sys.html"><strong aria-hidden="true">2.22.4.</strong> js-sys</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="reference/types/boxed-slices.html"><strong aria-hidden="true">2.22.5.</strong> Box&lt;[T]&gt; and Vec&lt;T&gt;</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="reference/types/pointers.html"><strong aria-hidden="true">2.22.6.</strong> *const T and *mut T</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="reference/types/non-null.html"><strong aria-hidden="true">2.22.7.</strong> NonNull&lt;T&gt;</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="reference/types/numbers.html"><strong aria-hidden="true">2.22.8.</strong> Numbers</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="reference/types/bool.html"><strong aria-hidden="true">2.22.9.</strong> bool</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="reference/types/char.html"><strong aria-hidden="true">2.22.10.</strong> char</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="reference/types/str.html"><strong aria-hidden="true">2.22.11.</strong> str</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="reference/types/string.html"><strong aria-hidden="true">2.22.12.</strong> String</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="reference/types/number-slices.html"><strong aria-hidden="true">2.22.13.</strong> Number Slices</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="reference/types/boxed-number-slices.html"><strong aria-hidden="true">2.22.14.</strong> Boxed Number Slices</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="reference/types/result.html"><strong aria-hidden="true">2.22.15.</strong> Result&lt;T, E&gt;</a></span></li></ol><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="reference/attributes/index.html"><strong aria-hidden="true">2.23.</strong> #[wasm_bindgen] Attributes</a></span><ol class="section"><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="reference/attributes/on-js-imports/index.html"><strong aria-hidden="true">2.23.1.</strong> On JavaScript Imports</a></span><ol class="section"><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="reference/attributes/on-js-imports/catch.html"><strong aria-hidden="true">2.23.1.1.</strong> catch</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="reference/attributes/on-js-imports/constructor.html"><strong aria-hidden="true">2.23.1.2.</strong> constructor</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="reference/attributes/on-js-imports/extends.html"><strong aria-hidden="true">2.23.1.3.</strong> extends</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="reference/attributes/on-js-imports/getter-and-setter.html"><strong aria-hidden="true">2.23.1.4.</strong> getter and setter</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="reference/attributes/on-js-imports/final.html"><strong aria-hidden="true">2.23.1.5.</strong> final</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="reference/attributes/on-js-imports/indexing-getter-setter-deleter.html"><strong aria-hidden="true">2.23.1.6.</strong> indexing_getter, indexing_setter, and indexing_deleter</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="reference/attributes/on-js-imports/js_class.html"><strong aria-hidden="true">2.23.1.7.</strong> js_class = &quot;Blah&quot;</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="reference/attributes/on-js-imports/js_name.html"><strong aria-hidden="true">2.23.1.8.</strong> js_name</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="reference/attributes/on-js-imports/js_namespace.html"><strong aria-hidden="true">2.23.1.9.</strong> js_namespace</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="reference/attributes/on-js-imports/method.html"><strong aria-hidden="true">2.23.1.10.</strong> method</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="reference/attributes/on-js-imports/module.html"><strong aria-hidden="true">2.23.1.11.</strong> module = &quot;blah&quot;</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="reference/attributes/on-js-imports/raw_module.html"><strong aria-hidden="true">2.23.1.12.</strong> raw_module = &quot;blah&quot;</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="reference/attributes/on-js-imports/no_deref.html"><strong aria-hidden="true">2.23.1.13.</strong> no_deref</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="reference/attributes/on-js-imports/no_upcast.html"><strong aria-hidden="true">2.23.1.14.</strong> no_upcast</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="reference/attributes/on-js-imports/no_promising.html"><strong aria-hidden="true">2.23.1.15.</strong> no_promising</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="reference/attributes/on-js-imports/reexport.html"><strong aria-hidden="true">2.23.1.16.</strong> reexport</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="reference/attributes/on-js-imports/static_method_of.html"><strong aria-hidden="true">2.23.1.17.</strong> static_method_of = Blah</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="reference/attributes/on-js-imports/structural.html"><strong aria-hidden="true">2.23.1.18.</strong> structural</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="reference/attributes/on-js-imports/typescript_type.html"><strong aria-hidden="true">2.23.1.19.</strong> typescript_type</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="reference/attributes/on-js-imports/variadic.html"><strong aria-hidden="true">2.23.1.20.</strong> variadic</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="reference/attributes/on-js-imports/vendor_prefix.html"><strong aria-hidden="true">2.23.1.21.</strong> vendor_prefix</a></span></li></ol><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="reference/attributes/on-rust-exports/index.html"><strong aria-hidden="true">2.23.2.</strong> On Rust Exports</a></span><ol class="section"><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="reference/attributes/on-rust-exports/constructor.html"><strong aria-hidden="true">2.23.2.1.</strong> constructor</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="reference/attributes/on-rust-exports/js_name.html"><strong aria-hidden="true">2.23.2.2.</strong> js_name = Blah</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="reference/attributes/on-rust-exports/js_namespace.html"><strong aria-hidden="true">2.23.2.3.</strong> js_namespace</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="reference/attributes/on-rust-exports/js_class.html"><strong aria-hidden="true">2.23.2.4.</strong> js_class = Blah</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="reference/attributes/on-rust-exports/readonly.html"><strong aria-hidden="true">2.23.2.5.</strong> readonly</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="reference/attributes/on-rust-exports/skip.html"><strong aria-hidden="true">2.23.2.6.</strong> skip</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="reference/attributes/on-rust-exports/skip_jsdoc.html"><strong aria-hidden="true">2.23.2.7.</strong> skip_jsdoc</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="reference/attributes/on-rust-exports/start.html"><strong aria-hidden="true">2.23.2.8.</strong> start</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="reference/attributes/on-rust-exports/main.html"><strong aria-hidden="true">2.23.2.9.</strong> main</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="reference/attributes/on-rust-exports/this.html"><strong aria-hidden="true">2.23.2.10.</strong> this</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="reference/attributes/on-rust-exports/typescript_custom_section.html"><strong aria-hidden="true">2.23.2.11.</strong> typescript_custom_section</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="reference/attributes/on-rust-exports/getter-and-setter.html"><strong aria-hidden="true">2.23.2.12.</strong> getter and setter</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="reference/attributes/on-rust-exports/inspectable.html"><strong aria-hidden="true">2.23.2.13.</strong> inspectable</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="reference/attributes/on-rust-exports/skip_typescript.html"><strong aria-hidden="true">2.23.2.14.</strong> skip_typescript</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="reference/attributes/on-rust-exports/getter_with_clone.html"><strong aria-hidden="true">2.23.2.15.</strong> getter_with_clone</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="reference/attributes/on-rust-exports/private.html"><strong aria-hidden="true">2.23.2.16.</strong> private</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="reference/attributes/on-rust-exports/unchecked_type.html"><strong aria-hidden="true">2.23.2.17.</strong> unchecked_return_type, unchecked_param_type, and unchecked_optional_param_type</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="reference/attributes/on-rust-exports/description.html"><strong aria-hidden="true">2.23.2.18.</strong> return_description and param_description</a></span></li></ol></li></ol></li></ol><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="web-sys/index.html"><strong aria-hidden="true">3.</strong> web-sys</a></span><ol class="section"><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="web-sys/using-web-sys.html"><strong aria-hidden="true">3.1.</strong> Using web-sys</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="web-sys/cargo-features.html"><strong aria-hidden="true">3.2.</strong> Cargo Features</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="web-sys/function-overloads.html"><strong aria-hidden="true">3.3.</strong> Function Overloads</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="web-sys/type-translations.html"><strong aria-hidden="true">3.4.</strong> Type Translations</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="web-sys/inheritance.html"><strong aria-hidden="true">3.5.</strong> Inheritance</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="web-sys/unstable-apis.html"><strong aria-hidden="true">3.6.</strong> Unstable APIs</a></span></li></ol><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="wasm-bindgen-test/index.html"><strong aria-hidden="true">4.</strong> Testing with wasm-bindgen-test</a></span><ol class="section"><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="wasm-bindgen-test/usage.html"><strong aria-hidden="true">4.1.</strong> Usage</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="wasm-bindgen-test/asynchronous-tests.html"><strong aria-hidden="true">4.2.</strong> Writing Asynchronous Tests</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="wasm-bindgen-test/browsers.html"><strong aria-hidden="true">4.3.</strong> Testing in Headless Browsers</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="wasm-bindgen-test/continuous-integration.html"><strong aria-hidden="true">4.4.</strong> Continuous Integration</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="wasm-bindgen-test/coverage.html"><strong aria-hidden="true">4.5.</strong> Coverage (Experimental)</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="wasm-bindgen-test/benchmark.html"><strong aria-hidden="true">4.6.</strong> Benchmark</a></span></li></ol><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="contributing/index.html"><strong aria-hidden="true">5.</strong> Contributing to wasm-bindgen</a></span><ol class="section"><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="contributing/testing.html"><strong aria-hidden="true">5.1.</strong> Testing</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="contributing/design/index.html"><strong aria-hidden="true">5.2.</strong> Internal Design</a></span><ol class="section"><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="contributing/design/js-objects-in-rust.html"><strong aria-hidden="true">5.2.1.</strong> JS Objects in Rust</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="contributing/design/exporting-rust.html"><strong aria-hidden="true">5.2.2.</strong> Exporting a function to JS</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="contributing/design/exporting-rust-struct.html"><strong aria-hidden="true">5.2.3.</strong> Exporting a struct to JS</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="contributing/design/importing-js.html"><strong aria-hidden="true">5.2.4.</strong> Importing a function from JS</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="contributing/design/importing-js-struct.html"><strong aria-hidden="true">5.2.5.</strong> Importing a class from JS</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="contributing/design/rust-type-conversions.html"><strong aria-hidden="true">5.2.6.</strong> Rust Type conversions</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="contributing/design/describe.html"><strong aria-hidden="true">5.2.7.</strong> Types in wasm-bindgen</a></span></li></ol><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="contributing/js-sys/index.html"><strong aria-hidden="true">5.3.</strong> js-sys</a></span><ol class="section"><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="contributing/js-sys/testing.html"><strong aria-hidden="true">5.3.1.</strong> Testing</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="contributing/js-sys/adding-more-apis.html"><strong aria-hidden="true">5.3.2.</strong> Adding More APIs</a></span></li></ol><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="contributing/web-sys/index.html"><strong aria-hidden="true">5.4.</strong> web-sys</a></span><ol class="section"><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="contributing/web-sys/overview.html"><strong aria-hidden="true">5.4.1.</strong> Overview</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="contributing/web-sys/testing.html"><strong aria-hidden="true">5.4.2.</strong> Testing</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="contributing/web-sys/logging.html"><strong aria-hidden="true">5.4.3.</strong> Logging</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="contributing/web-sys/supporting-more-web-apis.html"><strong aria-hidden="true">5.4.4.</strong> Supporting More Web APIs</a></span></li></ol><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="contributing/publishing.html"><strong aria-hidden="true">5.5.</strong> Publishing</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="contributing/governance.html"><strong aria-hidden="true">5.6.</strong> Governance</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="contributing/team.html"><strong aria-hidden="true">5.7.</strong> Team</a></span></li></ol></li></ol>';
        // Set the current, active page, and reveal it if it's hidden
        let current_page = document.location.href.toString().split('#')[0].split('?')[0];
        if (current_page.endsWith('/')) {
            current_page += 'index.html';
        }
        const links = Array.prototype.slice.call(this.querySelectorAll('a'));
        const l = links.length;
        for (let i = 0; i < l; ++i) {
            const link = links[i];
            const href = link.getAttribute('href');
            if (href && !href.startsWith('#') && !/^(?:[a-z+]+:)?\/\//.test(href)) {
                link.href = path_to_root + href;
            }
            // The 'index' page is supposed to alias the first chapter in the book.
            if (link.href === current_page
                || i === 0
                && path_to_root === ''
                && current_page.endsWith('/index.html')) {
                link.classList.add('active');
                let parent = link.parentElement;
                while (parent) {
                    if (parent.tagName === 'LI' && parent.classList.contains('chapter-item')) {
                        parent.classList.add('expanded');
                    }
                    parent = parent.parentElement;
                }
            }
        }
        // Track and set sidebar scroll position
        this.addEventListener('click', e => {
            if (e.target.tagName === 'A') {
                const clientRect = e.target.getBoundingClientRect();
                const sidebarRect = this.getBoundingClientRect();
                sessionStorage.setItem('sidebar-scroll-offset', clientRect.top - sidebarRect.top);
            }
        }, { passive: true });
        const sidebarScrollOffset = sessionStorage.getItem('sidebar-scroll-offset');
        sessionStorage.removeItem('sidebar-scroll-offset');
        if (sidebarScrollOffset !== null) {
            // preserve sidebar scroll position when navigating via links within sidebar
            const activeSection = this.querySelector('.active');
            if (activeSection) {
                const clientRect = activeSection.getBoundingClientRect();
                const sidebarRect = this.getBoundingClientRect();
                const currentOffset = clientRect.top - sidebarRect.top;
                this.scrollTop += currentOffset - parseFloat(sidebarScrollOffset);
            }
        } else {
            // scroll sidebar to current active section when navigating via
            // 'next/previous chapter' buttons
            const activeSection = document.querySelector('#mdbook-sidebar .active');
            if (activeSection) {
                activeSection.scrollIntoView({ block: 'center' });
            }
        }
        // Toggle buttons
        const sidebarAnchorToggles = document.querySelectorAll('.chapter-fold-toggle');
        function toggleSection(ev) {
            ev.currentTarget.parentElement.parentElement.classList.toggle('expanded');
        }
        Array.from(sidebarAnchorToggles).forEach(el => {
            el.addEventListener('click', toggleSection);
        });
    }
}
window.customElements.define('mdbook-sidebar-scrollbox', MDBookSidebarScrollbox);


// ---------------------------------------------------------------------------
// Support for dynamically adding headers to the sidebar.

(function() {
    // This is used to detect which direction the page has scrolled since the
    // last scroll event.
    let lastKnownScrollPosition = 0;
    // This is the threshold in px from the top of the screen where it will
    // consider a header the "current" header when scrolling down.
    const defaultDownThreshold = 150;
    // Same as defaultDownThreshold, except when scrolling up.
    const defaultUpThreshold = 300;
    // The threshold is a virtual horizontal line on the screen where it
    // considers the "current" header to be above the line. The threshold is
    // modified dynamically to handle headers that are near the bottom of the
    // screen, and to slightly offset the behavior when scrolling up vs down.
    let threshold = defaultDownThreshold;
    // This is used to disable updates while scrolling. This is needed when
    // clicking the header in the sidebar, which triggers a scroll event. It
    // is somewhat finicky to detect when the scroll has finished, so this
    // uses a relatively dumb system of disabling scroll updates for a short
    // time after the click.
    let disableScroll = false;
    // Array of header elements on the page.
    let headers;
    // Array of li elements that are initially collapsed headers in the sidebar.
    // I'm not sure why eslint seems to have a false positive here.
    // eslint-disable-next-line prefer-const
    let headerToggles = [];
    // This is a debugging tool for the threshold which you can enable in the console.
    let thresholdDebug = false;

    // Updates the threshold based on the scroll position.
    function updateThreshold() {
        const scrollTop = window.pageYOffset || document.documentElement.scrollTop;
        const windowHeight = window.innerHeight;
        const documentHeight = document.documentElement.scrollHeight;

        // The number of pixels below the viewport, at most documentHeight.
        // This is used to push the threshold down to the bottom of the page
        // as the user scrolls towards the bottom.
        const pixelsBelow = Math.max(0, documentHeight - (scrollTop + windowHeight));
        // The number of pixels above the viewport, at least defaultDownThreshold.
        // Similar to pixelsBelow, this is used to push the threshold back towards
        // the top when reaching the top of the page.
        const pixelsAbove = Math.max(0, defaultDownThreshold - scrollTop);
        // How much the threshold should be offset once it gets close to the
        // bottom of the page.
        const bottomAdd = Math.max(0, windowHeight - pixelsBelow - defaultDownThreshold);
        let adjustedBottomAdd = bottomAdd;

        // Adjusts bottomAdd for a small document. The calculation above
        // assumes the document is at least twice the windowheight in size. If
        // it is less than that, then bottomAdd needs to be shrunk
        // proportional to the difference in size.
        if (documentHeight < windowHeight * 2) {
            const maxPixelsBelow = documentHeight - windowHeight;
            const t = 1 - pixelsBelow / Math.max(1, maxPixelsBelow);
            const clamp = Math.max(0, Math.min(1, t));
            adjustedBottomAdd *= clamp;
        }

        let scrollingDown = true;
        if (scrollTop < lastKnownScrollPosition) {
            scrollingDown = false;
        }

        if (scrollingDown) {
            // When scrolling down, move the threshold up towards the default
            // downwards threshold position. If near the bottom of the page,
            // adjustedBottomAdd will offset the threshold towards the bottom
            // of the page.
            const amountScrolledDown = scrollTop - lastKnownScrollPosition;
            const adjustedDefault = defaultDownThreshold + adjustedBottomAdd;
            threshold = Math.max(adjustedDefault, threshold - amountScrolledDown);
        } else {
            // When scrolling up, move the threshold down towards the default
            // upwards threshold position. If near the bottom of the page,
            // quickly transition the threshold back up where it normally
            // belongs.
            const amountScrolledUp = lastKnownScrollPosition - scrollTop;
            const adjustedDefault = defaultUpThreshold - pixelsAbove
                + Math.max(0, adjustedBottomAdd - defaultDownThreshold);
            threshold = Math.min(adjustedDefault, threshold + amountScrolledUp);
        }

        if (documentHeight <= windowHeight) {
            threshold = 0;
        }

        if (thresholdDebug) {
            const id = 'mdbook-threshold-debug-data';
            let data = document.getElementById(id);
            if (data === null) {
                data = document.createElement('div');
                data.id = id;
                data.style.cssText = `
                    position: fixed;
                    top: 50px;
                    right: 10px;
                    background-color: 0xeeeeee;
                    z-index: 9999;
                    pointer-events: none;
                `;
                document.body.appendChild(data);
            }
            data.innerHTML = `
                <table>
                  <tr><td>documentHeight</td><td>${documentHeight.toFixed(1)}</td></tr>
                  <tr><td>windowHeight</td><td>${windowHeight.toFixed(1)}</td></tr>
                  <tr><td>scrollTop</td><td>${scrollTop.toFixed(1)}</td></tr>
                  <tr><td>pixelsAbove</td><td>${pixelsAbove.toFixed(1)}</td></tr>
                  <tr><td>pixelsBelow</td><td>${pixelsBelow.toFixed(1)}</td></tr>
                  <tr><td>bottomAdd</td><td>${bottomAdd.toFixed(1)}</td></tr>
                  <tr><td>adjustedBottomAdd</td><td>${adjustedBottomAdd.toFixed(1)}</td></tr>
                  <tr><td>scrollingDown</td><td>${scrollingDown}</td></tr>
                  <tr><td>threshold</td><td>${threshold.toFixed(1)}</td></tr>
                </table>
            `;
            drawDebugLine();
        }

        lastKnownScrollPosition = scrollTop;
    }

    function drawDebugLine() {
        if (!document.body) {
            return;
        }
        const id = 'mdbook-threshold-debug-line';
        const existingLine = document.getElementById(id);
        if (existingLine) {
            existingLine.remove();
        }
        const line = document.createElement('div');
        line.id = id;
        line.style.cssText = `
            position: fixed;
            top: ${threshold}px;
            left: 0;
            width: 100vw;
            height: 2px;
            background-color: red;
            z-index: 9999;
            pointer-events: none;
        `;
        document.body.appendChild(line);
    }

    function mdbookEnableThresholdDebug() {
        thresholdDebug = true;
        updateThreshold();
        drawDebugLine();
    }

    window.mdbookEnableThresholdDebug = mdbookEnableThresholdDebug;

    // Updates which headers in the sidebar should be expanded. If the current
    // header is inside a collapsed group, then it, and all its parents should
    // be expanded.
    function updateHeaderExpanded(currentA) {
        // Add expanded to all header-item li ancestors.
        let current = currentA.parentElement;
        while (current) {
            if (current.tagName === 'LI' && current.classList.contains('header-item')) {
                current.classList.add('expanded');
            }
            current = current.parentElement;
        }
    }

    // Updates which header is marked as the "current" header in the sidebar.
    // This is done with a virtual Y threshold, where headers at or below
    // that line will be considered the current one.
    function updateCurrentHeader() {
        if (!headers || !headers.length) {
            return;
        }

        // Reset the classes, which will be rebuilt below.
        const els = document.getElementsByClassName('current-header');
        for (const el of els) {
            el.classList.remove('current-header');
        }
        for (const toggle of headerToggles) {
            toggle.classList.remove('expanded');
        }

        // Find the last header that is above the threshold.
        let lastHeader = null;
        for (const header of headers) {
            const rect = header.getBoundingClientRect();
            if (rect.top <= threshold) {
                lastHeader = header;
            } else {
                break;
            }
        }
        if (lastHeader === null) {
            lastHeader = headers[0];
            const rect = lastHeader.getBoundingClientRect();
            const windowHeight = window.innerHeight;
            if (rect.top >= windowHeight) {
                return;
            }
        }

        // Get the anchor in the summary.
        const href = '#' + lastHeader.id;
        const a = [...document.querySelectorAll('.header-in-summary')]
            .find(element => element.getAttribute('href') === href);
        if (!a) {
            return;
        }

        a.classList.add('current-header');

        updateHeaderExpanded(a);
    }

    // Updates which header is "current" based on the threshold line.
    function reloadCurrentHeader() {
        if (disableScroll) {
            return;
        }
        updateThreshold();
        updateCurrentHeader();
    }


    // When clicking on a header in the sidebar, this adjusts the threshold so
    // that it is located next to the header. This is so that header becomes
    // "current".
    function headerThresholdClick(event) {
        // See disableScroll description why this is done.
        disableScroll = true;
        setTimeout(() => {
            disableScroll = false;
        }, 100);
        // requestAnimationFrame is used to delay the update of the "current"
        // header until after the scroll is done, and the header is in the new
        // position.
        requestAnimationFrame(() => {
            requestAnimationFrame(() => {
                // Closest is needed because if it has child elements like <code>.
                const a = event.target.closest('a');
                const href = a.getAttribute('href');
                const targetId = href.substring(1);
                const targetElement = document.getElementById(targetId);
                if (targetElement) {
                    threshold = targetElement.getBoundingClientRect().bottom;
                    updateCurrentHeader();
                }
            });
        });
    }

    // Takes the nodes from the given head and copies them over to the
    // destination, along with some filtering.
    function filterHeader(source, dest) {
        const clone = source.cloneNode(true);
        clone.querySelectorAll('mark').forEach(mark => {
            mark.replaceWith(...mark.childNodes);
        });
        dest.append(...clone.childNodes);
    }

    // Scans page for headers and adds them to the sidebar.
    document.addEventListener('DOMContentLoaded', function() {
        const activeSection = document.querySelector('#mdbook-sidebar .active');
        if (activeSection === null) {
            return;
        }

        const main = document.getElementsByTagName('main')[0];
        headers = Array.from(main.querySelectorAll('h2, h3, h4, h5, h6'))
            .filter(h => h.id !== '' && h.children.length && h.children[0].tagName === 'A');

        if (headers.length === 0) {
            return;
        }

        // Build a tree of headers in the sidebar.

        const stack = [];

        const firstLevel = parseInt(headers[0].tagName.charAt(1));
        for (let i = 1; i < firstLevel; i++) {
            const ol = document.createElement('ol');
            ol.classList.add('section');
            if (stack.length > 0) {
                stack[stack.length - 1].ol.appendChild(ol);
            }
            stack.push({level: i + 1, ol: ol});
        }

        // The level where it will start folding deeply nested headers.
        const foldLevel = 3;

        for (let i = 0; i < headers.length; i++) {
            const header = headers[i];
            const level = parseInt(header.tagName.charAt(1));

            const currentLevel = stack[stack.length - 1].level;
            if (level > currentLevel) {
                // Begin nesting to this level.
                for (let nextLevel = currentLevel + 1; nextLevel <= level; nextLevel++) {
                    const ol = document.createElement('ol');
                    ol.classList.add('section');
                    const last = stack[stack.length - 1];
                    const lastChild = last.ol.lastChild;
                    // Handle the case where jumping more than one nesting
                    // level, which doesn't have a list item to place this new
                    // list inside of.
                    if (lastChild) {
                        lastChild.appendChild(ol);
                    } else {
                        last.ol.appendChild(ol);
                    }
                    stack.push({level: nextLevel, ol: ol});
                }
            } else if (level < currentLevel) {
                while (stack.length > 1 && stack[stack.length - 1].level > level) {
                    stack.pop();
                }
            }

            const li = document.createElement('li');
            li.classList.add('header-item');
            li.classList.add('expanded');
            if (level < foldLevel) {
                li.classList.add('expanded');
            }
            const span = document.createElement('span');
            span.classList.add('chapter-link-wrapper');
            const a = document.createElement('a');
            span.appendChild(a);
            a.href = '#' + header.id;
            a.classList.add('header-in-summary');
            filterHeader(header.children[0], a);
            a.addEventListener('click', headerThresholdClick);
            const nextHeader = headers[i + 1];
            if (nextHeader !== undefined) {
                const nextLevel = parseInt(nextHeader.tagName.charAt(1));
                if (nextLevel > level && level >= foldLevel) {
                    const toggle = document.createElement('a');
                    toggle.classList.add('chapter-fold-toggle');
                    toggle.classList.add('header-toggle');
                    toggle.addEventListener('click', () => {
                        li.classList.toggle('expanded');
                    });
                    const toggleDiv = document.createElement('div');
                    toggleDiv.textContent = '❱';
                    toggle.appendChild(toggleDiv);
                    span.appendChild(toggle);
                    headerToggles.push(li);
                }
            }
            li.appendChild(span);

            const currentParent = stack[stack.length - 1];
            currentParent.ol.appendChild(li);
        }

        const onThisPage = document.createElement('div');
        onThisPage.classList.add('on-this-page');
        onThisPage.append(stack[0].ol);
        const activeItemSpan = activeSection.parentElement;
        activeItemSpan.after(onThisPage);
    });

    document.addEventListener('DOMContentLoaded', reloadCurrentHeader);
    document.addEventListener('scroll', reloadCurrentHeader, { passive: true });
})();

