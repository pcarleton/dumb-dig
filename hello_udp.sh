#!/bin/bash
# http://xmodulo.com/tcp-udp-socket-bash-shell.html
PORT=${1:-53}
MSG=${2:-Hello}

exec 3<>/dev/udp/127.0.0.1/$PORT

echo -n $MSG >&3
timeout 1 cat <&3
