#!/usr/bin/env bash

echo -e 'Downloading cert-manager CRDs...\n'
kubectl apply -f https://github.com/cert-manager/cert-manager/releases/download/v1.9.1/cert-manager.crds.yaml

echo -e 'Setup url-shortener application...\n'
helm install url-shortener deployment/rust-url-shortener --wait
echo -e 'Done.\n'

# add ingress IP to /etc/hosts
ip=`kubectl get ingress --field-selector metadata.name=app-ingress --namespace url-shortener -o custom-columns=:.status.loadBalancer.ingress[0].ip | tr -d '\n'`
mapping="$ip    short.home"

text="The following resolution has to be written in /etc/hosts

$mapping

If you prefer to do it manually, please add this line in /etc/hosts

Continue ?
"

if (whiptail --title "url-shortener installation" --yesno "$text" 16 60) then
  echo "need sudo to write in /etc/hosts:"
  echo "$mapping" | cat - /etc/hosts > /tmp/hosts_tmp && sudo mv /tmp/hosts_tmp /etc/hosts

  echo -e '(10/10) Application is deployed !'
  echo -e 'Open this link : https://short.home'
else
  echo "Please manually paste this line in /etc/hosts:"
  echo -e "$mapping\n"
  echo "Then open this link : https://short.home"
fi
