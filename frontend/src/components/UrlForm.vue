<template>
  <div id="main-item">
    <form id="url-form" @submit.prevent="shortUrl">
      <input autofocus type="text" placeholder="URL to shorten..." v-model="urlToShorten">
      <button disabled id="shorten-btn">Shorten</button>
    </form>
    <input type="text" :value='shortenedUrl' readonly>
    <span id="invalid-msg-tooltip"> invalid url </span>
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
  },

  watch:{
    urlToShorten(url){
      let btn = document.getElementById("shorten-btn")
      let tooltip = document.getElementById("invalid-msg-tooltip")

      if(isUrl(url)){
          btn.disabled = false;
          btn.style.opacity= 1;
          btn.style.cursor= "default";

          tooltip.style.visibility = "hidden";

      }
      else{
          btn.disabled = true;
          btn.style.opacity= 0.6;
          btn.style.cursor= "not-allowed";

          tooltip.style.visibility = "visible";
      }
    }
  }
}

function isUrl(input){
  try{
    const url = new URL(input)

    if(url.protocol != "http:" && url.protocol != "https:"){
      throw new Error("protocol must be http or https")
    }
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

  #main-item input, button, span{
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

    opacity: 0.6;
    cursor: not-allowed;
  }

  #main-item button:hover{
      background-color: #1D66CC;
  }

  #invalid-msg-tooltip{
    display: flex;
    justify-content: center;
    background-color: #ff9800;
    visibility: hidden;
  }

</style>
