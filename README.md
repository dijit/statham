# Statham 

Statham is a password creator and transporter.

⚠️ This project in its current form has only the Agent Code for Windows; it is under heavy development when I find time ⚠️

It is designed for Windows systems primarily because we don't like AD.

This is a reimplementation of a similar system I wrote at Massive Entertainment in Go for assisting with access to machines which may have lost network connection.

There are 4 main components:

`server/` a REST-like API for publishing/retreiving data; this service can be replaced with Hashicorp Vault with a config option to the agent

`agent/` a daemon/service (or cronjob/timer) that creates new users based on a config file, randomly generates a password and ships the encrypted password to `server/`, `proxy/` or Vault.

`proxy/` a write-only proxy towards `server/` with rate limiting.

`client/` a read-only client, intended to be used by admins wanting to get the latest password for a given server.


## How to build

In order to build this project, the dependencies must be vendored. This can be achieved by performing the following:

2. Run `cargo vendor --versioned-dirs cargo/vendor`
3. Rerun `cargo raze` to regenerate the Bazel BUILD files

At this point you should now be able to run `bazel build ...` to compile the source code.
