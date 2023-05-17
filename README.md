# About

This fullstack project consists in a url shortener web app which runs in Kubernetes.
The following stack is used:
- Deployment: Kubernetes / Helm
- Backend: Rust / Actix Web
- Frontend: Vue.js
- Storage: Redis
- CI: Jenkins

User inputs are sanitized on both frontend and backend, and some basic security has been applied to pods such as dropping containers capabilities.
Backend [Dockerfile](backend/Dockerfile) takes advantage of dependencies caching using cargo chef plugin.

The app is deployed in "url-shortener" namespace, which contains backend, frontend and Redis StatefulSet pods.
The main chart is only used to define the app endpoint, and to group front and back as dependencies.

# How to install

This project must run in an ingress enabled kubernetes cluster. If you don't have one you can quickly create it with [minikube](https://minikube.sigs.k8s.io/docs/start/).

You don't have to build anything since everything is pushed on dockerhub and artifacthub (helm repository is [here](https://github.com/danieldidiobalsamo/helm-charts-repo)), and the install script setups everything for you: charts installation and /etc/hosts file update with app domain name. 
[Helm](https://helm.sh/docs/intro/quickstart/) and [kubectl](https://kubernetes.io/docs/tasks/tools/) must be installed to use it.

~~~
# [optional] if you want to use minikube
# minikube start
# minikube addons enable ingress

sudo chmod +x install.sh
./install.sh
~~~

# How to uninstall

The uninstall script removes the charts, scales down and removes the redis statefulset (including the datas) and the hostname in /etc/hosts.

~~~
sudo chmod +x uninstall.sh
./uninstall.sh
~~~