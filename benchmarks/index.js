// Benchmarking framework that we're using
import 'https://unpkg.com/lodash@4.17.11/lodash.js';
import 'https://unpkg.com/benchmark@2.1.4/benchmark.js';

// Import lots of functions from JS/wasm, and rename everything to have the
// namespace of where it's coming from.
import wbindgen_init, {
  call_js_thunk_n_times as wbindgen_call_js_thunk_n_times,
  call_js_add_n_times as wbindgen_call_js_add_n_times,
  thunk as wbindgen_thunk,
  add as wbindgen_add,
  fibonacci as wbindgen_fibonacci,
  call_node_first_child_n_times as wbindgen_call_node_first_child_n_times,
  call_node_node_type_n_times as wbindgen_call_node_node_type_n_times,
  count_node_types as wbindgen_count_node_types,
  call_first_child_final_n_times as wbindgen_call_first_child_final_n_times,
  call_first_child_structural_n_times as wbindgen_call_first_child_structural_n_times,
  call_foo_bar_final_n_times as wbindgen_call_foo_bar_final_n_times,
  call_use_baz_n_times as wbindgen_call_use_baz_n_times,
  call_foo_bar_structural_n_times as wbindgen_call_foo_bar_structural_n_times,
  str_roundtrip as wbindgen_str_roundtrip,
} from './pkg/wasm_bindgen_benchmark.js';
import {
  call_js_thunk_n_times as js_call_js_thunk_n_times,
  call_js_add_n_times as js_call_js_add_n_times,
  thunk as js_thunk,
  add as js_add,
  fibonacci as js_fibonacci,
  call_node_first_child_n_times as js_call_node_first_child_n_times,
  call_node_node_type_n_times as js_call_node_node_type_n_times,
  count_node_types as js_count_node_types,
} from './js-benchmarks.js';
import * as globals from './globals.js';
import { Lock } from './utils.js';

// These are set for `raw.wasm`, which we import and configure manually:
let raw_call_js_thunk_n_times = null;
let raw_call_js_add_n_times = null;
let raw_thunk = null;
let raw_add = null;

// Create a `Map` of all benchmarks that we're going to execute, where the map
// is from a benchmark's name to a thunk to execute for the benchmark.
function makeBenchmarks() {
  const benchmarks = new Map();

  benchmarks.wbindgen_thunk = wbindgen_thunk;
  benchmarks.raw_thunk = raw_thunk;
  benchmarks.js_thunk = js_thunk;

  benchmarks.wbindgen_fib_40 = () => wbindgen_fibonacci(40);
  benchmarks.js_fib_40 = () => js_fibonacci(40);

  benchmarks.wbindgen_add = () => wbindgen_add(2, 3);
  benchmarks.raw_add = () => raw_add(2, 3);
  benchmarks.js_add = () => js_add(2, 3);

  benchmarks.js_call_js_thunk_n_times = () => js_call_js_thunk_n_times(10000);
  benchmarks.raw_call_js_thunk_n_times = () => raw_call_js_thunk_n_times(10000);
  benchmarks.wbindgen_call_js_thunk_n_times = () => wbindgen_call_js_thunk_n_times(10000);

  benchmarks.js_call_js_add_n_times = () => js_call_js_add_n_times(10000, 2, 3);
  benchmarks.raw_call_js_add_n_times = () => raw_call_js_add_n_times(10000, 2, 3);
  benchmarks.wbindgen_call_js_add_n_times = () => wbindgen_call_js_add_n_times(10000, 2, 3);

  const list = [];
  for (let i = 0; i < 10; i++)
    list.push(document.body);
  benchmarks.wbindgen_call_node_first_child_n_times = () => wbindgen_call_node_first_child_n_times(1000, list);
  benchmarks.js_call_node_first_child_n_times = () => js_call_node_first_child_n_times(1000, list);
  benchmarks.wbindgen_call_node_node_type_n_times = () => wbindgen_call_node_node_type_n_times(1000, list);
  benchmarks.js_call_node_node_type_n_times = () => js_call_node_node_type_n_times(1000, list);

  const body = document.body;
  benchmarks.wbindgen_count_node_types = () => wbindgen_count_node_types(body);
  benchmarks.js_count_node_types = () => js_count_node_types(body);

  benchmarks.wbindgen_call_first_child_final_n_times = () => wbindgen_call_first_child_final_n_times(10000, body);
  benchmarks.wbindgen_call_first_child_structural_n_times = () => wbindgen_call_first_child_structural_n_times(10000, body);

  const foo = new globals.Foo();
  benchmarks.wbindgen_call_foo_bar_final_n_times = () => wbindgen_call_foo_bar_final_n_times(10000, foo);
  benchmarks.wbindgen_call_foo_bar_structural_n_times = () => wbindgen_call_foo_bar_structural_n_times(10000, foo);
  benchmarks.wbindgen_call_use_baz_n_times = () => wbindgen_call_use_baz_n_times(10000);


  const strings = {
    ascii_small: 'ja',
    ascii_medium: 'aym0566x',
    ascii_number: '505874924095815681',
    ascii_date: 'Sun Aug 31 00:29:15 +0000 2014',
    ascii_url: 'https://pbs.twimg.com/profile_images/497760886795153410/LDjAwR_y_normal.jpeg',
    ascii_link: '<a href="http://twitter.com/download/iphone" rel="nofollow">Twitter for iPhone</a>',
    unicode: '@aym0566x \n\n名前:前田あゆみ\n第一印象:なんか怖っ！\n今の印象:とりあえずキモい。噛み合わない\n好きなところ:ぶすでキモいとこ😋✨✨\n思い出:んーーー、ありすぎ😊❤️\nLINE交換できる？:あぁ……ごめん✋\nトプ画をみて:照れますがな😘✨\n一言:お前は一生もんのダチ💖'
  }
  const template = document.querySelector('tr.str-benchmark');
  template.remove();
  const tbody = document.querySelector('tbody#wbindgen-body');
  for (const bm in strings) {
    const s = strings[bm];
    const bm_name = `wbindgen_str_${bm}`;
    benchmarks[bm_name] = () => wbindgen_str_roundtrip(s);

    const row = template.cloneNode(true);
    row.querySelector('.str').textContent = bm;
    row.querySelector('td.bm').id = bm_name;
    row.removeAttribute('style');
    tbody.appendChild(row);
  }

  return benchmarks;
}

