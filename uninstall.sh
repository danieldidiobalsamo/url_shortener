#!/usr/bin/env bash

text="This script is going to remove url-shortener app, including its persistent volume.

It also removes url-shortener name resolution in /etc/hosts

Are you sure to continue ?"

if (whiptail --title "url-shortener uninstall" --yesno "$text" 14 60) then
    echo "helm uninstall..."
    helm uninstall url-shortener --wait --timeout=120s
    
    regex=$(printf "/%s/d" "short.home")
    cmd="sudo sed -i "$regex" /etc/hosts"
    echo "need permission to write in /etc/hosts:"
    echo $cmd
    $cmd
fi