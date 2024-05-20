#include <fcntl.h>
#include <sys/stat.h>
#define FUSE_USE_VERSION 31
#define _FILE_OFFSET_BITS 64
#include <fuse.h>
#include <stdio.h>
#include <string.h>

int hello_getattr(const char *path, struct stat *statbuf) {
  printf("getattr called for %s\n", path);

  memset(statbuf, 0, sizeof(struct stat));
  statbuf->st_mode = S_IFDIR | 0755;

  return 0;
}

int hello_readdir(const char *path, void *buf, fuse_fill_dir_t filler,
                  off_t offset, struct fuse_file_info *fi) {
  (void)offset;
  (void)fi;

  struct stat file_statbuf;
  file_statbuf.st_mode = S_IFREG | 0644;

  struct stat dir_statbuf;
  dir_statbuf.st_mode = S_IFDIR | 0755;

  filler(buf, "test", &dir_statbuf, 0);
  filler(buf, "test2", &dir_statbuf, 0);
  filler(buf, "test3", &file_statbuf, 0);

  return 0;
}

static const struct fuse_operations hello_oper = {
    .getattr = hello_getattr,
    .readdir = hello_readdir,
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
