exports.get_iterable = () => ["one", "two", "three"];

exports.get_not_iterable = () => new Object;

exports.get_symbol_iterator_throws = () => ({
  [Symbol.iterator]: () => { throw new Error("nope"); },
});

exports.get_symbol_iterator_not_function = () => ({
  [Symbol.iterator]: 5,
});

exports.get_symbol_iterator_returns_not_object = () => ({
  [Symbol.iterator]: () => 5,
});

exports.get_symbol_iterator_returns_object_without_next = () => ({
  [Symbol.iterator]: () => new Object,
});

exports.get_iterator_like = () => {
  const values = ["one", "two"];
  return { next: () => ({ done: values.length === 0, value: values.shift() }) };
};

exports.get_naturals = () => (function* () {
  let i = 0;
  while (true) yield i++;
})();

exports.get_throwing_iterator = () => (function* () {
  yield "one";
  throw new Error("nope");
})();
