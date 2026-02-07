// User Timing Level 3
// https://www.w3.org/TR/user-timing/
// W3C Candidate Recommendation Draft 13 February 2025

dictionary PerformanceMarkOptions {
    any detail;
    DOMHighResTimeStamp startTime;
};

dictionary PerformanceMeasureOptions {
    any detail;
    (DOMString or DOMHighResTimeStamp) start;
    DOMHighResTimeStamp duration;
    (DOMString or DOMHighResTimeStamp) end;
};

// Override stable Performance methods with Level 3 signatures
[Exposed=(Window,Worker)]
partial interface Performance {
  // Override: stable returns undefined, Level 3 returns PerformanceMark
  // Throws SyntaxError if markName matches PerformanceTiming attribute
  // Throws TypeError if startTime is negative
  [Throws]
  PerformanceMark mark(DOMString markName);
  [Throws]
  PerformanceMark mark(DOMString markName, optional PerformanceMarkOptions markOptions = {});

  // Override: stable returns undefined, Level 3 returns PerformanceMeasure
  // Throws TypeError for invalid options combinations
  // Throws SyntaxError if mark name not found
  [Throws]
  PerformanceMeasure measure(DOMString measureName);
  [Throws]
  PerformanceMeasure measure(DOMString measureName, optional DOMString startOrMeasureOptions);
  [Throws]
  PerformanceMeasure measure(DOMString measureName, optional DOMString startOrMeasureOptions, optional DOMString endMark);
  [Throws]
  PerformanceMeasure measure(DOMString measureName, optional PerformanceMeasureOptions startOrMeasureOptions = {});
};

[Exposed=(Window,Worker)]
interface PerformanceMark : PerformanceEntry {
  // Throws SyntaxError if markName matches PerformanceTiming attribute (in Window)
  // Throws TypeError if startTime is negative
  [Throws]
  constructor(DOMString markName, optional PerformanceMarkOptions markOptions = {});
  readonly attribute any detail;
};

[Exposed=(Window,Worker)]
interface PerformanceMeasure : PerformanceEntry {
  readonly attribute any detail;
};
