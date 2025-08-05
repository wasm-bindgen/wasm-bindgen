[Exposed=Window]
interface CommandEvent : Event {
  constructor(DOMString type, optional CommandEventInit eventInitDict = {});
  readonly attribute Element? source;
  readonly attribute DOMString command;
};

dictionary CommandEventInit : EventInit {
  Element? source = null;
  DOMString command = "";
};