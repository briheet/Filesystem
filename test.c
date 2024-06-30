#include <asm-generic/errno.h>
// #include <cerrno>
#include <fcntl.h>
#include <sys/stat.h>
#include <sys/types.h>
#define FUSE_USE_VERSION 31
#define _FILE_OFFSET_BITS 64
// #include <errno-base.h>
#include <fuse.h>
#include <stdio.h>
#include <string.h>

static struct options {
  const char *filename;
  const char *contents;
} options;

void statbuf_for_path(const char *path, struct stat *statbuf) {
  if (strcmp(path, "/") == 0 || strcmp(path, "/test") == 0 ||
      strcmp(path, "/test2") == 0) {
    statbuf->st_mode = S_IFDIR | 0755;
  } else {
    statbuf->st_mode = S_IFREG | 0644;
  }
}

static int hello_open(const char *path, char *buf, size_t size, off_t offset,
                      struct fuse_file_info *fi) {
  size_t len;
  (void)fi;

  if (strcmp(path + 1, options.filename) != 0) {
    return -ENOENT; // ENDENT from errno.h turns out to be -2
  }

  if ((fi->flags & O_ACCMODE) != O_RDONLY) {
    return -EACCES;
  }

  return 0;
}

int hello_getattr(const char *path, struct stat *statbuf) {
  memset(statbuf, 0, sizeof(struct stat));
  statbuf_for_path(path, statbuf);

  return 0;
}

int hello_readdir(const char *path, void *buf, fuse_fill_dir_t filler,
                  off_t offset, struct fuse_file_info *info) {
  (void)info;
  (void)offset;

  struct stat statbuf;
  statbuf_for_path("/test", &statbuf);
  filler(buf, "test", &statbuf, 0);
  statbuf_for_path("/test2", &statbuf);
  filler(buf, "test2", &statbuf, 0);
  statbuf_for_path("/test3", &statbuf);
  filler(buf, "test3", &statbuf, 0);

  return 0;
}

static const struct fuse_operations hello_oper = {
    .getattr = hello_getattr,
    .readdir = hello_readdir,
    .open = hello_open,
};

int main(int argc, char *argv[]) {
  int ret;
  struct fuse_args args = FUSE_ARGS_INIT(argc, argv);

  /* Parse options */
  if (fuse_opt_parse(&args, NULL, NULL, NULL) == -1)
    return 1;

  ret = fuse_main(args.argc, args.argv, &hello_oper, NULL);
  fuse_opt_free_args(&args);
  return ret;
}
