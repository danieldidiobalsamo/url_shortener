<template>
  <header>
    hello header
  </header>
  <main>
    <form @submit.prevent="shortUrl">
        <input type="text" placeholder="URL to shorten..." v-model="urlToShorten">
        <button>Shorten</button>
    </form>
    <input type="text" :value='shortenedUrl'>
  </main>
  <footer>
    hello footer
  </footer>
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
  *{
    font-size: 20px;
  }

  body{
    margin: 0;
  }

  main{
    display: flex;
    flex-direction: column;
    flex-wrap: wrap;
    align-content: center;
    justify-content: center;
    height: 80vh;
    width: 100vw;
  }

  header{
    height: 10vh;
    width: 100vw;
  }

  footer{
    height: 10vh;
    width: 100vw;
  }
</style>
