// WebKit-specific API
// Information gathered from https://developer.apple.com/documentation/webkitjs/gestureevent

interface GestureEvent : UIEvent
{
  constructor(DOMString type, optional GestureEventInit eventInitDict);

  readonly attribute float   scale;
  readonly attribute float   rotation;

  readonly attribute boolean ctrlKey;
  readonly attribute boolean shiftKey;
  readonly attribute boolean altKey;
  readonly attribute boolean metaKey;

  readonly attribute long    clientX;
  readonly attribute long    clientY;
  readonly attribute long    screenX;
  readonly attribute long    screenY;
};

dictionary GestureEventInit : UIEventInit
{
  float   scale;
  float   rotation;

  boolean ctrlKey;
  boolean shiftKey;
  boolean altKey;
  boolean metaKey;

  long    clientX;
  long    clientY;
  long    screenX;
  long    screenY;
};
