#!/usr/bin/env bash

# get app domain name (the user can choose a custom one using install script)
domainName=`kubectl get ingress --field-selector metadata.name=url-shortener --namespace url-shortener -o custom-columns=:.spec.tls[0].hosts[0]`

text="This script is going to remove url-shortener app, including its persistent volume.

It also removes url-shortener name resolution in /etc/hosts

Are you sure to continue ?"

if (whiptail --title "url-shortener uninstall" --yesno "$text" 14 60) then
    echo "helm uninstall..."
    helm uninstall url-shortener --wait --timeout=120s
    
    regex=$(printf "/%s/d" $domainName)
    cmd="sudo sed -i "$regex" /etc/hosts"
    echo $cmd
    $cmd
fi