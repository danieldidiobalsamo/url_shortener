<template>
  <div id="about-me">
    <a href="https://github.com/danieldidiobalsamo"><img :src="githubImg"/></a>
    <a href="https://fr.linkedin.com/in/danieldidiobalsamo/en"><img :src="linkedinImg"/></a>
  </div>
</template>

<script>
  import linkedinImg from "../assets/linkedin.png"
  import githubImg from "../assets/github.png"

  export default {
    data(){
        return{
          linkedinImg: linkedinImg,
          githubImg: githubImg,
        }
    }
  }

</script>

<style type="text/css">
  #about-me{
    height: 8vh;
    width: 100vw;

    display: flex;
    justify-content: flex-end;
    align-items: center;
    gap: 1%;
  }
</style>