#!/usr/bin/env bash

text="This script is going to remove url-shortener app and its redis-cluster, including persistent datas.

It also removes url-shortener name resolution in /etc/hosts

Are you sure to continue ?"

if (whiptail --title "url-shortener uninstall" --yesno "$text" 14 60) then

    echo "remove url-shortener..."

    helm uninstall url-shortener -n url-shortener
    kubectl delete ns url-shortener

    echo "scaling redis statefulset to 0..."
    kubectl scale sts redis-sts --replicas=0 -n url-shortener-redis
    echo "waiting for all redis pods to terminate..."
    kubectl wait --for=jsonpath='{.status.availableReplicas}'=0 sts/redis-sts -n url-shortener-redis

    echo "remove url-shortener-redis..."
    helm uninstall url-shortener-redis -n url-shortener-redis
    kubectl delete ns url-shortener-redis
    
    regex=$(printf "/%s/d" "short.home")
    cmd="sudo sed -i "$regex" /etc/hosts"
    echo "need permission to write in /etc/hosts:"
    echo $cmd
    $cmd
fi