# date_mv - Program to rename file to its creation date (uses exif if possible)

```
USAGE:
    date_mv [OPTIONS] <PATH>

ARGS:
    <PATH>    Path to file/directory to rename

OPTIONS:
    -d, --directory    Use if need to rename all files within directory
    -h, --help         Print help information
```

## Examples
1. Rename a file in user`s home directory: ```date_mv ~/filename```
1. Rename all files in directory: ```date_mv -d ~/directory```
