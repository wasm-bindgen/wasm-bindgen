// https://w3c.github.io/window-management/

partial interface Screen /* : EventTarget */ {
  [SecureContext]
  readonly attribute boolean isExtended;

  // This clashes with an attribute from `Screen.webidl`.
  // [SecureContext]
  // attribute EventHandler onchange;
};

partial interface Window {
  [SecureContext, Throws]
  Promise<ScreenDetails> getScreenDetails();
};

[Exposed=Window, SecureContext]
interface ScreenDetails : EventTarget {
  readonly attribute FrozenArray<ScreenDetailed> screens;
  readonly attribute ScreenDetailed currentScreen;

  attribute EventHandler onscreenschange;
  attribute EventHandler oncurrentscreenchange;
};

[Exposed=Window, SecureContext]
interface ScreenDetailed : Screen {
  readonly attribute long availLeft;
  readonly attribute long availTop;
  readonly attribute long left;
  readonly attribute long top;
  readonly attribute boolean isPrimary;
  readonly attribute boolean isInternal;
  readonly attribute float devicePixelRatio;
  readonly attribute DOMString label;
};

partial dictionary FullscreenOptions {
  ScreenDetailed screen;
};
