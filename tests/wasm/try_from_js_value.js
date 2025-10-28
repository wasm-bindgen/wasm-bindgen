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
