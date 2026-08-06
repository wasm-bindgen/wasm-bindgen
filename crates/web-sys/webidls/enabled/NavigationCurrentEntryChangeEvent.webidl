[Exposed=Window]
interface NavigationCurrentEntryChangeEvent : Event {
  constructor(DOMString type, NavigationCurrentEntryChangeEventInit eventInitDict);

  readonly attribute NavigationApiType? navigationType;
  readonly attribute NavigationHistoryEntry from;
};

dictionary NavigationCurrentEntryChangeEventInit : EventInit {
  NavigationApiType? navigationType = null;
  required NavigationHistoryEntry from;
};
