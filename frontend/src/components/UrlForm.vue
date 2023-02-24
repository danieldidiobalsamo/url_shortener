<template>
  <div id="main-item">
    <form id="url-form" @submit.prevent="shortUrl">
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
    row-gap: 5%;

    height: 84vh;
    width: 100vw;
  }

  #url-form{
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    column-gap: 5em;
  }

  #main-item input, button{
    background: #2480FF;
    color: white;
    border-radius: 5px;
    border-color: #1578ff;
  }

  #main-item input{
    width: 35em;
    height: 2em;
  }

  #main-item input::placeholder{
    color: white;
    opacity: 0.7;
  }

  #main-item button{
    width: 10em;
    height: 2em;
  }

  #main-item button:hover{
      background-color: #1D66CC;
  }
</style>
