<template>
  <div id="main-item">
    <form @submit.prevent="shortUrl">
        <input type="text" placeholder="URL to shorten..." v-model="urlToShorten">
        <button>Shorten</button>
    </form>
    <input type="text" :value='shortenedUrl'>
  </div>
</template>

<script>
export default {
  data(){
    return{
      urlToShorten: '',
      shortenedUrl: ''
    }
  },

  methods: {
    shortUrl(){
      if(isUrl(this.urlToShorten)){
        const server = "http://short.home.backend"
        fetch(server + "/encode/" + encodeURIComponent(this.urlToShorten), {
          method: "GET",
          mode: "cors",
          referrer: "no-referrer"
        })
        .then(res => {
          if(res.ok){
            return res.text()
          }
          else{
            throw new Error("Received HTTP " + res.status + " " + res.statusText)
          }
        })
        .then(res =>{
          this.shortenedUrl = server + "/decode/" + res;
        }).catch((err)=>{
          alert(err)
        })
      }
      else{
        alert("Bad url")
      }
    }
  }
}

function isUrl(input){
  try{
    new URL(input)
  }
  catch(err){
    return false;
  }

  return true;
}
</script>

<style type="text/css">
  #main-item{
    display: flex;
    flex-direction: column;
    flex-wrap: wrap;
    align-content: center;
    justify-content: center;
    height: 84vh;
    width: 100vw;
  }
</style>
