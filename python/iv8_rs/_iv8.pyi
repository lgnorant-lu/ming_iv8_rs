"""
iv8-rs Python type stubs.

Type annotations for the iv8_rs._iv8 extension module.
Covers v0.1 (core), v0.2 (type conversion), v0.3 (observability), v0.4 (environment precision),
v0.5 (StructuredTrace, CFG, Taint Tracking, Crypto Detection, VM Diff, Module Isolation, CDP Scope, Probe).
"""

from __future__ import annotations

from collections.abc import Callable
from typing import Any

# --- Version ---

__version__: str

# --- Exceptions ---

class JSError(Exception):
    """Base class for JavaScript runtime errors."""
    ...

class JSCompileError(JSError):
    """JavaScript compilation/syntax error."""
    ...

class JSTimeoutError(JSError):
    """JavaScript execution timeout."""
    ...

class JSMemoryError(JSError):
    """JavaScript out-of-memory error."""
    ...

class JSPanic(JSError):
    """Internal Rust panic (should not occur in normal usage)."""
    ...

# --- Module-level functions ---

def enable_logging(level: str = "info") -> None:
    """
    Enable tracing/logging output.

    Can also be enabled via the ``IV8_LOG`` environment variable.

    Args:
        level: Log level. One of "trace", "debug", "info", "warn", "error".
    """
    ...


def instrument_source(
    source: str,
    mode: str = "auto",
    capture_stack_depth: int = 3,
    capture_env: bool = True,
    env_targets: list[str] | None = None,
    limit: int = 100000,
    handler_array: str | None = None,
    pc_var: str | None = None,
    stack_var: str | None = None,
    index_array: str | None = None,
    dispatch_pattern: str | None = None,
    expose_handlers: bool = False,
) -> tuple[str, dict[str, Any]]:
    """
    Detect JSVMP pattern and inject unified tracing code.

    Strategy:
    - Replaces the dispatch expression (e.g. ``A[Q[U++]]()``) with a logging wrapper
    - Prepends global object Proxies at source start (captures env reads with PC)

    Output format: "TYPE,PC,target,value" where TYPE is D/R/C/W.

    Args:
        source: The JS source code to instrument.
        mode: Detection mode. "auto" (default), "chaosvm", or "switch_vm".
        capture_stack_depth: Stack elements to capture per dispatch (default 3).
        capture_env: Whether to inject environment Proxy wrappers (default True).
        env_targets: Global objects to proxy (default: navigator, screen, document,
            location, Math, crypto, performance).
        limit: Maximum trace entries (default 100000).
        handler_array: Manual override for handler array variable name.
        pc_var: Manual override for program counter variable name.
        stack_var: Manual override for stack variable name.
        index_array: Manual override for index/bytecode array variable name.
        dispatch_pattern: Manual override for the dispatch expression string.
        expose_handlers: If True, assign handler table to globalThis.__iv8_vm_handlers__.

    Returns:
        Tuple of (patched_source, vm_info_dict).
        vm_info_dict keys: handler_array, index_array, pc_var, stack_var,
        mode, dispatch_pattern, dispatch_offset, head_code_length, …

    Raises:
        RuntimeError: If no JSVMP dispatch pattern detected and no manual params given.
    """
    ...


def prepare_entry(
    source: str,
    persona: str = "analysis",
    entry_targets: list[str] | None = None,
) -> dict[str, Any]:
    """Plan a single-source entry (EntryPlan-shaped dict). persona: runtime|analysis."""
    ...


def plan_multi_entry(
    sources: list[tuple[str, str]],
    persona: str = "analysis",
) -> dict[str, Any]:
    """Plan multiple named sources. sources: list of (name, source_text)."""
    ...


def run_with_entry(
    plan: dict[str, Any],
    source: str,
    chunks: list[str] | None = None,
    entry_expr: str | None = None,
) -> dict[str, Any]:
    """
    Execute a prepared entry plan.

    chunks: ordered JS source strings evaluated before source (no URL fetch).
    """
    ...


def trace_diff(trace_a: list[str], trace_b: list[str]) -> dict[str, Any]:
    """
    Compare two trace logs and find the first divergence point.

    Args:
        trace_a: First trace log (list of strings).
        trace_b: Second trace log (list of strings).

    Returns:
        Dict with divergence info:
        - index: position of first difference (-1 if identical)
        - a: entry from trace_a at divergence point (or None)
        - b: entry from trace_b at divergence point (or None)
        - total_a: length of trace_a
        - total_b: length of trace_b
        - match_count: number of matching entries before divergence
    """
    ...

