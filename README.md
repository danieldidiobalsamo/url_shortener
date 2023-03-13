# How to launch
## If you have a kubernetes cluster

To install the chart and update your /etc/hosts file automatically, just launch :

~~~
sudo chmod +x install.sh
./install.sh
~~~

Then open http://short.home/

## If you don't have a kubernetes cluster

I recommend to use minikube.

- Install minikube : https://minikube.sigs.k8s.io/docs/start/
- Install kubectl : https://kubernetes.io/docs/tasks/tools/
- Install helm : https://helm.sh/docs/intro/quickstart/

~~~
minikube start
minikube addons enable ingress
~~~

To install the chart and update your /etc/hosts file automatically, just launch :

~~~
sudo chmod +x install.sh
./install.sh
~~~

You can now open http://short.home/

# How to uninstall

Simply call uninstall script :
~~~
sudo chmod +x uninstall.sh
./uninstall.sh
~~~