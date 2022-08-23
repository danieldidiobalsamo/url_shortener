# If you have a kubernetes cluster

Launch `install.sh` script, which will deploy application in a separate namespace, and install cert-manager.

~~~
sudo chmod +x install.sh
./install.sh
~~~

Then open https://url-shortener-rust.com

# If you don't have a kubernetes cluster

I recommend to use minikube.

Install minikube : https://minikube.sigs.k8s.io/docs/start/

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