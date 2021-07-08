# Conflict of Rust "features" with resolver v2

This repository proves experimentally that even with [Rust's feature resolver
v2](https://doc.rust-lang.org/cargo/reference/resolver.html#feature-resolver-version-2),
it is possible to have a conflict. This means that all features should still be
*additive* in order to avoid these cases.

Please let me know if there's anything wrong in the code, but it's quite simple.
The main crate, `resolver-v2-conflict` has two dependencies: `async-dep` and
`sync-dep`. Both of these depend on `problematic-crate` but with different
features (`async` and `sync`, respectively). It turns out that these features
are conflictive instead of additive (as it would be with
[`maybe_async`](https://docs.rs/maybe-async/)), so it's impossible to compile
`resolver-v2-conflict`.

There is some confusion about the resolver v2 regarding that it's supposed to
fix this case, but in reality it's not. You can use the `toggle-resolver` script
to switch between the resolver 1 and the resolver 2 easily:

```commandline
$ # First attempt with v1
$ cargo build
   Compiling problematic-crate v0.1.0 (/home/mario/Programming/resolver-v2-conflict/problematic-crate)
error: Conflict found.
 --> problematic-crate/src/lib.rs:2:1
  |
2 | / compile_error! {
3 | |     "Conflict found."
4 | | }
  | |_^

error: aborting due to previous error

error: could not compile `problematic-crate`

To learn more, run the command again with --verbose.

$ # Switching to v2
$ ./toggle-resolver.sh

$ # It will fail as well:
$ cargo build
   Compiling problematic-crate v0.1.0 (/home/mario/Programming/resolver-v2-conflict/problematic-crate)
error: Conflict found.
 --> problematic-crate/src/lib.rs:2:1
  |
2 | / compile_error! {
3 | |     "Conflict found."
4 | | }
  | |_^

error: aborting due to previous error

error: could not compile `problematic-crate`

To learn more, run the command again with --verbose.
```

## Explanation

As discussed in [this `maybe_async`'s
issue](https://github.com/fMeow/maybe-async-rs/issues/6), the only cases in
which the resolver avoids conflicts are the following:

* Features for target-specific dependencies are not enabled if the target is not
  currently being built.
* Features enabled on build-dependencies or proc-macros will not be unified when
  those same dependencies are used as a normal dependency.
* Features enabled on dev-dependencies will not be unified when those same
  dependencies are used as a normal dependency, unless those dev-dependencies
  are currently being built.

None of these include this crate's case, so when `problematic-crate` is
compiled, both its conflictive features are enabled, and thus it fails.
