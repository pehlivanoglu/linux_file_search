all binaries must be executed as superuser

keeps special characters in a single database named _.db ( #, *, &, etc.)

DONE -- will edit populate_db() function to exclude desired directories. (maybe with config.toml) --is done with default setup type

lots of files created and removed in a small amount of time in Linux. Will solve the issue

## OPTIMIZATION

use file system monitoring tools (like inotify on Linux)

deamonize the update rs - reindexing periodically

## generated files:
daemon files at /tmp
config file at ~/.config
db files at /var/lib/file_search/