// Set up the page and initialize all event handlers and such.
function run() {
  const benchmarks = makeBenchmarks();

  const benchmarkLock = new Lock();
  for (const td of document.querySelectorAll('td.bm')) {
    const bm = benchmarks[td.id];
    if (typeof bm !== 'function')
      throw new Error(`no benchmark registered for ${td.id}`);

    const run = document.createElement('a');
    run.href = '#';
    run.innerText = '(run)';
    run.onclick = function() {
      benchmarkLock.withLock(async () => {
        await executeAndUpdate(td.id, bm, td);
      });
      run.remove();
      td.innerText = 'executing ...';
      return false;
    };
    td.appendChild(run);
  }

  for (const a of document.querySelectorAll('.about-open')) {
    a.onclick = function() {
      a.nextElementSibling.style.display = 'block';
      a.remove();
      return false;
    };
  }
}

async function executeAndUpdate(name, bm, td) {
  const result = await executeBenchmark(name, bm);
  console.log(result.target);
  const rme = Math.round(result.target.stats.rme * 100) / 100;
  td.innerText = `${Math.round(result.target.hz).toLocaleString()}/s ±${rme}%`;
}

function executeBenchmark(name, bm) {
  return new Promise((resolve, reject) => {
    const suite = new Benchmark.Suite();
    suite.add(name, bm);
    suite.on('cycle', resolve);
    suite.run({ async: true });
  });
}

// Load Wasm files and when they're done (plus the DOM) then we initialize
// everything
const wasms = [];
wasms.push(wbindgen_init('./pkg/wasm_bindgen_benchmark_bg.wasm'));
wasms.push(fetch('./raw.wasm')
  .then(r => r.arrayBuffer())
  .then(m => WebAssembly.instantiate(m, { './globals.js': globals }))
  .then(m => {
    raw_call_js_thunk_n_times = m.instance.exports.call_js_thunk_n_times;
    raw_call_js_add_n_times = m.instance.exports.call_js_add_n_times;
    raw_thunk = m.instance.exports.thunk;
    raw_add = m.instance.exports.add;
  }));

Promise.all(wasms)
  .then(() => {
    if (document.readyState === 'loading')
      document.addEventListener('DOMContentLoaded', run);
    else
      run();
  })
  .catch(console.error);
