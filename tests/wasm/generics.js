exports.MyDurableObject = class MyDurableObject {
  constructor() {}
};

exports.MyDurableObjectStub = class MyDurableObjectStub {
  constructor() {}
};

exports.DurableObjectNamespace = class DurableObjectNamespace {
  constructor() {}

  getByName(name) {
    return new exports.MyDurableObjectStub();
  }
};
