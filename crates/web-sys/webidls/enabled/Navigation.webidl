[Exposed=Window]
interface Navigation : EventTarget {
  sequence<NavigationHistoryEntry> entries();
  readonly attribute NavigationHistoryEntry? currentEntry;
  undefined updateCurrentEntry(NavigationUpdateCurrentEntryOptions options);
  readonly attribute NavigationTransition? transition;
  readonly attribute NavigationActivation? activation;

  readonly attribute boolean canGoBack;
  readonly attribute boolean canGoForward;

  NavigationResult navigate(USVString url, optional NavigationNavigateOptions options = {});
  NavigationResult reload(optional NavigationReloadOptions options = {});

  NavigationResult traverseTo(DOMString key, optional NavigationOptions options = {});
  NavigationResult back(optional NavigationOptions options = {});
  NavigationResult forward(optional NavigationOptions options = {});

  attribute EventHandler onnavigate;
  attribute EventHandler onnavigatesuccess;
  attribute EventHandler onnavigateerror;
  attribute EventHandler oncurrententrychange;
};

dictionary NavigationUpdateCurrentEntryOptions {
  required any state;
};

dictionary NavigationOptions {
  any info;
};

dictionary NavigationNavigateOptions : NavigationOptions {
  any state;
  NavigationHistoryBehavior history = "auto";
};

dictionary NavigationReloadOptions : NavigationOptions {
  any state;
};

dictionary NavigationResult {
  Promise<NavigationHistoryEntry> committed;
  Promise<NavigationHistoryEntry> finished;
};

enum NavigationHistoryBehavior {
  "auto",
  "push",
  "replace"
};

// Named `NavigationType` in the HTML spec; renamed to avoid colliding with
// the legacy `NavigationType` from PerformanceNavigationTiming.
enum NavigationApiType {
  "push",
  "replace",
  "reload",
  "traverse"
};