# --- JSContext ---

class JSContext:
    """
    A JavaScript execution context backed by V8.

    Each instance owns a V8 Isolate and must be used from the thread that
    created it. Provides the full browser environment surface for JS execution.

    Example::

        ctx = JSContext()
        result = ctx.eval("1 + 1")  # -> 2
        ctx.close()

        # Or use as context manager:
        with JSContext() as ctx:
            result = ctx.eval("navigator.userAgent")
    """

    def __init__(
        self,
        environment: dict[str, Any] | None = None,
        config: dict[str, Any] | None = None,
        time_mode: str = "logical",
        js_api: str = "__iv8__",
        strict_compat: bool = True,
        random_seed: int | None = None,
        crypto_seed: int | None = None,
        time_freeze: float | None = None,
        worker_mode: bool = False,
    ) -> None:
        """
        Create a new JSContext.

        Args:
            environment: Browser environment overrides. Supports flat format
                (``{"navigator.userAgent": "..."}``) and nested format
                (``{"navigator": {"userAgent": "..."}}``, auto-flattened).
            config: Additional configuration. Supported keys:
                - timezone: IANA timezone string (e.g. "America/New_York")
                - locale: BCP 47 locale string (e.g. "zh-CN")
                - storage_path: optional path for storage persistence
            time_mode: "logical" (default, controlled clock) or "system" (real clock).
            js_api: Name of the internal tool object (default "__iv8__").
            strict_compat: If True (default), replicate iv8 0.1.2 behavior.
                If False, enable enhanced type conversions.
            random_seed: Seed for deterministic Math.random() (v0.3).
            crypto_seed: Seed for deterministic crypto.getRandomValues() (v0.3).
            time_freeze: Frozen timestamp in ms for Date.now() (v0.3).
            worker_mode: Worker-side construction path (native).
        """
        ...

    # --- Core eval ---

    def eval(
        self,
        source: str,
        /,
        name: str | None = None,
        line: int = -1,
        col: int = -1,
        to_py: bool = False,
        devtools: bool = True,
    ) -> Any:
        """
        Evaluate JavaScript source code and return the result.

        Type conversion (strict_compat=True):
            null/undefined -> None, boolean -> bool, int -> int, float -> float,
            string -> str, Array -> list, Object -> dict, ArrayBuffer -> bytes,
            BigInt -> None, Date -> "[object Date]"

        Type conversion (strict_compat=False):
            BigInt -> int, Date -> datetime.datetime, Map -> dict, Set -> set,
            TypedArray -> list[int|float]

        Args:
            source: JavaScript source code to evaluate.
            name: Optional source URL for stack traces.
            line: Line offset for error reporting.
            col: Column offset for error reporting.
            to_py: Ignored (always deep-converts).
            devtools: Ignored (automatic).

        Raises:
            JSCompileError: Syntax error in source.
            JSError: JavaScript exception thrown.
        """
        ...

    def eval_promise(self, source: str, max_ticks: int = 1000) -> Any:
        """
        Evaluate JavaScript and await the result if it's a Promise.

        Runs the event loop until the Promise settles.

        Args:
            source: JavaScript source code that returns a Promise.
            max_ticks: Maximum event loop ticks to wait (default 1000).
        """
        ...

    # --- Resource & Network ---

    def add_resource(
        self,
        url: str,
        body: str | bytes,
        status: int = 200,
        headers: dict[str, str] | None = None,
    ) -> None:
        """
        Add a resource to the offline bundle.

        When JS calls fetch(url) or XHR, the registered response is returned.
        """
        ...

    def set_network_handler(
        self,
        handler: Callable[[str, str], tuple[int, str | bytes] | None],
    ) -> None:
        """
        Set a Python network handler for fetch/XHR fallback.

        Three-layer chain: ResourceBundle -> Python handler -> NetworkError.
        Handler receives (url, method), returns (status, body) or None.
        """
        ...

    def clear_network_handler(self) -> None:
        """Clear the network handler (revert to offline-only mode)."""
        ...

    # --- DOM & Page ---

    def page_load(self, html: str, base_url: str | None = None) -> None:
        """
        Load an HTML page: parse DOM, execute scripts, fire DOMContentLoaded.

        After calling this, document.getElementById, querySelector, etc. work.
        """
        ...

    def page_load_with_headers(
        self,
        html: str,
        base_url: str | None = None,
        headers: dict[str, str] | None = None,
    ) -> None:
        """Load HTML with response header context (e.g. Set-Cookie)."""
        ...

    # --- Expose ---

    def expose(
        self,
        name_or_data: str | Any,
        callable_or_name: Any | None = None,
    ) -> None:
        """
        Expose a Python callable as a global JS function, or store data.

        Mode 1: expose(name, callable) -- registers a JS function.
        Mode 2: expose(data, name) -- stores data at __iv8__.data.name.
        """
        ...

    def expose_module(self, module: Any) -> None:
        """Expose all callable members of a Python module to JS global scope."""
        ...

    # --- Inspector / DevTools (v0.3 M15) ---

    def with_devtools(
        self,
        port: int = 9229,
        watch_apis: list[str] | None = None,
        enable_console: bool = True,
        wait: bool = True,
    ) -> JSContext:
        """
        Start the V8 Inspector (CDP WebSocket server).

        Args:
            port: WebSocket port (default 9229).
            watch_apis: List of API paths to auto-breakpoint on access.
            enable_console: Whether to enable DevTools console (default True).
            wait: Whether to wait for a DevTools client to connect (default True).
                Set to False for programmatic CDP use.

        Returns:
            self (for chaining).
        """
        ...

    def get_devtools_url(self) -> str | None:
        """Get the DevTools URL for the current inspector session."""
        ...

    def process_inspector_messages(self) -> None:
        """Process pending CDP messages (call periodically when debugging)."""
        ...

    # --- CDP Programmatic API (v0.3 M15) ---

    def cdp_set_breakpoint(
        self,
        url: str,
        line: int,
        column: int | None = None,
        condition: str | None = None,
    ) -> str:
        """
        Set a breakpoint by script URL.

        Args:
            url: Script URL (e.g. "tdc.js" or full URL).
            line: Line number (0-based).
            column: Column number (0-based, optional).
            condition: JS expression; breakpoint only fires when true.

        Returns:
            breakpoint_id (str) for later removal.

        Requires with_devtools() to have been called first.
        """
        ...

    def cdp_remove_breakpoint(self, breakpoint_id: str) -> None:
        """Remove a breakpoint by id."""
        ...

    def cdp_evaluate_on_frame(self, call_frame_id: str, expression: str) -> Any:
        """
        Evaluate an expression on a call frame while paused at a breakpoint.

        Args:
            call_frame_id: Frame ID from cdp_get_call_frames().
            expression: JS expression to evaluate in that frame's scope.
        """
        ...

    def cdp_resume(self) -> None:
        """Resume execution after a breakpoint pause."""
        ...

    def cdp_step_over(self) -> None:
        """Step over (next statement, skip function calls)."""
        ...

    def cdp_step_into(self) -> None:
        """Step into (enter function calls)."""
        ...

    def cdp_step_out(self) -> None:
        """Step out of the current function."""
        ...

    def cdp_get_call_frames(self) -> list[dict[str, Any]] | None:
        """
        Get call frames from the last breakpoint pause.

        Returns:
            List of frame dicts, or None if not paused.
            Each frame: {functionName, url, lineNumber, columnNumber, callFrameId}
        """
        ...

    def cdp_process_events(self) -> bool:
        """
        Process CDP events (check if execution paused at breakpoint).

        Returns:
            True if a Debugger.paused event was received.
        """
        ...

    def cdp_get_scope_properties(
        self,
        object_id: str,
        own_properties: bool = True,
    ) -> list[dict[str, Any]] | Any:
        """Get scope / object properties while paused (CDP Runtime.getProperties)."""
        ...

    # --- Trace Mode (v0.3 M16) ---

    def set_trace_point(
        self,
        url: str,
        line: int,
        column: int | None = None,
        expression: str = "'hit'",
    ) -> str:
        """
        Set a trace point: conditional breakpoint that records without pausing.

        The expression is evaluated each time the line is hit and pushed to
        an internal trace array. Call get_trace_log() after execution.

        Args:
            url: Script URL to set trace point in.
            line: Line number (0-based).
            column: Column number (0-based, optional).
            expression: JS expression to evaluate and record each hit.

        Returns:
            trace_point_id (str) for later removal.
        """
        ...

    def remove_trace_point(self, trace_point_id: str) -> None:
        """Remove a trace point by id."""
        ...

    def get_trace_log(self) -> list[Any]:
        """Get all entries recorded by trace points since last clear."""
        ...

    def clear_trace_log(self) -> None:
        """Clear the trace log."""
        ...

    def set_trace_limit(self, max_entries: int) -> None:
        """
        Set maximum trace log size. When reached, trace points stop recording.

        Args:
            max_entries: Maximum entries. Set to 0 to disable limit.
        """
        ...

    # --- Deterministic Mode (v0.3 M17) ---
    # (Configured via __init__ params: random_seed, crypto_seed, time_freeze)

    # --- VM-aware Helper (v0.3 M18) ---

    def detect_chaosvm_vars(self, source: str) -> dict[str, str] | None:
        """
        Detect ChaosVM/JSVMP variable names from JS source code.

        Searches for patterns like A[Q[U++]]() (handler_array[index_array[pc++]]()).

        Args:
            source: The JS source code to analyze.

        Returns:
            Dict with detected names: {handler_array, index_array, pc, stack},
            or None if no VM pattern found.
        """
        ...

    def instrument_chaosvm(
        self,
        handler_array: str,
        pc_var: str,
        stack_var: str,
        capture_stack_depth: int = 3,
        limit: int = 100000,
    ) -> None:
        """
        Instrument a ChaosVM handler array for high-performance tracing.

        Wraps the handler array with a Proxy that records every dispatch.
        Much faster than CDP breakpoints (~0.5s for 50000 instructions).

        After calling this, execute JS normally, then call get_vm_trace().

        Args:
            handler_array: Variable name of the handler array (e.g. "A").
            pc_var: Variable name of the program counter (e.g. "U").
            stack_var: Variable name of the stack (e.g. "S").
            capture_stack_depth: Stack top elements to capture (default 3).
            limit: Maximum trace entries (default 100000).
        """
        ...

    def get_vm_trace(self) -> list[str]:
        """
        Get the VM trace log (after instrument_chaosvm + execution).

        Returns:
            List of trace entry strings: "pc,opcode,stack0,stack1,..."
        """
        ...

    def clear_vm_trace(self) -> None:
        """Clear the VM trace log."""
        ...

    def uninstrument_chaosvm(self, handler_array: str) -> None:
        """Restore original handler array (undo instrument_chaosvm)."""
        ...

    def detect_vm_dispatch(
        self,
        script_url: str,
        patterns: list[str] | None = None,
    ) -> dict[str, Any] | None:
        """
        Detect a JSVMP dispatch loop in a loaded script.

        Args:
            script_url: URL of the script to search in.
            patterns: Optional list of search strings.

        Returns:
            Dict with {url, line, column, pattern} if found, None otherwise.
            Requires with_devtools() for CDP search.
        """
        ...

    def trace_vm(
        self,
        url: str,
        line: int,
        column: int | None = None,
        vars: list[str] | None = None,
        limit: int = 50000,
    ) -> str:
        """
        High-level VM trace: set trace point with structured capture.

        Combines set_trace_point + expression building. After eval, call
        get_trace_log() to get results.

        Args:
            url: Script URL.
            line: Dispatch loop line number.
            column: Column (optional).
            vars: JS expressions to capture (default: ["pc", "H[pc]"]).
            limit: Max trace entries (default 50000).

        Returns:
            trace_point_id (str).
        """
        ...

    # --- M19: Deep Trace (unified instrument_source integration) ---

    def get_unified_trace(self) -> list[str]:
        """
        Get the unified trace log (from instrument_source injection).

        Entry format: "TYPE,PC,target,value"
        - D,pc,opcode,stack_depth -- VM dispatch
        - R,pc,obj.prop,value -- Environment read
        - C,pc,obj.method,result -- Function call
        - W,pc,obj.prop,value -- Property write
        """
        ...

    def clear_unified_trace(self) -> None:
        """Clear the unified trace log."""
        ...

    # --- Recording (v0.3 M19) ---

    def start_recording(
        self,
        targets: list[str] | None = None,
        record_reads: bool = True,
        record_writes: bool = True,
        record_calls: bool = True,
        limit: int = 50000,
    ) -> None:
        """
        Start recording all property reads/writes/calls on global objects.

        Args:
            targets: Global object names to monitor (default: navigator, screen,
                document, location, Math, crypto, performance).
            record_reads: Record property reads (default True).
            record_writes: Record property writes (default True).
            record_calls: Record function calls (default True).
            limit: Maximum entries (default 50000).
        """
        ...

    def stop_recording(self) -> list[str]:
        """
        Stop recording and return all captured entries.

        Returns:
            List of entry strings: "TYPE,target.prop,value"
            TYPE: R=read, W=write, C=call.
        """
        ...

    # --- Profiler & Coverage (v0.3 M19) ---

    def start_profiler(self) -> None:
        """
        Start V8 CPU Profiler (function-level call graph).

        Requires with_devtools(wait=False) to have been called.
        """
        ...

    def stop_profiler(self) -> dict[str, Any] | None:
        """
        Stop V8 CPU Profiler and return the profile data.

        Returns:
            Dict with V8 CPU Profile format (nodes, startTime, endTime, samples),
            or None if profiler was not started.
        """
        ...

    def start_coverage(self) -> None:
        """
        Start precise code coverage collection.

        Requires with_devtools(wait=False).
        """
        ...

    def stop_coverage(self) -> list[dict[str, Any]] | None:
        """
        Stop coverage collection and return results.

        Returns:
            List of script coverage dicts (scriptId, url, functions with ranges).
        """
        ...

    # --- Console ---

    def get_console_messages(self) -> list[dict[str, str]]:
        """
        Get all console messages captured since context creation.

        Returns:
            List of dicts with 'level' and 'text' keys.
        """
        ...

    def clear_console_messages(self) -> None:
        """Clear all captured console messages."""
        ...

    # --- Storage ---

    def persist_storage(self, path: str) -> None:
        """Persist storage state to path."""
        ...

    def load_storage(self, path: str) -> None:
        """Load storage state from path."""
        ...

    # --- Worker internal ---

    def set_worker_prototype(self) -> None:
        """
        Set globalThis.__proto__ to DedicatedWorkerGlobalScope.prototype.

        Worker construction path; not a general application API.
        """
        ...

    # --- Lifecycle ---

    def is_disposed(self) -> bool:
        """Check if the context has been disposed."""
        ...

    def close(self) -> None:
        """Close the context and release V8 resources."""
        ...

    def __enter__(self) -> JSContext: ...
    def __exit__(
        self,
        exc_type: type | None,
        exc_val: BaseException | None,
        exc_tb: Any | None,
    ) -> bool: ...

    @classmethod
    def get_defaults(cls) -> dict[str, Any]:
        """Return the 393 default environment entries as a dict."""
        ...

