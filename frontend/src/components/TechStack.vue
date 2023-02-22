<template>
  <div id="tech-stack">
    <img :src="rustImg">
    <img :src="redisImg">
    <img :src="vueImg">
    <img :src="k8sImg">
    <img :src="helmImg">
  </div>
</template>

<script>
  import redisImg from "../assets/redis.png"
  import rustImg from "../assets/rust.png"
  import vueImg from "../assets/vue.png"
  import k8sImg from "../assets/k8s.png"
  import helmImg from "../assets/helm.png"

  export default {
    data(){
      return{
        redisImg: redisImg,
        rustImg: rustImg,
        vueImg: vueImg,
        k8sImg: k8sImg,
        helmImg: helmImg,
      }
    }
  }
</script>

<style type="text/css">
  #tech-stack{
    height: 8vh;
    width: 100vw;

    display: flex;
    justify-content: space-evenly;
  }
</style>