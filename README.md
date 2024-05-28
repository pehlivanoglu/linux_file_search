all binaries must be executed as superuser

keeps special characters in a single database named _.db ( #, *, &, etc.)

lots of files created and removed in a small amount of time in Linux. Will solve the issue

## OPTIMIZATION

use file system monitoring tools (like inotify on Linux)

deamonize the update rs - reindexing periodically

## generated files:
daemon files at /tmp
config file at /etc/file_search/
db files at /var/lib/file_search/

## how to daemonize
put */2 * * * * /home/pehlivanoglu/CS350/project/linux_file_search/target/release/update
to crontab -e