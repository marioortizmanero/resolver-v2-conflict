#[cfg(all(feature = "async", feature = "sync"))]
compile_error! {
    "Conflict found."
}

// This crate includes a nice error to find the conflict, but the usual would
// look like:
//
// ```
// error[E0428]: the name `something` is defined multiple times
//   --> problematic-crate/src/lib.rs:12:1
//    |
// 7  | pub fn something() {
//    | ------------------ previous definition of the value `something` here
// ...
// 12 | pub fn something() {
//    | ^^^^^^^^^^^^^^^^^^ `something` redefined here
//    |
//    = note: `something` must be defined only once in the value namespace of this module
// ```
//
// To see that error instead, uncomment the following lines:
//
// #[cfg(feature = "async")]
// pub fn something() {
//     println!("async version")
// }
// 
// #[cfg(feature = "sync")]
// pub fn something() {
//     println!("sync version")
// }
