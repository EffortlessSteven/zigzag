# Wait for the actual sent-design recovery writer

Fork issue #9; baseline 81825c4. The restoration test polls for sent.json for
200 five-millisecond intervals, then proceeds even if the writer did not finish.
Previous full runs have failed restoration and passed isolated retries. That
does not by itself establish why every old run failed, but the missing completion
precondition is explicit in the source.

Keep production persistence asynchronous. Extract a private task-returning seam
under the existing keep_sent wrapper, preserving synchronous fallback when thread
creation fails. The test joins the actual task, failing on a writer panic, before
checking all existing UUID, PEN stream, stitch-count, recognition and forgetting
assertions. No new timeout, filesystem abstraction or public API is needed.

Proof: session tests on this standalone branch, then integrated supported-toolchain
qualification. This is completion/preservation evidence, not a claimed regression
of every historical intermittency. Do not change file formats, write mechanics,
error UX or machine authorization. Roll back this topic independently. Fork only.

All nine standalone Windows session tests PASS with the joined writer.
Production still drops the handle and does not wait on the UI thread.
