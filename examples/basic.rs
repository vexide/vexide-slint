#![no_std]
#![no_main]

// NOTE:
// This example WILL NOT BUILD by default, because it lacks a build.rs
// script. If you want to build this, see README.md for instructions.

slint::include_modules!();

use vexide::prelude::*;
use vexide_slint::initialize_slint_platform;

struct Robot {
    // ...
}

impl Compete for Robot {
    // ...
}

#[vexide::main]
async fn main(peripherals: Peripherals) {
    let robot = Robot {
        // ...
    };

    // Since running the Slint UI is a blocking operation, we need to spawn the
    // competition task as a separate task that will run concurrently.
    // The Slint runtime internally polls all spawned futures.
    vexide::task::spawn(robot.compete()).detach();

    // Initialize the Slint platform with the V5 display-backed implementation.
    initialize_slint_platform(peripherals.display);
    // Create and run the application. For more information on this, see the
    // Slint documentation.
    App::new()
        .expect("Failed to create window")
        .run()
        .expect("Failed to run application");
    // Since MyApplication::run() could return if the application is closed
    // programmatically, we need to convince the compiler that the return type
    // is `!` (never).
    vexide::program::exit();
}
