import {
  ExportedNamedStruct,
  named_struct_by_value,
  named_struct_by_shared_ref,
  named_struct_by_exclusive_ref,
  return_named_struct,
  named_struct_by_optional_value,
  return_optional_named_struct,

  ExportedTupleStruct,
  return_tuple_struct
} from './guide_supported_types_examples';

let namedStruct = return_named_struct(42);
console.log(namedStruct instanceof ExportedNamedStruct); // true
console.log(namedStruct.inner); // 42

named_struct_by_shared_ref(namedStruct);
named_struct_by_exclusive_ref(namedStruct);
named_struct_by_value(namedStruct);
// `namedStruct` has been moved into Rust and can no longer be used.

let optionalNamedStruct = return_optional_named_struct(42);
named_struct_by_optional_value(optionalNamedStruct);

let tupleStruct = return_tuple_struct(10, 20);
console.log(tupleStruct instanceof ExportedTupleStruct); // true
console.log(tupleStruct[0], tupleStruct[1]); // 10, 20
