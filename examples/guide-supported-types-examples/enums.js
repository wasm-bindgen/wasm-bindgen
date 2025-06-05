import {
  take_number,
  take_string,
  take_number_option,
  take_string_option,
  return_number,
  return_string,
  return_number_option,
  return_string_option,
  NumberEnum,
  // nothing generated for StringEnum :(
} from "./guide_supported_types_examples";

take_enum_number(NumberEnum.Foo);
take_enum_string("spam");

take_enum_number_option(NumberEnum.Bar);
take_enum_number_option(undefined);

take_enum_string_option("eggs");
take_enum_string_option(undefined);

return_number(); // -> `NumberEnum`
return_string(); // -> `StringEnum`

return_number_option(); // -> `NumberEnum | undefined`
return_string_option(); // -> `StringEnum | undefined`
