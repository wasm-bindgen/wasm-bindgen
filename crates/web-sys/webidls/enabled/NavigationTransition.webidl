[Exposed=Window]
interface NavigationTransition {
  readonly attribute NavigationType navigationType;
  readonly attribute NavigationHistoryEntry from;
  readonly attribute NavigationDestination to;
  readonly attribute Promise<undefined> committed;
  readonly attribute Promise<undefined> finished;
};
