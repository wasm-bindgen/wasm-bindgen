// Test fixture for Phase 1 fixes:
// - Numeric enums with negative values (1.1)
// - Multiple extends (1.2)
// - Heritage dotted paths (1.3)
// - try_ collision (1.4)
// - Member dedup after merge (1.5)

// 1.1: Numeric enum with negative values → should use #[repr(i32)]
declare enum SignedEnum {
  Negative = -1,
  Zero = 0,
  Positive = 1,
}

// 1.1: Numeric enum with only positive values → should use #[repr(u32)]
declare enum UnsignedEnum {
  A = 0,
  B = 1,
  C = 2,
}

// 1.1: Numeric enum with auto-increment from negative
declare enum AutoIncrement {
  Start = -2,
  // auto-incremented to -1
  Next,
  // auto-incremented to 0
  Last,
}

// 1.2: Interface extending multiple parents
interface EventTarget {
  addEventListener(type: string, listener: Function): void;
}

interface Serializable {
  toJSON(): any;
}

interface EventEmitter extends EventTarget, Serializable {
  emit(event: string): boolean;
}

// 1.3: Namespace with dotted heritage path
declare namespace NodeJS {
  interface EventEmitter {
    on(event: string, listener: Function): this;
  }
}

// 1.3: Class extending a dotted namespace path
declare class Stream extends NodeJS.EventEmitter {
  pipe(destination: Stream): Stream;
}

// 1.4: try_ collision — class has both `count` and `try_count`
declare class Counter {
  count(): number;
  try_count(): number;
  reset(): void;
}

// 1.5: Member dedup — two interface blocks with overlapping members
interface Duplex {
  read(): any;
  write(data: string): boolean;
}

interface Duplex {
  // Override write with different signature
  write(data: ArrayBuffer): boolean;
  end(): void;
}

// 1.6: Namespace with interface, variable, and dictionary — all should get js_namespace
declare namespace Intl {
  interface Collator {
    compare(a: string, b: string): number;
  }

  interface CollatorOptions {
    usage?: string;
    sensitivity?: string;
  }

  const defaultLocale: string;
}
