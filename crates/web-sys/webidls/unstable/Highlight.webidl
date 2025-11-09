enum HighlightType {
  "highlight",
  "spelling-error",
  "grammar-error"
};

[Exposed=Window]
interface Highlight {
  constructor(AbstractRange... initialRanges);
  setlike<AbstractRange>;
  attribute long priority;
  attribute HighlightType type;
};

partial namespace CSS {
  readonly attribute HighlightRegistry highlights;
};

[Exposed=Window]
interface HighlightRegistry {
  maplike<DOMString, Highlight>;
};

partial interface HighlightRegistry {
  sequence<HighlightHitResult> highlightsFromPoint(float x, float y, optional HighlightsFromPointOptions options = {});
};

dictionary HighlightHitResult {
  Highlight highlight;
  sequence<AbstractRange> ranges;
};

dictionary HighlightsFromPointOptions {
  sequence<ShadowRoot> shadowRoots = [];
};
