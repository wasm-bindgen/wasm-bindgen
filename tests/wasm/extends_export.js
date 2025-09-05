// Mock implementations for testing
exports.RpcTarget = class RpcTarget {
    constructor() {
        // Base class constructor
        this.baseProperty = "from_base";
    }
    
    base_method() {
        return "base_method_called";
    }
};

exports.AnotherBase = class AnotherBase {
    constructor() {
        this.anotherProperty = "another_base";
    }
    
    another_method() {
        return "another_method_called";
    }
};
