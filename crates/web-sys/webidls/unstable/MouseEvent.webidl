/* Unstable MouseEvent attributes with double types per CSSOM View spec draft */

partial interface MouseEvent {
  readonly attribute double screenX;
  readonly attribute double screenY;
  readonly attribute double clientX;
  readonly attribute double clientY;
  readonly attribute double offsetX;
  readonly attribute double offsetY;
  readonly attribute double pageX;
  readonly attribute double pageY;
};
