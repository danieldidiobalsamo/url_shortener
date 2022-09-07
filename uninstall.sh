#!/usr/bin/env bash

text="This script is going to remove url-shortener app, including its persistent volume.
It won't modify cert-manager namespace.

It also removes 'url-shortener-rust.com' resolution in /etc/hosts

Are you sure to continue ?"

if (whiptail --title "url-shortener uninstall" --yesno "$text" 14 60) then
    helm uninstall url-shortener
    echo "sudo sed -i '/url-shortener-rust.com/d' /etc/hosts"
    sudo sed -i '/url-shortener-rust.com/d' /etc/hosts
fi