# --- Debugger ---

class Debugger:
    """
    Runtime analysis assistant for a JSContext.

    Provides lightweight instrumentation via hookNative:
    - API call tracing
    - Property watching (read/write interception)
    - Environment snapshot
    - Call log capture and summary

    Example::

        dbg = Debugger(ctx)
        dbg.trace_api('Math.random')
        ctx.eval('Math.random(); Math.random();')
        log = dbg.get_call_log()
    """

    def __init__(self, ctx: JSContext) -> None:
        """Create a Debugger attached to a JSContext."""
        ...

    def trace_api(self, api_path: str) -> None:
        """
        Trace all calls to a JS API path.

        Args:
            api_path: Dot-path like 'Math.random', 'document.getElementById'.
        """
        ...

    def trace_apis(self, api_paths: list[str]) -> None:
        """Trace multiple APIs at once."""
        ...

    def get_call_log(self) -> list[dict[str, Any]]:
        """
        Get the call log.

        Each entry: {api, args, result, timestamp}.
        """
        ...

    def clear_call_log(self) -> None:
        """Clear the call log."""
        ...

    def get_traced_apis(self) -> list[str]:
        """Get the list of currently traced APIs."""
        ...

    def eval_traced(self, source: str) -> tuple[Any, list[dict[str, Any]]]:
        """
        Evaluate JS and return both the result and the call log.

        Clears the log before evaluation, then captures all traced calls.

        Returns:
            Tuple of (result, call_log_entries).
        """
        ...

    def snapshot(self) -> dict[str, Any]:
        """
        Get a snapshot of the current environment.

        Returns dict with key environment properties: userAgent, platform,
        language, hardwareConcurrency, screenWidth, screenHeight, hasChrome,
        hasCrypto, performanceNow, dateNow, documentURL, etc.
        """
        ...

    def watch_property(
        self,
        obj_path: str,
        prop: str,
        mode: str = "both",
    ) -> None:
        """
        Install a watch on a property -- logs every read/write.

        Args:
            obj_path: Path to the object (e.g. 'navigator', 'document').
            prop: Property name to watch (e.g. 'userAgent', 'cookie').
            mode: 'read', 'write', or 'both' (default 'both').
        """
        ...

    def get_call_summary(self) -> dict[str, int]:
        """Get a summary of call counts per API."""
        ...

    def schedule_pause(self) -> None:
        """Schedule a pause on the next JS statement (requires DevTools)."""
        ...

    def __repr__(self) -> str: ...
