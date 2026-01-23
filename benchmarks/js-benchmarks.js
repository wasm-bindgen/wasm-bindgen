import { jsthunk, add as jsadd } from './globals.js';

export function fibonacci(n) {
  let a = 1;
  let b = 1;
  let tmp = 0;

  while (n > 1) {
    tmp = b;
    b += a;
    a = tmp;
    --n;
  }

  return a;
}

export function thunk() {}

export function call_js_thunk_n_times(n) {
  for (var i = 0; i < n; i++) {
    jsthunk();
  }
}

export function add(a, b) {
  return a + b;
}

export function call_js_add_n_times(n, a, b) {
  for (var i = 0; i < n; i++) {
    jsadd(a, b);
  }
}

export function call_node_first_child_n_times(n, array_of_elements) {
  for (let i = 0; i < n; i++) {
    for (const element of array_of_elements)
      if (element.firstChild === null)
        throw new Error("bad");
  }
}

export function call_node_node_type_n_times(n, array_of_elements) {
  for (let i = 0; i < n; i++) {
    for (const element of array_of_elements)
      if (element.nodeType === 100)
        throw new Error("bad");
  }
}

export function call_node_has_child_nodes_n_times(n, array_of_elements) {
  for (let i = 0; i < n; i++) {
    for (const element of array_of_elements)
      if (!element.hasChildNodes())
        throw new Error("bad");
  }
}

export function count_node_types(element) {
  const types = [];

  function count(node, types) {
    while(node) {
      const type = node.nodeType;
      while (types.length <= type)
        types.push(0);
      types[type] += 1;
      count(node.firstChild, types);
      node = node.nextSibling;
    }
  }

  count(element, types);
  return types;
}
