#!/bin/bash
# http://xmodulo.com/tcp-udp-socket-bash-shell.html
#PORT=${1:-53}
#MSG=${2:-Hello}


exec 3<>/dev/udp/8.8.8.8/53

#echo -n $MSG >&3
#echo -n "HELLO" >&3
cat dnsquery.txt >&3
timeout 1 cat <&3
