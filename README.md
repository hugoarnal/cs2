# cs2

Epitech Banana (v4) Coding Style Helper

> [!WARNING]
> This tool is unfortunately only usable by Epitech students.
>
> You need to have access to `epiclang` and `banana-coding-style-checker` repos to be able to use `cs2`.

## Features

- Easily install [epiclang](https://github.com/Epitech/epiclang) & [banana-coding-style-checker](https://github.com/Epitech/banana-coding-style-checker) [(see here)](#installing-dependencies)
- Remove duplicate errors[^1]
  - Pipe current command: `make 2>&1 | cs2`
  - `cs2 run <command>`
- Finds your build system[^2] and builds it automatically
- Ignores all errors from files in your `.gitignore`

[^1]: It seems as if header files work differently when using banana. When they are included, if there's an error in them, it prints it every time the header is included.

[^2]: see [Supported build systems](#supported-build-systems)

## Usage

There are multiple ways to use `cs2`.

### Build system (most common)

```sh
cs2
```

Only running `cs2` will automatically find your build system, recompile your project by using `epiclang` and `banana`.

> Warning: your project must need to support changing environment variables like CC to use epiclang.

#### Supported build systems:
- GNU Makefile

To be supported in the future (or send a PR to make it work :-)):
- CMake (in the future)

### Run command into cs2

```
epiclang main.c 2>&1 | cs2
cs2 run epiclang main.c
```

Both of these commands will run `epiclang main.c` and format the error output with `cs2`.

> Mark the usage of `2>&1` for piping into `cs2` which is necessary.

## Install

Requirements:
- Rust
- Cargo
- clang (>= 20)
- CMake
- LLVM
- llvm-libs

### Installing cs2

Use the `install.sh` script:

```sh
curl -s https://raw.githubusercontent.com/hugoarnal/cs2/main/install.sh | sh
```

You can also clone the repo directly to `/usr/local/share/cs2` then run `compile.sh`:
```sh
git clone https://github.com/hugoarnal/cs2.git /tmp/cs2-cs2
sudo mkdir -p /usr/local/share/cs2
sudo mv /tmp/cs2-cs2 /usr/local/share/cs2/cs2
/usr/local/share/cs2/cs2/compile.sh
```

### Installing dependencies

After installing cs2, you can install `epiclang` and `banana` plugin with:

```sh
cs2 install
```

Only need one? Use `cs2 install --package`:
```sh
cs2 install --package banana
```

> You can install `banana` faster with `-j`, like `make -j`.

## Updating dependencies

Update all of them:
```sh
cs2 update
```

Update only of of them with:
```sh
cs2 update --package banana
```

> You can install `banana` faster with `-j`, like `make -j`.

Force rebuild/copy even with if there is not update:
```sh
cs2 update --package banana --force
```
