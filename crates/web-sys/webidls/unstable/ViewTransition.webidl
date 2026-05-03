partial interface Document {
  [Throws]
  ViewTransition startViewTransition(
    optional (ViewTransitionUpdateCallback or StartViewTransitionOptions) callbackOptions = {}
  );
  readonly attribute ViewTransition? activeViewTransition;
};

callback ViewTransitionUpdateCallback = Promise<any> ();

dictionary StartViewTransitionOptions {
  ViewTransitionUpdateCallback? update = null;
  sequence<DOMString>? types = null;
};


partial interface Element {
  [Throws]
  ViewTransition startViewTransition(
    optional (ViewTransitionUpdateCallback or StartViewTransitionOptions) callbackOptions = {}
  );
  readonly attribute ViewTransition? activeViewTransition;
};

[Exposed=Window]
interface ViewTransition {
  readonly attribute Promise<undefined> updateCallbackDone;
  readonly attribute Promise<undefined> ready;
  readonly attribute Promise<undefined> finished;
  [Throws]
  undefined skipTransition();
  [SameObject] readonly attribute ViewTransitionTypeSet types;
  readonly attribute Element transitionRoot;
  undefined waitUntil(Promise<any> promise);
};

[Exposed=Window]
interface ViewTransitionTypeSet {
  setlike<DOMString>;
};

[Exposed=Window]
interface CSSViewTransitionRule : CSSRule {
  readonly attribute CSSOMString navigation;
  [SameObject] readonly attribute FrozenArray<CSSOMString> types;
};
