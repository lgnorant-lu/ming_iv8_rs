"""
iv8-rs Python type stubs.

Type annotations for the iv8_rs._iv8 extension module.
"""

from __future__ import annotations
from typing import Any, Callable, Optional, Union, List, Dict, Tuple

# ─── Version ──────────────────────────────────────────────────────────────────

__version__: str

# ─── Exceptions ───────────────────────────────────────────────────────────────

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

# ─── JSContext ────────────────────────────────────────────────────────────────

class JSContext:
    """
    A JavaScript execution context backed by V8.

    Each instance owns a V8 Isolate and must be used from the thread that created it.

    Example::

        ctx = JSContext()
        result = ctx.eval("1 + 1")  # → 2
        ctx.close()

        # Or use as context manager:
        with JSContext() as ctx:
            result = ctx.eval("navigator.userAgent")
    """

    def __init__(
        self,
        environment: Optional[Dict[str, Any]] = None,
        config: Optional[Dict[str, Any]] = None,
        time_mode: str = "logical",
        js_api: str = "__iv8__",
        strict_compat: bool = True,
    ) -> None:
        """
        Create a new JSContext.

        Args:
            environment: Browser environment overrides. Supports both flat format
                (``{"navigator.userAgent": "..."}```) and nested format
                (``{"navigator": {"userAgent": "..."}}``, auto-flattened).
            config: Additional configuration dict. Supported keys:
                - ``timezone``: IANA timezone string (e.g. ``"America/New_York"``)
                - ``locale``: BCP 47 locale string (e.g. ``"zh-CN"``)
            time_mode: Time mode. ``"logical"`` (default) uses a controlled clock
                that advances only when explicitly advanced. ``"system"`` uses
                the real system clock.
            js_api: Name of the internal tool object (default ``"__iv8__"``).
            strict_compat: If True (default), replicate iv8 0.1.2 behavior including
                known bugs. If False, enable enhanced behavior.
        """
        ...

    def eval(
        self,
        source: str,
        /,
        name: Optional[str] = None,
        line: int = -1,
        col: int = -1,
        to_py: bool = False,
        devtools: bool = True,
    ) -> Any:
        """
        Evaluate JavaScript source code and return the result as a Python object.

        Type conversion (JS → Python):
            - ``null`` / ``undefined`` → ``None``
            - ``boolean`` → ``bool``
            - integer number → ``int``
            - float number → ``float``
            - ``string`` → ``str``
            - ``Array`` → ``list``
            - ``Object`` → ``dict``
            - ``ArrayBuffer`` / ``TypedArray`` → ``bytes``

        Args:
            source: JavaScript source code to evaluate.
            name: Optional source URL for stack traces.
            line: Line offset for error reporting.
            col: Column offset for error reporting.
            to_py: Ignored (always deep-converts, matching iv8 ``to_py=True``).
            devtools: Ignored (DevTools integration is automatic).

        Returns:
            The JavaScript result converted to a Python object.

        Raises:
            JSCompileError: If the source has a syntax error.
            JSError: If the JavaScript throws an exception.
            JSTimeoutError: If execution times out.
        """
        ...

    def eval_promise(self, source: str, max_ticks: int = 1000) -> Any:
        """
        Evaluate JavaScript and await the result if it's a Promise.

        Runs the event loop until the Promise settles (up to ``max_ticks``
        iterations). Returns the resolved value.

        Args:
            source: JavaScript source code that returns a Promise.
            max_ticks: Maximum event loop ticks to wait (default 1000).

        Returns:
            The resolved Promise value converted to a Python object.

        Raises:
            JSError: If the Promise rejects or the source throws.
        """
        ...

    def page_load(self, html: str, base_url: Optional[str] = None) -> None:
        """
        Load an HTML page: parse DOM, execute inline scripts, fire DOMContentLoaded.

        This is the primary way to set up a browser environment for JS execution.
        After calling this, ``document.getElementById``, ``querySelector``, etc.
        are available.

        Args:
            html: The HTML source to parse.
            base_url: Optional base URL for resolving relative URLs and setting
                ``location.href``.
        """
        ...

    def expose(
        self,
        name_or_data: Union[str, Any],
        callable_or_name: Optional[Any] = None,
    ) -> None:
        """
        Expose a Python callable as a global JS function, or store data.

        Two modes:

        **Mode 1: expose(name, callable)** — registers a JS function::

            def my_func(x, y):
                return x + y
            ctx.expose("myFunc", my_func)
            ctx.eval("myFunc(1, 2)")  # → 3

        **Mode 2: expose(data, name)** — stores data at ``__iv8__.data.name``::

            ctx.expose({"html": "...", "resources": {}}, "s1")
            ctx.eval("__iv8__.page.load(__iv8__.data.s1)")

        Args:
            name_or_data: Either the function name (str) or data dict.
            callable_or_name: Either the callable or the data name.
        """
        ...

    def expose_module(self, module: Any) -> None:
        """
        Expose all callable members of a Python module to JS global scope.

        If the module has ``__all__``, only those names are exposed.
        Otherwise, all public callables (not starting with ``_``) are exposed.

        Args:
            module: A Python module object.
        """
        ...

    def add_resource(
        self,
        url: str,
        body: Union[str, bytes],
        status: int = 200,
        headers: Optional[Dict[str, str]] = None,
    ) -> None:
        """
        Add a resource to the offline bundle.

        When JS later calls ``fetch(url)`` or ``XMLHttpRequest``, the registered
        response is returned.

        Args:
            url: The URL to register.
            body: Response body (str or bytes).
            status: HTTP status code (default 200).
            headers: Optional response headers dict.
        """
        ...

    def set_network_handler(
        self,
        handler: Callable[[str, str], Optional[Tuple[int, Union[str, bytes]]]],
    ) -> None:
        """
        Set a Python network handler for fetch/XHR fallback.

        Runs as the second tier of the three-layer chain:

        1. ResourceBundle (pre-registered offline responses)
        2. Python handler (this) — always called when a URL is not in the
           bundle, regardless of ``strict_compat`` mode
        3. NetworkError (offline default when handler returns ``None``)

        The handler receives ``(url: str, method: str)`` and should return:
        - ``(status: int, body: str | bytes)`` to provide a response
        - ``None`` to fall through to NetworkError

        Both ``fetch()`` and synchronous XMLHttpRequest call the handler.
        For asynchronous XHR, the handler is invoked when the event loop
        drains the timer queue.

        Example::

            def handler(url, method):
                if 'api.example.com' in url:
                    return (200, '{"ok": true}')
                return None
            ctx.set_network_handler(handler)

        Args:
            handler: Callable that takes (url, method) and returns response or None.
        """
        ...

    def clear_network_handler(self) -> None:
        """Clear the network handler (revert to offline-only mode)."""
        ...

    def with_devtools(
        self,
        port: int = 9229,
        watch_apis: Optional[List[str]] = None,
        enable_console: bool = True,
    ) -> "JSContext":
        """
        Start the V8 Inspector (CDP WebSocket server).

        Opens a DevTools debugging session. Returns self for chaining::

            ctx = JSContext().with_devtools(port=9229, watch_apis=["Math.random"])

        Args:
            port: WebSocket port (default 9229).
            watch_apis: List of API paths to auto-breakpoint on access.
            enable_console: Whether to enable DevTools console (default True).

        Returns:
            self (for chaining).
        """
        ...

    def get_devtools_url(self) -> Optional[str]:
        """Get the DevTools URL for the current inspector session."""
        ...

    def process_inspector_messages(self) -> None:
        """Process pending CDP messages (call periodically when debugging)."""
        ...

    def get_console_messages(self) -> List[Dict[str, str]]:
        """
        Get all console messages captured since context creation.

        Returns:
            List of dicts with ``'level'`` and ``'text'`` keys.
            Levels: ``'log'``, ``'info'``, ``'warn'``, ``'error'``, ``'debug'``,
            ``'trace'``, ``'assert'``.
        """
        ...

    def clear_console_messages(self) -> None:
        """Clear all captured console messages."""
        ...

    def is_disposed(self) -> bool:
        """Check if the context has been disposed."""
        ...

    def close(self) -> None:
        """Close the context and release V8 resources."""
        ...

    def __enter__(self) -> "JSContext":
        """Context manager entry — returns self."""
        ...

    def __exit__(
        self,
        exc_type: Optional[type],
        exc_val: Optional[BaseException],
        exc_tb: Optional[Any],
    ) -> bool:
        """Context manager exit — closes the context."""
        ...

    @classmethod
    def get_defaults(cls) -> Dict[str, Any]:
        """
        Return the 393 default environment entries as a dict.

        Useful for inspecting or customizing the default browser fingerprint.
        """
        ...

