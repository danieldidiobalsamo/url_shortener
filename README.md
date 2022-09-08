# How to launch
## If you have a kubernetes cluster

Launch `install.sh` script, which will deploy application in a separate namespace, and install cert-manager.

~~~
sudo chmod +x install.sh
./install.sh
~~~

Then open https://url-shortener-rust.com

## If you don't have a kubernetes cluster

I recommend to use minikube.

- Install minikube : https://minikube.sigs.k8s.io/docs/start/
- Install kubectl : https://kubernetes.io/docs/tasks/tools/
- Install helm : https://helm.sh/docs/intro/quickstart/

~~~
minikube start
minikube addons enable ingress
~~~

Then launch `install.sh` script, which will deploy application in a separate namespace, and install cert-manager.

~~~
sudo chmod +x install.sh
./install.sh
~~~

You can now open https://url-shortener-rust.com

# How to uninstall

Simply call uninstall script :
~~~
sudo chmod +x uninstall.sh
./uninstall.sh
~~~