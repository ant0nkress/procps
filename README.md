# procps
Rust reimplemtation of the procps project

Provides command line and full screen utilities for browsing procfs, a "pseudo" file system dynamically generated by the kernel to provide information about the status of entries in its process table (such as whether the process is running, stopped, or a "zombie").

* /bin/kill: Sends a signal to a process, typically to terminate the process.
* /bin/ps: Displays information about active processes.
* /usr/bin/free: Shows the amount of free and used memory in the system.
* /usr/bin/pgrep: Searches for processes based on name and other attributes.
* /usr/bin/pidwait: Waits for a specific process to terminate.
* /usr/bin/pmap: Displays the memory map of a process.
* /usr/bin/pwdx: Shows the current working directory of a process.
* /usr/bin/skill: Sends a signal to processes based on criteria like user, terminal, etc.
* /usr/bin/slabtop: Displays detailed kernel slab cache information in real time.
* /usr/bin/tload: Prints a graphical representation of system load average to the terminal.
* /usr/bin/top: Displays real-time information about system processes.
* /usr/bin/uptime: Shows how long the system has been running, including load average.
* /usr/bin/vmstat: Reports information about processes, memory, paging, block IO, traps, and CPU activity.
* /usr/bin/w: Shows who is logged on and what they are doing.
* /usr/bin/watch: Executes a program periodically, showing output fullscreen.
* /usr/bin/pkill: Kills processes based on name and other attributes.
* /usr/bin/snice: Changes the scheduling priority of a running process.

Note:

 * /bin/kill is already implemented in https://github.com/uutils/coreutils

