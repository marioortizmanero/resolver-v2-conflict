# Conflict of features with resolver v2

This repository proves experimentally that even with Rust's feature resolver v2,
it is possible to have a conflict. This means that all features should still be
*additive* in order to avoid these cases.

Please let me know if there's anything wrong in the code, but it's quite simple.
The main crate, `resolver-v2-conflict` has two dependencies: `async-dep` and
`sync-dep`, both of which depend on `problematic-crate` but with different
features. It turns out that these features are conflictive instead of additive,
so it's impossible to compile `resolver-v2-conflict` even with the resolver v2.

You can use the `toggle-resolver` script to switch between the resolver 1 and
the resolver 2 easily.
