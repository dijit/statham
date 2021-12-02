# Statham 

Statham is a password creator and transporter.

It is designed for Windows systems primarily because we don't like AD.


## How to build

In order to build this project, the dependencies must be vendored. This can be achieved by performing the following:

2. Run `cargo vendor --versioned-dirs cargo/vendor`
3. Rerun `cargo raze` to regenerate the Bazel BUILD files

At this point you should now be able to run `bazel build ...` to compile the source code.
