# How to install

This project must run in an ingress enabled kubernetes cluster. If you don't have one you can quickly create it with minikube.

I strongly recommend to use the install script since it setups everything for you: application charts installation and /etc/hosts update with ingress ip. Helm and kubectl must be installed to use it.

~~~
# if you want to use minikube
# minikube start
# minikube addons enable ingress

sudo chmod +x install.sh
./install.sh
~~~