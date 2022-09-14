# The Future Circular Collider

_The Future Circular Collider_ is a simulated lab experiment authored for the 2022 U.S. Physics Team training camp. This repository contains the source code for the (command-line) programs used to run the simulation.

- [Where do I download the program?](#where-do-i-download-the-program)
  - [Technical details on CPU architecture](#technical-details-on-cpu-architecture)
- [How do I run the program?](#how-do-i-run-the-program)
- [Compiling the program from source](#compiling-the-program-from-source)
- [Acknowledgements and... copyright?](#acknowledgements-and-copyright)

# Where do I download the program?

There are two programs defined in this repository:

- `collision-black-box` contains the main simulation program; **this is probably the one you want**.

- `cli-check` is just a software-compatibility test meant to assess whether students were able to run and interact the simulation program correctly without giving away the actual content of the simulation lab. You probably don't care about this one.

Go to the [Releases page][releases], and under **Assets**, download the program whose name matches your operating system:

- `collision-black-box-linux`: Linux-based operating systems.
- `collision-black-box-windows`: Windows.
- `collision-black-box-macos`: macOS.

## Technical details on CPU architecture

All programs are built for the `x86_64` architecture, which works for most systems. If you are on a different architecture, you'll have to compile this program from source using [`cargo` (builder tool for Rust projects)][cargo].

[releases]: https://github.com/USPhysicsTeam/2022-future-circular-collider/releases
[cargo]: https://doc.rust-lang.org/cargo/getting-started/installation.html

# How do I run the program?

Running the program requires basic command-line literacy (see [MDN's command-line crash course][mdn] for a quick tutorial on how to navigate the command line).

Instructions:

- `cd` to the directory/folder containing the downloaded program (usually your "Downloads" folder):

  ```sh
  cd ~/Downloads
  ```

- macOS/Linux only-- mark the program as executable:

  ```sh
  # replace `*` below with `macos` or `linux` depending on your operating system
  chmod +x collision-black-box-*
  ```

- Then, run the program by invoking the name of the downloaded executable file as a command. (The difference between macOS/Linux and Windows here is that macOS/Linux requires a `./` before the name; Windows does not.)

  - macOS/Linux:
    ```sh
    # replace `*` with `macos`/`linux`
    ./collision-black-box-*
    ```
  - On Windows:
    ```sh
    collision-black-box-windows
    ```

- Running that command should print out a help page listing out available subcommands for running different parts of the simulation lab.

  - To run part 1 (Collision with Wall), use the subcommand `wall`:

    ```sh
    # macOS/Linux
    ./collision-black-box-* wall

    # Windows
    collision-black-box-windows wall
    ```

  - To run part 2 (Collision with Disk), use the subcommand `disk`:

    ```sh
    # macOS/Linux
    ./collision-black-box-* disk

    # Windows
    collision-black-box-windows disk
    ```

[mdn]: https://developer.mozilla.org/en-US/docs/Learn/Tools_and_testing/Understanding_client-side_tools/Command_line

# Compiling the program from source

Compiling the program requires moderate familiarity with the command-line and Rust's `cargo` tool. The main project is defined in the `collision-black-box` folder. First `cd` into that folder. Run the program directly with `cargo run`, or, alternatively, compile-and-install the program with `cargo install --path .` and then invoke the program as `collision-black-box`.

# Acknowledgements and... copyright?

This simulation lab was designed and implemented by U.S. Physics Team Coaches Kevin Zhou and Kye Shi. Please cite us if you wish to _distribute_ any copies or modified copies of this code.
