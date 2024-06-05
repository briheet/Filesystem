# FUSE

info from [Here](https://github.com/libfuse/libfuse)

Making a fuse filesystem
Majorily testing it out right now

The idea is to have files in like a database and view it out.
The files also have custom links for you to navigate.

How to run for now:

```bash
mkdir -p test_mount
```

```bash
# Compile the FUSE program
gcc -lfuse -o test test.c

# Run the FUSE program with a mount point
./test -d test_mount

### Troubleshooting
If the commands are not working, consider the following:
```

- **Check if FUSE is installed**: Ensure that FUSE is installed on your system. You can check this by running:

```bash
  fuse --version
```

- With these corrections, the markdown should render properly and provide clear instructions.
