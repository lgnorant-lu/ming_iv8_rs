Core API Reference
==================

.. warning::

   This is an auto-generated reference page. For complete calling contracts
   (host bounds, thread safety, offline networking), see
   :doc:`the hand-written contracts <index>`.

Module-level functions
-----------------------

.. autofunction:: iv8_rs.enable_logging
.. autofunction:: iv8_rs.instrument_source
.. autofunction:: iv8_rs.trace_diff
.. autofunction:: iv8_rs.prepare_entry
.. autofunction:: iv8_rs.plan_multi_entry
.. autofunction:: iv8_rs.run_with_entry
.. autofunction:: iv8_rs.load_profile

Exceptions
----------

.. autoexception:: iv8_rs.JSError
.. autoexception:: iv8_rs.JSCompileError
.. autoexception:: iv8_rs.JSTimeoutError
.. autoexception:: iv8_rs.JSMemoryError
.. autoexception:: iv8_rs.JSPanic

Factory
-------

.. autofunction:: iv8_rs.JSContext

Native context (methods)
------------------------

.. autoclass:: iv8_rs._iv8.JSContext
   :members:
   :undoc-members:
   :show-inheritance:

Debugger
--------

.. autoclass:: iv8_rs.Debugger
   :members:
   :undoc-members:
   :show-inheritance:
