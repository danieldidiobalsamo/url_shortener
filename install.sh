#!/usr/bin/env bash

echo -e 'Downloading cert-manager CRDs...\n'
kubectl apply -f https://github.com/cert-manager/cert-manager/releases/download/v1.9.1/cert-manager.crds.yaml

# deploy app
echo -e 'Setup url-shortener application and wait for pods / ingress...\n'
helm install url-shortener deployment/url-shortener --wait

# add ingress IP to /etc/hosts
domainName="short.home"

ip=`kubectl get ingress --field-selector metadata.name=url-shortener --namespace url-shortener -o custom-columns=:.status.loadBalancer.ingress[0].ip | tr -d '\n'`
mapping="$ip    $domainName
$ip    $domainName.backend"

text="The following resolution has to be written in /etc/hosts, or manually be configured in your internal DNS :

$mapping


Update /etc/hosts ?
"

if (whiptail --title "url-shortener installation" --yesno "$text" 16 80) then
  echo "need permission to write in /etc/hosts:"
  echo "$mapping" | cat - /etc/hosts > /tmp/hosts_tmp && sudo mv /tmp/hosts_tmp /etc/hosts

  echo -e '================================================================================='
  echo -e 'Application is deployed !'
  echo -e "Open this link : http://$domainName"
  echo -e '================================================================================='
else
  echo -e '================================================================================='
  echo "Please manually configure your internal DNS with the following :"
  echo -e "$mapping\n"
  echo -e "Then Open this link : http://$domainName"
  echo -e '================================================================================='
fi
