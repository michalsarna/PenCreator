# Testing with the GRBL simulator

The simulator (grbl-sim: the real GRBL 1.1h firmware compiled for the PC), the harness of the tests and the tests of the
serial link are in baseCreator: see `docs/grbl-sim.md` there.

PenCreator's own tests (`crates/pencreator/src/grbl_sim_tests.rs`) run the jobs of the plotter against it: a drawing with the
pen on the Z axis, one with a servo on the spindle output (`M3 S…`, off at the end), and a job with a pen change (`M0` stops
the job until it is told to go on). They are ignored in a normal `cargo test`; build the simulator with
`tools/grbl-sim/build.sh` of baseCreator into `target/grbl-sim` (or point `GRBL_SIM_EXE` at a build) and run

```sh
cargo test -p pencreator grbl_sim -- --ignored --test-threads=1
```

The *Tests* workflow builds it with the `grbl-sim` action of baseCreator and runs them.
