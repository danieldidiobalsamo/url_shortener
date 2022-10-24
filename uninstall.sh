#!/usr/bin/env bash

text="This script is going to remove url-shortener app, including its persistent volume.

It also removes 'short.home' resolution in /etc/hosts

Are you sure to continue ?"

if (whiptail --title "url-shortener uninstall" --yesno "$text" 14 60) then
    echo "helm uninstall..."
    helm uninstall url-shortener --wait --timeout=120s
    
    echo "sudo sed -i '/short.home/d' /etc/hosts"
    sudo sed -i '/short.home/d' /etc/hosts
fi