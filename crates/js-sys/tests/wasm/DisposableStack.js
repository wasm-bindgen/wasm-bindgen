exports.make_disposable = (log, name) => ({
  [Symbol.dispose]() {
    log.push(name);
  },
});

exports.make_throwing_disposable = (message) => ({
  [Symbol.dispose]() {
    throw new Error(message);
  },
});

exports.make_async_disposable = (log, name) => ({
  async [Symbol.asyncDispose]() {
    await null;
    log.push(name);
  },
});

exports.delayed_push = (log, name) =>
  Promise.resolve().then(() => {
    log.push(name);
  });
