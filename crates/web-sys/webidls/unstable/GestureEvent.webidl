// WebKit-specific API
// Information gathered from https://developer.apple.com/documentation/webkitjs/gestureevent

interface GestureEvent : UIEvent
{
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

  undefined initGestureEvent(
      DOMString type,
      boolean canBubble,
      boolean cancelable,
      Window? view,
      long detail,
      long screenX,
      long screenY,
      long clientX,
      long clientY,
      boolean ctrlKey,
      boolean altKey,
      boolean shiftKey,
      boolean metaKey,
      EventTarget? target,
      float scale,
      float rotation
  );
};
