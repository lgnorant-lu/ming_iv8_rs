"""Sphinx configuration for iv8-rs."""

import os
import sys

# --- Path setup ---
# iv8_rs must be importable (built extension in this repo)
_repo_root = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", ".."))
if _repo_root not in sys.path:
    sys.path.insert(0, _repo_root)

# Ensure we find the built extension
_python_path = os.path.join(_repo_root, "python")
if _python_path not in sys.path:
    sys.path.insert(0, _python_path)

# --- Project info ---
project = "iv8-rs"
copyright = "2026, iv8 contributors"
author = "iv8 contributors"
release = "0.8.12"

# --- Extensions ---
extensions = [
    "sphinx.ext.autodoc",
    "sphinx.ext.autosummary",
    "sphinx.ext.napoleon",
    "sphinx.ext.viewcode",
    "sphinx_rtd_theme",
]

# --- Napoleon settings ---
napoleon_google_docstring = True
napoleon_numpy_docstring = True
napoleon_include_init_with_doc = True
napoleon_include_private_with_doc = False
napoleon_include_special_with_doc = True
napoleon_use_admonition_for_examples = True
napoleon_use_param = True
napoleon_use_rtype = True

# --- Autodoc settings ---
autodoc_default_options = {
    "members": True,
    "undoc-members": False,
    "show-inheritance": True,
    "special-members": "__init__",
    "imported-members": False,
}
autodoc_mock_imports = []

# --- Theme ---
html_theme = "sphinx_rtd_theme"
html_static_path = []
html_title = f"iv8-rs {release}"

# --- Autosummary ---
autosummary_generate = True
