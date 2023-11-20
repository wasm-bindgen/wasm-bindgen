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

take_number(NumberEnum.Foo);
take_string("spam");

take_number_option(NumberEnum.Bar);
take_number_option(undefined);

take_string_option("eggs");
take_string_option(undefined);

return_number(); // typed as `NumberEnum`
return_string(); // typed as `any`

return_number_option(); // typed as `NumberEnum | undefined`
return_string_option(); // typed as `any | undefined`
