FROM elixir:1.17.3
RUN apt-get update
RUN apt-get install -y inotify-tools
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
RUN rustup default 1.82.0
RUN mix local.hex --force
RUN mix local.rebar --force
RUN mkdir -p /elixir
WORKDIR /elixir
COPY . /elixir
