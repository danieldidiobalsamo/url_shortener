#!/usr/bin/env bash

######################################################################
# url-shortener-redis chart installation and cluster setup
######################################################################

# deploy app
echo -e 'Setup url-shortener redis cluster...\n'
helm install url-shortener-redis deployment/url-shortener-redis \
  --namespace url-shortener-redis --create-namespace

# call redis-cli into redis-sts-0 to create cluster shards
echo -e 'Setup redis cluster shards...\n'

nbReplicasTarget=$(kubectl get sts redis-sts -o custom-columns=:.spec.replicas -n url-shortener-redis)
availableReplicas=$(kubectl get sts redis-sts -o custom-columns=:.status.availableReplicas -n url-shortener-redis)

until [[ $nbReplicasTarget -eq $availableReplicas && $availableReplicas -ne 0 ]]
do
  availableReplicas=$(kubectl get sts redis-sts -o custom-columns=:.status.availableReplicas -n url-shortener-redis)
done

pods_ips=$(kubectl get pods -l app=redis-sts -o jsonpath='{range.items[*]} {.status.podIP}:6379{end}' -n url-shortener-redis)
kubectl exec redis-sts-0 -n url-shortener-redis -- redis-cli --cluster create --cluster-replicas 1 $pods_ips --cluster-yes

######################################################################
# url-shortener chart installation and /etc/hosts update
######################################################################

echo -e 'Setup url-shortener application and wait for pods / ingress...\n'
helm install url-shortener deployment/url-shortener --wait

# helm install --wait doesn't wait for ingress to get an IP
function getIngressIP () {
  ip=`kubectl get ingress --field-selector metadata.name=url-shortener --namespace url-shortener -o custom-columns=:.status.loadBalancer.ingress[0].ip | tr -d '\n'`
  echo $ip
}

ip=$( getIngressIP )
until [[ $ip =~ ^[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}$ ]]
do
  ip=$( getIngressIP )
done

# add ingress IP to /etc/hosts
domainName="short.home"

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
