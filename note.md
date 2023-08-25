
## service discovery


use consul to implement service discovery

```shell
docker pull consul:1.15.4
docker run \
-d \
-p 8500:8500 \
-p 8600:8600/udp \
--name=badger \
consul:1.15.4 agent -server -ui -node=server-1 -bootstrap-expect=1 -client=0.0.0.0
```

consul also support load balance and health check


## front-load balance
gateway load balance: 


check with this [link](https://learn.hashicorp.com/tutorials/consul/load-balancing-nginx?in=consul/load-balancing)use this
use this method, can implement the architecture like this:

![img.png](https://developer.hashicorp.com/_next/image?url=https%3A%2F%2Fcontent.hashicorp.com%2Fapi%2Fassets%3Fproduct%3Dtutorials%26version%3Dmain%26asset%3Dpublic%252Fimg%252Fconsul%252Fnginx-plus%252Finfra-nginx-plus.png%26width%3D1484%26height%3D1080&w=3840&q=75)


## back-load balance
use kubernetes `Service`  to implement back-load balance, such as `ClusterIP` and `NodePort`