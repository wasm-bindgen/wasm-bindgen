// Test interface hierarchy for Upcast functionality
[Constructor]
interface BaseType {
  attribute DOMString value;
};

[Constructor]
interface ChildType : BaseType {
  attribute long childValue;
};

[Constructor]
interface GrandChildType : ChildType {
  attribute boolean grandChildValue;
};

// Functions that accept parent types should also accept child types
interface UpcastTest {
  // Function accepting BaseType
  static DOMString processBase(BaseType obj);

  // Function accepting ChildType
  static DOMString processChild(ChildType obj);
};
