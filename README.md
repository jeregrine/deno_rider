# DenoRider

DenoRider is an Elixir library that embeds the [Deno](https://deno.com) runtime
via [Rustler](https://hexdocs.pm/rustler). It is a performant way to run
JavaScript in Elixir and it doesn't depend on external executables.

## Installation

Add `:deno_rider` to your `mix.exs`:

```elixir
{:deno_rider, "~> 0.1"}
```

Add `DenoRider` to your application's supervisor:

```elixir
# lib/my_app/application.ex
def start(_type, _args) do
  children = [
    ...,
    DenoRider
  ]

  Supervisor.start_link(children, strategy: :one_for_one, name: MyApp.Supervisor)
end
```

See [`start_link/1`](https://hexdocs.pm/deno_rider/DenoRider.html#start_link/1)
for more information about options when starting DenoRider.

### Precompiled NIFs

DenoRider provides precompiled NIFs (via
[RustlerPrecompiled](https://hexdocs.pm/rustler_precompiled)) for the following
targets:

* `aarch64-apple-darwin`
* `aarch64-unknown-linux-gnu`
* `x86_64-apple-darwin`
* `x86_64-pc-windows-msvc`
* `x86_64-unknown-linux-gnu`

Contributions for precompiling DenoRider to more targets are welcome!

You can build DenoRider yourself by setting the environment variable
`DENO_RIDER_BUILD` to `true`.

## Usage

To run JavaScript code with DenoRider, you use `eval`. For example:

```elixir
iex> DenoRider.eval("1 + 2")
{:ok, 3}
iex> DenoRider.eval("globalThis.foo = 'bar'")
{:ok, "bar"}
iex> DenoRider.eval("globalThis.foo")
{:ok, "bar"}
```

See [`eval/1`](https://hexdocs.pm/deno_rider/DenoRider.html#eval/1) and
[`eval/2`](https://hexdocs.pm/deno_rider/DenoRider.html#eval/2) for different
ways of running JavaScript code.

If you don't want to run DenoRider as a process, you can manage the runtime
manually:

```elixir
iex> {:ok, runtime} = DenoRider.start_runtime() |> Task.await()
{:ok, %DenoRider.Runtime{reference: #Reference<0.328177905.1027473408.14690>}}
iex> DenoRider.eval("1 + 2", runtime: runtime) |> Task.await()
{:ok, 3}
iex> DenoRider.stop_runtime(runtime) |> Task.await()
{:ok, nil}
```

Read the [full documentation](https://hexdocs.pm/deno_rider/DenoRider.html) for
more information.

## FAQ

### When should I use DenoRider?

If you need to run JavaScript from Elixir, you can give DenoRider a try. Some
common use cases for that is for example server-side rendering of React or
wrapping a JavaScript library in Elixir.

### What makes DenoRider different from other ways of calling JavaScript from Elixir?

Since DenoRider embeds the Deno runtime, you don't need any external executables
(such as `node`) and the latency for your JavaScript calls is very low.

### When shouldn't I use DenoRider?

One reason could be that you want to use another JavaScript runtime, such as
Node or Bun. Or if you want to use Deno features that DenoRider doesn't provide.

### What are some common pitfalls?

There's currently no npm integration, which means that if you want to use an npm
package you first need to convert it into something that's consumable by
DenoRider. To do that you can use [Rollup](https://rollupjs.org), for example.

## License

DenoRider is released under the MIT license. See the [LICENSE](LICENSE) file for
more information.
