export class TryFromJsValueCustomType {
  constructor() {
    this.value = 42;
  }

  get_value() {
    return this.value;
  }
}

export function make_custom_type() {
  return new TryFromJsValueCustomType();
}

export function make_plain_object() {
  return { value: 42 };
}

export function make_string_array() {
  return ["hello", "world", ""];
}

export function make_mixed_array() {
  return ["hello", 42, "world"];
}

export function make_number_array() {
  return [1, 2, 3, 4, 5];
}

export function make_nested_array() {
  return [["a", "b"], ["c"], []];
}

export function make_empty_array() {
  return [];
}

export function make_array_with_undefined() {
  return ["hello", undefined, "world"];
}

export function make_array_like_object() {
  return { 0: "a", 1: "b", length: 2 };
}
