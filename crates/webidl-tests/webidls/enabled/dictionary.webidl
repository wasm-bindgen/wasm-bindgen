// example from https://heycam.github.io/webidl/#idl-dictionaries
dictionary B : A {
  long b;
  long a;
};

dictionary A {
  long c;
  long g;
};

dictionary C : B {
  long e;
  long f;
};

partial dictionary A {
  long h;
  long d;
};

// case needs changing
dictionary camel_case_me {
  long snakeCaseMe;
};

dictionary ManyTypes {
  DOMString a;
  octet n1;
  byte n2;
  unsigned short n3;
  short n4;
  unsigned long n5;
  long n6;
  // TODO: needs fixing
  // OtherDict c;
};

dictionary OtherDict {
  long a;
};

dictionary Required {
  required DOMString b;
  required long a;
  long c;
};

dictionary PreserveNames {
  long weird_fieldName;
};

// Test nullable sequence field (reproduces gen_L10nValue / gen_RegisteredKey bug)
dictionary DictWithNullableSequence {
  sequence<long>? nullableSequence = null;
};
