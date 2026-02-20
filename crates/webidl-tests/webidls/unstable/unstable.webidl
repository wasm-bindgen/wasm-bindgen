enum UnstableEnum {
  "a",
  "b"
};

dictionary UnstableDictionary {
  UnstableEnum unstableEnum;
};

typedef unsigned long UnstableTypedef;

[NoInterfaceObject]
partial interface UnstableInterface {
  UnstableTypedef enum_value(optional UnstableDictionary unstableDictionary = {});
};

interface GetUnstableInterface {
  static UnstableInterface get();
};

// Test dictionary with union field - should generate expanded setters
[Exposed=Window]
interface TypeA {
  constructor();
};
[Exposed=Window]
interface TypeB {
  constructor();
};

dictionary DictWithUnion {
  required (TypeA or TypeB) view;
  (TypeA or TypeB) optionalView;
};
