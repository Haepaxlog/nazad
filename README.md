# Nazad - Language Learning Dashboard
(/ˈna.zat/ - no recursion required)

![Nazad Logo](./icons/nazad_logo.png)

## What is This?
Nazad is a dashboard written in Rust with [Dioxus](https://github.com/DioxusLabs/dioxus) to track Language Learning. It is inspired by [Destaq's ll-dashboard](https://github.com/Destaq/ll-dashboard).

## How to run it
### Development

You need to have [trunk](https://trunkrs.dev/) installed locally by any of these methods:
```bash 
# Install via homebrew on Mac, Linux or Windows (WSL).
brew install trunk

# Install a release binary (great for CI).
# You will need to specify a value for ${VERSION}.
wget -qO- https://github.com/thedodd/trunk/releases/download/${VERSION}/trunk-x86_64-unknown-linux-gnu.tar.gz | tar -xzf-

# Install via cargo.
cargo install --locked trunk
# Until wasm-bindgen has pre-built binaries for Apple M1, M1 users will
# need to install wasm-bindgen manually.
cargo install --locked wasm-bindgen-cli
```


Simply serve the whole App with trunk:

```bash
$ yarn run dev
```

### Release 

Serve the optimized trunk App locally:

```bash
$ yarn run release
```

Or use the Dockerfile (here you don't need trunk installed locally):

```bash
$ docker build -t nazad-docker .
$ docker exec -p 8080:8080 -it localhost/nazad-docker
```

### Contributing
I mean, if you really want to who am I to judge :), even though it will probably be always more of an experiment
