# Ray Tracing in Rust

Simple path tracer made in Rust based on Peter Shirley's book
[_Ray Tracing in One Weekend_](https://raytracing.github.io/books/RayTracingInOneWeekend.html).

To-do:
- [x] _Ray Tracing in One Weekend_ (actually in more than one weekend...)
- [x] Refactor the code
- [x] Command line arguments
- [x] Progress bar
- [ ] Parallelism (multithreading)
- [x] Save to a better image format (BMP)

## Usage

```
$ cargo build --release
$ ./target/release/raytracing_in_rust --help
Raytracing in Rust 0.1.0

USAGE:
    raytracing_in_rust [FLAGS] [OPTIONS] <output>

FLAGS:
    -h, --help        Prints help information
    -p, --parallel    Use multithreading for rendering.
    -V, --version     Prints version information

OPTIONS:
    -j <thread-number>        Number of threads to spawn. Default is number of logical cores.

ARGS:
    <output>    Where to save the result (BMP file).
```
