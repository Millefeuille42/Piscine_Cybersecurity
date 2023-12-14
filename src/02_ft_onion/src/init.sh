#!/bin/sh

cp -n /default/torrc /etc/tor/torrc
cp -n /default/nginx.conf /etc/nginx/nginx.conf
cp -n /default/sshd_config /etc/ssh/sshd_config
cp -n /default/index.html /var/tor/lib/hidden/webserv/index.html

nginx &
PID1=$!
tor &
PID2=$!
/usr/sbin/sshd -De &
PID3=$!

check_services() {
    if ! kill -0 "$PID1" >/dev/null 2>&1 || ! kill -0 "$PID2" >/dev/null 2>&1 || ! kill -0 "$PID3" >/dev/null 2>&1; then
        echo "One of the services has exited. Stopping the container."
        exit 1
    fi
}

sleep 0.2

printf "
##########################################################################
 _		       _____________________
| |		      | TOR services online |
| |_ ___  _ __	      | ------------------- |
| __/ _ \| '__|	      |	ssh:  4242	    |
| || (_) | |	      | http: 80            |
 \__\___/|_|          |_____________________|

> %s <
##########################################################################
" `cat hostname`

while true; do
    sleep 1
    check_services
done
