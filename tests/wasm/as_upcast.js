const assert = require('assert');

class Parent {
    constructor(value) {
        this.value = value;
    }
}

class Child extends Parent {
    constructor(value) {
        super(value);
    }
}

exports.Parent = Parent;
exports.Child = Child;

exports.process_parent = (obj) => {
    assert(obj instanceof Parent);
    return obj.value;
};

exports.process_promise = async (promise) => {
    const parent = await promise;
    assert(parent instanceof Parent);
    return parent.value;
};
