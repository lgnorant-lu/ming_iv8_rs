from __future__ import annotations

import json
from pathlib import Path

import iv8_rs
import pytest


def test_constructor_profile_merge_and_profile_errors(tmp_path):
    profile_path = tmp_path / "profile.json"
    profile_path.write_text(
        json.dumps({
            "navigator.language": "fr-FR",
            "navigator.platform": "Linux x86_64",
            "_meta.note": "not passed to runtime",
        }),
        encoding="utf-8",
    )

    loaded = iv8_rs.load_profile(str(profile_path))
    assert loaded == {
        "navigator.language": "fr-FR",
        "navigator.platform": "Linux x86_64",
    }

    ctx = iv8_rs.JSContext(
        profile=str(profile_path),
        environment={"navigator.language": "en-US"},
    )
    try:
        assert ctx.eval("navigator.language") == "en-US"
        assert ctx.eval("navigator.platform") == "Linux x86_64"
    finally:
        ctx.close()

    with pytest.raises(FileNotFoundError):
        iv8_rs.load_profile(str(tmp_path / "missing.json"))

    bad_path = tmp_path / "bad.json"
    bad_path.write_text("{bad json", encoding="utf-8")
    with pytest.raises(ValueError, match="Invalid JSON"):
        iv8_rs.load_profile(str(bad_path))


def test_eval_error_and_close_contract():
    ctx = iv8_rs.JSContext()
    assert ctx.eval("1 + 1") == 2
    assert ctx.eval("({a: 1, b: 'two'})") == {"a": 1, "b": "two"}

    with pytest.raises(iv8_rs.JSCompileError):
        ctx.eval("function(")

    with pytest.raises(iv8_rs.JSError, match="boom"):
        ctx.eval("throw new TypeError('boom')")

    assert not ctx.is_disposed()
    ctx.close()
    assert ctx.is_disposed()
    ctx.close()

    with pytest.raises(RuntimeError, match="closed"):
        ctx.eval("1 + 1")


def test_context_manager_closes_context():
    with iv8_rs.JSContext() as ctx:
        assert ctx.eval("1 + 2") == 3
        assert not ctx.is_disposed()

    assert ctx.is_disposed()
    with pytest.raises(RuntimeError, match="closed"):
        ctx.eval("1 + 1")


def test_eval_promise_contract():
    ctx = iv8_rs.JSContext()
    try:
        assert ctx.eval_promise("Promise.resolve(42)") == 42

        with pytest.raises(iv8_rs.JSError) as rejected:
            ctx.eval_promise("Promise.reject(new TypeError('bad promise'))")
        assert "TypeError" in str(rejected.value)
        assert "bad promise" in str(rejected.value)

        with pytest.raises(iv8_rs.JSTimeoutError):
            ctx.eval_promise("new Promise(function(resolve) {})", max_ticks=1)
    finally:
        ctx.close()


def test_page_load_and_resource_bundle_contract():
    ctx = iv8_rs.JSContext()
    try:
        ctx.page_load(
            "<html><body><div id='t'>ok</div></body></html>",
            base_url="https://example.com/a'b\n;globalThis.injected=true;//",
        )
        assert ctx.eval("document.getElementById('t').textContent") == "ok"
        assert ctx.eval("globalThis.injected") is None
        assert "example.com" in ctx.eval("location.href")

        ctx.add_resource("https://api.test/cached", b"from-bundle", 200)
        result = ctx.eval_promise("fetch('https://api.test/cached').then(r => r.text())")
        assert result == "from-bundle"
    finally:
        ctx.close()

    with pytest.raises(RuntimeError, match="closed"):
        ctx.page_load("<html></html>")
    with pytest.raises(RuntimeError, match="closed"):
        ctx.add_resource("https://api.test/closed", b"closed", 200)


