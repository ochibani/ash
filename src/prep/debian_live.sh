#!/bin/sh

# If you are installing this on VirtualBox to test it out, set RAM bigger than
# 1024MB as otherwise it errors out (running out of cache/RAM space)

#set echo off

main() {
    if [ -z "$HOME" ]; then HOME=~ ; fi
    RELEASE="sid"
    prep_packages="git tmux parted btrfs-progs dosfstools cryptsetup debootstrap ntp efibootmgr"

    sync_time
    echo "Please wait for 30 seconds!"
    sleep 30 # Wait before updating repo and downloading packages
    fixdb
    apt-get -y --fix-broken install $prep_packages
    configs
    #git clone http://github.com/ashos/ashos
    git config --global --add safe.directory $HOME/ashos # prevent fatal error "unsafe repository is owned by someone else"
    #cd ashos
    #/bin/bash ./src/prep/parted_gpt_example.sh $2
    #python3 setup.py $1 $2 $3
}

# Configurations
configs() {
    echo "export LC_ALL=C LC_CTYPE=C LANGUAGE=C" | tee -a $HOME/.bashrc
    #echo "alias p='curl -F "'"sprunge=<-"'" sprunge.us'" | tee -a $HOME/.bashrc
    echo "alias p='curl -F "'"f:1=<-"'" ix.io'" | tee -a $HOME/.bashrc
    echo "alias d='df -h | grep -v sda'" | tee -a $HOME/.bashrc
    echo "setw -g mode-keys vi" | tee -a $HOME/.tmux.conf
    echo "set -g history-limit 999999" | tee -a $HOME/.tmux.conf
}

# Remove man pages (fixes slow man-db trigger) and update packages db
fixdb() {
    sed -i "s/[^ ]*[^ ]/$RELEASE/3" /etc/apt/sources.list
    apt-get -y autoremove
    apt-get -y autoclean
    apt-get -y clean # Needed?
    apt-get -y remove --purge man-db
    apt-get -y update
    apt-get -y check # Needed?
}

# Sync time
sync_time() {
    if [ -x "$(command -v wget)" ]; then
        date -s "$(wget -qSO- --max-redirect=0 google.com 2>&1 | grep Date: | cut -d' ' -f5-8)Z"
    elif [ -x "$(command -v curl)" ]; then
        date -s "$(curl -I google.com 2>&1 | grep Date: | cut -d' ' -f3-6)Z"
    else
        echo "F: Syncing time failed! Neither wget nor curl available."
    fi
}

main "$@"; exit

