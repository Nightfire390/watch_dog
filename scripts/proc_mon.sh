#!/bin/bash

procs="/tmp/procs.toml"

get_processes() {
    ps ax --format user:30,pid,tty,start,command | tail -n +2 | tee | awk '
        /grep|ps|awk/ {
            next
        } {
            print "[[proc]]"
            printf "username = \"%s\"\n", $1
            printf "pid = %s\n", $2
            printf "tty = \"%s\"\n", $3
            printf "start = \"%s\"\n", $4
            printf "cmd = \"%s", $5
            for(i = 6; i <= NF; i++) {
                printf " %s", $i
            }
            printf "\"\n\n"
        }' | head -n -1 > $procs
}

get_processes
