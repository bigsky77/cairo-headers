# cairo-headers

Generate perfect Cairo headers straight to your clipboard! Inspired by this [transmissions11/headers](https://github.com/transmissions11/headers).

## Build

Install Rust and Cargo.  

Install the repo to your path. 

```sh
cargo install --path your/path
```

You may also need to install libxcb. 

```sh
sudo apt-get install libxcb-render0-dev libxcb-shape0-dev libxcb-xfi
xes0-dev
```

## Use

This program has three basic commands 

L is used for line.  

```sh
bigsky /headers l tempest
```

```sh
### ============ tempest =============
```

H is used for headers

```sh
bigsky /headers h tempest
```

```sh
### ==================================
###              TEMPEST
### ==================================
```

and F is used for functions

```sh
bigsky /headers f 
```

```sh
# notice
# dev
# param
```
