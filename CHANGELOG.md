# Changelog

## 0.2.0 (2025-03-31)

### Improvements

* A JavaScript API, `DenoRider.apply`, is introduced for calling Elixir from
  JavaScript.
* Promises returned in `DenoRider.eval` are resolved before being returned to
  Elixir-land.
* The JavaScript runtime now always gets a dedicated process. No need for
  awaiting tasks anymore.
* `DenoRider.eval!` is introduced, which raises if the evaluation isn't
  successful.
* Dependencies are upgraded.

### Breaking changes

* `DenoRider.start_runtime` and `DenoRider.stop_runtime` are removed. Use
  `DenoRider.start` and `DenoRider.stop` instead.
* `DenoRider.eval/2` no longer takes a `:runtime` option. Use the `:pid` option
  instead.
* `DenoRider.eval/2` no longer returns a task for manually managed runtimes.

## 0.1.1 (2025-01-11)

### Fixes

* Build `%DenoRider.Error{}` as exception instead of struct.
