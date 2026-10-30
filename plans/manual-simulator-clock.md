# Observe simulator stages without wall-clock races

Fork issue #9; clean baseline 81825c4. Integrated Rust 1.95 qualification on
91e9ac2 failed waiting for Initial after the fast simulator had already homed to
Idle. A fixed number of UI frames plus a 0.15-second simulated person cannot
establish observation of every stage under load. Other platforms passed; that
does not rule out the source-level timing race.

Add an opt-in manually advanced clock to the existing Sim. Default construction
and automatic progression remain unchanged. All state timestamps use the same
clock; explicit advancement is checked/fallible and rejects real-time instances.
No new crate, feature, transport or protocol is needed. Exercise the manual clock
through an actual Session and use it only for the whole-job UI test, allowing a
state to advance after its UI has been checked. Keep homing, trace, start, thread,
progress and completion assertions. Bound UI waiting by elapsed time rather than
1500 potentially expensive frames. Do not increase delays to hide the race.

Proof: existing automatic simulator tests, a manual Session lifecycle test,
the whole-job UI test, and integrated Windows/Rust 1.95/Linux qualification.
The opt-in clock is simulator/test support, not physical-machine qualification.
Rollback: this topic independently. No upstream publication is authorized.

Windows standalone proof: all five simulator tests and seven sewing-window tests
pass. The manual Session test covers homing, trace, start, thread change and
completion plus overflow rejection and real-time-mode rejection. Replacing only
stage timestamps with Instant::now makes it fail (`trace started without
advancement`), demonstrating that all timestamps must share the manual clock.
Existing automatic simulator tests still pass. Integrated proof follows.
