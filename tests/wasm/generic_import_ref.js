exports.takeRefPrimitive = function(x) {
  return x + 1;
};

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
