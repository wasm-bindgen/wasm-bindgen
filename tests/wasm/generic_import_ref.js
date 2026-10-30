exports.RefThing = class RefThing {
  constructor(val) {
    this.val = val;
  }
};

exports.readRefThing = function(x) {
  return x.val;
};

exports.echoRef = function(x) {
  return x;
};

exports.show = function(v) {
  return `${typeof v} ${v}`;
};

exports.Init = class Init {};

exports.makeReq = function(input, init) {
  return { input, cf: init.cf };
};
