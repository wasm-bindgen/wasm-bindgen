/* -*- Mode: IDL; tab-width: 2; indent-tabs-mode: nil; c-basic-offset: 2 -*- */
// WebKit-specific API
// Information gathered from https://developer.apple.com/documentation/webkitjs/gestureevent

[Constructor(DOMString type, optional GestureEventInit eventInitDict)]
interface GestureEvent : UIEvent
{
  readonly attribute float   scale;
  readonly attribute float   rotation;

  readonly attribute boolean ctrlKey;
  readonly attribute boolean shiftKey;
  readonly attribute boolean altKey;
  readonly attribute boolean metaKey;

  [BinaryName="clientX"]
  readonly attribute long    x;
  [BinaryName="clientY"]
  readonly attribute long    y;
  [NeedsCallerType]
  readonly attribute long    screenX;
  [NeedsCallerType]
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
