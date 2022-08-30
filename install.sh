#!/usr/bin/env bash

echo -e '(1/9) Adding offical cert-manager repo (jetstack)...\n'
helm repo add jetstack https://charts.jetstack.io
echo -e '(2/9) Updating helm repositories...\n'
helm repo update
echo -e 'Done.\n'

echo -e '(3/9) Creating cert-manager namespace...\n'
kubectl create namespace cert-manager
echo -e 'Done.\n'

echo -e '(4/9) Downloading cert-manager CRDs...\n'
kubectl apply -f https://github.com/cert-manager/cert-manager/releases/download/v1.9.1/cert-manager.crds.yaml

echo -e '(5/9) Setup url-shortener application...\n'
helm install url-shortener deployment/rust-url-shortener
echo -e 'Done.\n'

echo -e '(6/9) Setup cert-manager...\n'
helm install \
  cert-manager jetstack/cert-manager \
  --namespace cert-manager \
  --version v1.1.1 \
  --set installCRDs=false \
  1>/dev/null # redirect standard output (not error) to null (it shows generic info not relevant with this application usage of cert-manager)
echo -e 'Done.\n'

echo -e '(7/9) Waiting for all application pods to be ready...\n'
kubectl wait pods --namespace url-shortener --all --for condition=Ready --timeout=90s
echo -e 'Done.\n'

# adding ingress IP to /etc/hosts
ip=`kubectl get ingress --field-selector metadata.name=app-ingress --namespace url-shortener -o custom-columns=:.status.loadBalancer.ingress[0].ip`
mapping="$ip    url-shortener-rust.com"

echo -e '(8/9) I need to write ingress IP in /etc/hosts'
echo -e "If you prefer to do it manually, please add this line in /etc/hosts and kill this script :"
echo -e $mapping
echo "$mapping" | cat - /etc/hosts > /tmp/hosts_tmp && sudo mv /tmp/hosts_tmp /etc/hosts
echo -e 'Done.\n'

echo -e '(9/9) Application is deployed !'
echo -e 'Open this link : https://url-shortener-rust.com'