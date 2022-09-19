#!/usr/bin/env bash

echo -e '(1/10) Adding offical cert-manager repo (jetstack)...\n'
helm repo add jetstack https://charts.jetstack.io
echo -e '(2/10) Updating helm repositories...\n'
helm repo update
echo -e 'Done.\n'

echo -e '(3/10) Creating cert-manager namespace...\n'
kubectl create namespace cert-manager
echo -e 'Done.\n'

echo -e '(4/10) Downloading cert-manager CRDs...\n'
kubectl apply -f https://github.com/cert-manager/cert-manager/releases/download/v1.9.1/cert-manager.crds.yaml

echo -e '\n(5/10) Setup url-shortener application...\n'
helm install url-shortener deployment/rust-url-shortener
echo -e 'Done.\n'

echo -e '(6/10) Setup cert-manager...\n'
helm install \
  cert-manager jetstack/cert-manager \
  --namespace cert-manager \
  --version v1.9.1 \
  --set installCRDs=false \
  1>/dev/null # redirect standard output (not error) to null (it shows generic info not relevant with this application usage of cert-manager)
echo -e 'Done.\n'

echo -e '(7/10) Waiting for all pods to be ready...\n'
kubectl wait pods --for condition=ready --namespace url-shortener --all --timeout=120s
kubectl wait pods --for condition=ready --namespace cert-manager --all --timeout=120s
echo -e 'Done.\n'

echo -e '(8/10) Waiting for ingress to get an IP...\n'

function getIngressIP () {
  ip=`kubectl get ingress --field-selector metadata.name=app-ingress --namespace url-shortener -o custom-columns=:.status.loadBalancer.ingress[0].ip | tr -d '\n'`
  echo $ip
}

ip=$( getIngressIP )
until [[ $ip =~ ^[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}$ ]]
do
  ip=$( getIngressIP )
done
echo -e "$ip assigned\n"
echo -e 'Done.\n'

# adding ingress IP to /etc/hosts
mapping="$ip    short.home"

text="The following resolution has to be written in /etc/hosts
$mapping
If you prefer to do it manually, please add this line in /etc/hosts

Continue ?
"

if (whiptail --title "url-shortener uninstall" --yesno "$text" 14 60) then
  echo "need sudo to write in /etc/hosts:"
  echo "$mapping" | cat - /etc/hosts > /tmp/hosts_tmp && sudo mv /tmp/hosts_tmp /etc/hosts

  echo -e '(10/10) Application is deployed !'
  echo -e 'Open this link : https://short.home'
else
  echo "Please manually paste this line in /etc/hosts:"
  echo -e "$mapping\n"
  echo "Then open this link : https://short.home"
fi
