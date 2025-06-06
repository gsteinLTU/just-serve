# just-serve

When you want to say "just serve this directory" and not worry about CORS. For example, my use case was loading an script from a local directory in a page running not on localhost. Don't use this in production, obviously.

I was getting tired of everything else running into a brick wall in the browser due to various CORS issues. If you need more control, you should probably write your own server.

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
