#!/usr/bin/env bash

echo -e 'Downloading cert-manager CRDs...\n'
kubectl apply -f https://github.com/cert-manager/cert-manager/releases/download/v1.9.1/cert-manager.crds.yaml

# let user choose the application domain name
text="
Please choose url-shortener application domain name.

Note: if a private domain name is provided, a 'Kubernetes Ingress Controller Fake Certificate' is issued. Otherwise let's encrypt issues one.
"

domainName=$(whiptail --title "url-shortener installation" --inputbox "$text" 16 60 short.home 3>&1 1>&2 2>&3)

# deploy app
echo -e 'Setup url-shortener application and wait for pods / ingress...\n'
helm install url-shortener deployment/rust-url-shortener \
  --set applicationDomainName=$domainName \
  --wait
echo -e 'Done.\n'

# add ingress IP to /etc/hosts
ip=`kubectl get ingress --field-selector metadata.name=url-shortener --namespace url-shortener -o custom-columns=:.status.loadBalancer.ingress[0].ip | tr -d '\n'`
mapping="$ip    $domainName"

text="The following resolution has to be written in /etc/hosts

$mapping

If you prefer to do it manually, please add this line in /etc/hosts

Continue ?
"

if (whiptail --title "url-shortener installation" --yesno "$text" 16 60) then
  echo "need sudo to write in /etc/hosts:"
  echo "$mapping" | cat - /etc/hosts > /tmp/hosts_tmp && sudo mv /tmp/hosts_tmp /etc/hosts

  echo -e '(10/10) Application is deployed !'
  echo -e "Open this link : https://$domainName"
else
  echo "Please manually paste this line in /etc/hosts:"
  echo -e "$mapping\n"
  echo -e "Then Open this link : https://$domainName"
fi
