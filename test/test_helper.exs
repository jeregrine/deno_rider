{:ok, _} = DenoRider.start_link([])
:ok = ExUnit.start(exclude: [:benchmark, :panic])