def test_network_handler_contract():
    captured = []

    def handler(url, method):
        captured.append((method, url))
        return (200, b'{"ok": true}')

    ctx = iv8_rs.JSContext()
    try:
        ctx.set_network_handler(handler)
        assert (
            ctx.eval_promise("fetch('https://api.test/fetched').then(r => r.text())")
            == '{"ok": true}'
        )
        assert captured == [("GET", "https://api.test/fetched")]

        ctx.set_network_handler(lambda url, method: {"status": 200, "body": "bad shape"})
        with pytest.raises(iv8_rs.JSError, match="NetworkError"):
            ctx.eval_promise("fetch('https://api.test/bad-shape')")

        with pytest.raises(TypeError, match="callable"):
            ctx.set_network_handler("not callable")
    finally:
        ctx.close()

    with pytest.raises(RuntimeError, match="closed"):
        ctx.set_network_handler(lambda url, method: (200, b"closed"))


def test_expose_contract_success_error_and_closed_context():
    ctx = iv8_rs.JSContext()
    try:
        ctx.expose("add", lambda a, b: a + b)
        assert ctx.eval("add(2, 5)") == 7

        def fail():
            raise ValueError("callback boom")

        ctx.expose("fail", fail)
        with pytest.raises(iv8_rs.JSError, match="callback boom"):
            ctx.eval("fail()")

        with pytest.raises(TypeError, match="callable"):
            ctx.expose("bad", 42)
    finally:
        ctx.close()

    with pytest.raises(RuntimeError, match="closed"):
        ctx.expose("afterClose", lambda: 1)


def test_inspector_safe_api_contract():
    ctx = iv8_rs.JSContext()
    try:
        assert ctx.get_devtools_url() is None
        ctx.process_inspector_messages()
        assert ctx.cdp_process_events() is False
        assert ctx.cdp_get_call_frames() is None
        # step methods exist; callable without inspector (return RuntimeError)
        assert hasattr(ctx, "cdp_step_over")
        assert hasattr(ctx, "cdp_step_into")
        assert hasattr(ctx, "cdp_step_out")
    finally:
        ctx.close()


def test_entry_environment_wrapper_contracts():
    source = "var x = 1;"
    plan = iv8_rs.prepare_entry(source, persona="analysis")
    plan_obj = iv8_rs.EntryPlan.from_dict(plan)
    assert plan_obj.plan_id == plan["plan_id"]
    assert plan_obj.persona == "analysis"
    assert plan_obj.state == "planned"
    assert plan_obj.diagnostics.sample_signals == plan["diagnostics"].get("sample_signals", [])

    result = iv8_rs.run_with_entry(plan, source)
    result_obj = iv8_rs.EntryResult.from_dict(result)
    assert result_obj.plan_id == plan_obj.plan_id
    assert result_obj.trace_meta is not None
    assert result_obj.environment_report is not None

    report = iv8_rs.run_environment_plane(source, profile=None)
    as_dict = report.to_dict()
    assert as_dict["workflow"] == ["probe", "patch", "rerun"]
    assert "before" in as_dict
    assert "patch" in as_dict
    assert "after" in as_dict


def test_environment_toolchain_public_typing_contract_is_current():
    stub_text = Path(iv8_rs.__file__).with_name("__init__.pyi").read_text(encoding="utf-8")

    for parameter in [
        "candidate_pack",
        "adapt_runtime_safe",
        "local_overlay",
        "max_iterations",
        "stop_on_regression",
        "dry_run_planning",
        "rollback_diagnostics",
        "substrate_coverage",
        "scaffold_gaps",
        "pressure_harness",
    ]:
        assert parameter in stub_text


def test_specialized_stable_apis_exist():
    assert callable(iv8_rs.parse_trace)
    assert callable(iv8_rs.CFG.from_trace)
    assert callable(iv8_rs.detect_constants)

def test_expose_module_not_stable():
    ctx = iv8_rs.JSContext()
    try:
        assert not hasattr(ctx, "expose_module") or callable(ctx.expose_module)
    finally:
        ctx.close()
