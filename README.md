# cs2

Epitech Banana (v4) Coding Style Helper

> [!WARNING]
> This tool is unfortunately only usable by Epitech students.
>
> You need to have access to `epiclang` and `banana-coding-style-checker` repos to be able to use `cs2`.

## Features

- Easily install [epiclang](https://github.com/Epitech/epiclang) & [banana-coding-style-checker](https://github.com/Epitech/banana-coding-style-checker) [(see here)](#installingupdating-packages)
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

#### Flags

Enable parallelism with `-j`

```sh
cs2 -j
cs2 -j2 # You can specify the amount of threads like that
```

> If in your Makefile, you're compiling other Makefiles (typically `make -C lib/my`) that are needed for your final linkage, make sure that you use the `.NOTPARALLEL` rule to run them in order.

Don't ignore `.gitignore` errors (`--no-ignore`)

```sh
cs2 --no-ignore
```

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
/tmp/cs2-cs2/compile.sh
sudo mv /tmp/cs2-cs2 /usr/local/share/cs2/cs2
```

### Installing/Updating packages

After installing cs2, you can install `epiclang` package and `banana` plugin with:

```sh
cs2 install
```

You can update the packages with:
```sh
cs2 update
```

Only need to update a single package? Use `cs2 install/update --package`:
```sh
cs2 install --package banana
cs2 update --package banana
```

Parallelism is supported to install `banana` faster:

See [Flags section of Build system](#flags) to know more about `-j` flags.
```sh
cs2 install --package banana -j
cs2 update --package banana -j
```

Force rebuild/copy (force build even with if there is no update) (`cs2 update` only):
```sh
cs2 update --package banana --force
```

> [!NOTE]
> [Ubuntu dump](https://github.com/Epitech/dump) installs `epiclang` and `banana` by default.
>
> It is **normal** if you get the `X seems to be installed by a package manager` warning.
>
> If you wish to get rid of this warning, you can uninstall the package that's causing this warning but it is **not recommended**.
