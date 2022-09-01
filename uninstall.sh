#!/usr/bin/env bash

text="This script is going to remove url-shortener app, including its persistent volume.
It won't modify cert-manager namespace.

Are you sure to continue ?"

if (whiptail --title "url-shortener uninstall" --yesno "$text" 12 60) then
    helm uninstall url-shortener
fi