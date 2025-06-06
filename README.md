# just-serve

[![Crates.io](https://img.shields.io/crates/v/just-serve)](https://crates.io/crates/just-serve)

When you want to say "just serve this directory" to test something and not worry about CORS.

just-serve is specifically designed to solve CORS issues that arise during local development when you need to serve files to web pages running on different origins.

I was getting tired of existing tools running into a brick wall in the browser due to various CORS issues (and often not having maintainers, which is unfortunate when browser standards evolve).

If you need more control, you should probably write your own server.

# Installation

```bash
cargo install just-serve
```

# Usage

```bash
just-serve -p [PORT] [DIRECTORY]
```

For example:

```bash
just-serve -p 8080 ./build
```

Or if the default port is fine and you want to serve the current directory:

```bash
just-serve
```

# Options
- `-p`, `--port`: Specify the port to serve on (default: 8080).
- `-h`, `--help`: Show help message and exit.
- `-V`, `--version`: Show version information and exit.

That's all. This is for when you just need to serve a directory right now, to be used by yourself, and you need CORS out of your way.
