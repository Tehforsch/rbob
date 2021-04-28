mod setup;

pub use setup::*;

// #[test]
// fn run_post_scaling() {
//     let out = run_bob_on_setup(
//         "post_scaling",
//         &[NormalArg("post"), RelativePath("."), NormalArg("scaling")],
//     );
//     assert!(out.success);
// }

// #[test]
// fn run_post_slice() {
//     let out = run_bob_on_setup(
//         "post_slice",
//         &[
//             NormalArg("post"),
//             RelativePath("."),
//             NormalArg("slice"),
//             NormalArg("z"),
//         ],
//     );
//     assert!(out.success);
// }
