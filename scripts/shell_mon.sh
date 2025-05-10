#!/bin/bash

shells="/tmp/shells.toml"

get_shells() {
    PROCPS_USERLEN=30 w -h -i -f | awk '{ 
        print "[[shell]]"
        printf "username = \"%s\"\n", $1
        printf "tty = \"%s\"\n", $2
        printf "from = \"%s\"\n", $6
        printf "process = \"%s\"\n\n", $7
    }' > $shells
}

get_shells