# ─── Debugger ─────────────────────────────────────────────────────────────────

class Debugger:
    """
    Runtime analysis assistant for a JSContext.

    Provides lightweight instrumentation for JS reverse engineering:
    - API call tracing via hookNative
    - Property watching (read/write interception)
    - Environment snapshot
    - Call log capture and summary

    Example::

        dbg = Debugger(ctx)
        dbg.trace_api('Math.random')
        ctx.eval('Math.random(); Math.random();')
        log = dbg.get_call_log()
        # [{'api': 'Math.random', 'args': '[]', 'result': '0.42', 'timestamp': 0.0}]
    """

    def __init__(self, ctx: JSContext) -> None:
        """
        Create a Debugger attached to a JSContext.

        Args:
            ctx: The JSContext to attach to.
        """
        ...

    def trace_api(self, api_path: str) -> None:
        """
        Trace all calls to a JS API path.

        Installs a hookNative interceptor that records every call.

        Args:
            api_path: Dot-path like ``'Math.random'``, ``'document.getElementById'``,
                ``'fetch'``.

        Example::

            dbg.trace_api('Math.random')
            ctx.eval('Math.random()')
            log = dbg.get_call_log()
        """
        ...

    def trace_apis(self, api_paths: List[str]) -> None:
        """
        Trace multiple APIs at once.

        Args:
            api_paths: List of dot-paths to trace.
        """
        ...

    def get_call_log(self) -> List[Dict[str, Any]]:
        """
        Get the call log as a list of dicts.

        Each entry has:
        - ``api``: The API path that was called.
        - ``args``: JSON-serialized arguments.
        - ``result``: JSON-serialized return value.
        - ``timestamp``: ``performance.now()`` at call time.

        Returns:
            List of call log entries.
        """
        ...

    def clear_call_log(self) -> None:
        """Clear the call log."""
        ...

    def get_traced_apis(self) -> List[str]:
        """Get the list of currently traced APIs."""
        ...

    def eval_traced(self, source: str) -> Tuple[Any, List[Dict[str, Any]]]:
        """
        Evaluate JS and return both the result and the call log.

        Clears the log before evaluation, then captures all traced calls.

        Args:
            source: JavaScript source code.

        Returns:
            Tuple of (result, call_log_entries).
        """
        ...

    def snapshot(self) -> Dict[str, Any]:
        """
        Get a snapshot of the current environment.

        Returns a dict with key environment properties including:
        ``userAgent``, ``platform``, ``language``, ``hardwareConcurrency``,
        ``screenWidth``, ``screenHeight``, ``hasChrome``, ``hasCrypto``,
        ``performanceNow``, ``dateNow``, ``documentURL``, etc.

        Returns:
            Dict of environment properties.
        """
        ...

    def watch_property(
        self,
        obj_path: str,
        prop: str,
        mode: str = "both",
    ) -> None:
        """
        Install a watch on a property — logs every read/write.

        Args:
            obj_path: Path to the object (e.g. ``'navigator'``, ``'document'``).
            prop: Property name to watch (e.g. ``'userAgent'``, ``'cookie'``).
            mode: ``'read'``, ``'write'``, or ``'both'`` (default ``'both'``).
        """
        ...

    def get_call_summary(self) -> Dict[str, int]:
        """
        Get a summary of call counts per API.

        Returns:
            Dict mapping API path to call count.
        """
        ...

    def schedule_pause(self) -> None:
        """
        Schedule a pause on the next JS statement.

        Requires DevTools to be connected (``ctx.with_devtools()``).
        """
        ...

    def __repr__(self) -> str: ...

# ─── Logging ──────────────────────────────────────────────────────────────────

def enable_logging(level: str = "info") -> None:
    """
    Enable tracing/logging output.

    Can also be enabled via the ``IV8_LOG`` environment variable.

    Args:
        level: Log level. One of ``"trace"``, ``"debug"``, ``"info"``,
            ``"warn"``, ``"error"``. Default ``"info"``.
    """
    ...
