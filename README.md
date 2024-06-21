# FUSE

info from [Here](https://github.com/libfuse/libfuse)

Making a fuse filesystem
Majorily testing it out right now

The idea is to have files in like a database and view it out.
The files also have custom links for you to navigate.

- How to run for now:

```bash
# Check if there is some issue or not
cargo check

#Build and run
cargo build
cargo run -- -d test_mount
```

- For compiling the C file and only testing from it

```bash
#Fuse version check and if not there install it with your package manager
fuse --version

# Compile the FUSE program
gcc -lfuse -o test test.c

# Run the FUSE program with a mount point
./test -d test_mount
```

- To Run the database for now

```bash
cargo run -- test_db
```

- To Check the files are created or not first install sqlitebrowser or any other tool

```bash
sqlitebrowser test_db/metadata.db
```

- Things done till now
  Tried the dummy filesystem written in C
  Built our storage in rust
  Hardcoded our files
  Generate bindings and write a fn for running the filesystem just like C would do with rust
