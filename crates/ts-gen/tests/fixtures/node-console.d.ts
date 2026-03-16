/**
 * Options for the Console constructor.
 */
export interface ConsoleOptions {
    stdout: object;
    stderr?: object | undefined;
    /**
     * Ignore errors when writing to the underlying streams.
     * @default true
     */
    ignoreErrors?: boolean | undefined;
    /**
     * Set color support for this `Console` instance.
     * @default 'auto'
     */
    colorMode?: boolean | string | undefined;
    /**
     * Set group indentation.
     * @default 2
     */
    groupIndentation?: number | undefined;
}

/**
 * The `Console` class can be used to create a simple logger with configurable
 * output streams and can be accessed using either `require('node:console').Console`
 * or `console.Console` (or their destructured counterparts).
 */
export declare class Console {
    /**
     * Creates a new `Console` with one or two writable stream instances.
     * `stdout` is a writable stream to print log or info output.
     * `stderr` is used for warning or error output.
     *
     * @param stdout A writable stream for log output
     * @param stderr A writable stream for error output
     * @param ignoreErrors Whether to ignore errors when writing
     */
    constructor(stdout: object, stderr?: object, ignoreErrors?: boolean);

    /**
     * A simple assertion test. If `condition` is falsy, an `AssertionError` is
     * written with an optional message.
     *
     * @param condition The condition to test
     * @param data Optional message or data
     */
    assert(condition?: boolean, ...data: any[]): void;

    /** Clears the console when possible. */
    clear(): void;

    /**
     * Maintains an internal counter and outputs to `stdout` the number of
     * times `count()` has been called with the given `label`.
     *
     * @param label The counter label
     */
    count(label?: string): void;

    /**
     * Resets the internal counter for the given `label`.
     *
     * @param label The counter label to reset
     */
    countReset(label?: string): void;

    /**
     * Prints to `stderr` with newline. Multiple arguments can be passed.
     *
     * @param data Values to output
     */
    debug(...data: any[]): void;

    /**
     * Prints to `stderr` with newline. Multiple arguments can be passed.
     *
     * @param data Values to output
     */
    error(...data: any[]): void;

    /**
     * Increases indentation of subsequent lines by spaces for `groupIndentation` length.
     *
     * @param data Optional label for the group
     */
    group(...data: any[]): void;

    /**
     * An alias for `group()`.
     *
     * @param data Optional label for the group
     */
    groupCollapsed(...data: any[]): void;

    /** Decreases indentation of subsequent lines. */
    groupEnd(): void;

    /**
     * Prints to `stdout` with newline. Multiple arguments can be passed.
     *
     * @param data Values to output
     */
    info(...data: any[]): void;

    /**
     * Prints to `stdout` with newline. Multiple arguments can be passed.
     * First argument is used as the primary message and additional arguments
     * are used as substitution values similar to `printf(3)`.
     *
     * @param data Values to output
     */
    log(...data: any[]): void;

    /**
     * Try to construct a table with the columns of the properties of
     * `tabularData` (or use `properties`) and rows of `tabularData`.
     *
     * @param tabularData Data to display as a table
     * @param properties Column headers to display
     */
    table(tabularData?: any, properties?: string[]): void;

    /**
     * Starts a timer identified by the given `label`.
     *
     * @param label The timer label
     */
    time(label?: string): void;

    /**
     * Stops a timer previously started with `time()` and prints the result.
     *
     * @param label The timer label
     */
    timeEnd(label?: string): void;

    /**
     * For a timer previously started with `time()`, prints the elapsed time
     * and other `data` arguments.
     *
     * @param label The timer label
     * @param data Additional values to output
     */
    timeLog(label?: string, ...data: any[]): void;

    /**
     * Prints to `stderr` the string `'Trace: '`, followed by `util.format()`
     * output of the message and current stack trace.
     *
     * @param data Values to output alongside the trace
     */
    trace(...data: any[]): void;

    /**
     * Prints to `stderr` with newline. Multiple arguments can be passed.
     *
     * @param data Values to output
     */
    warn(...data: any[]): void;

    /**
     * Starts a JavaScript CPU profile with an optional label.
     *
     * @param label The profile label
     */
    profile(label?: string): void;

    /**
     * Stops the current JavaScript CPU profiling session.
     *
     * @param label The profile label to stop
     */
    profileEnd(label?: string): void;

    /**
     * Adds an event with the label to the Timeline panel of the inspector.
     *
     * @param label The timestamp label
     */
    timeStamp(label?: string): void;
}
