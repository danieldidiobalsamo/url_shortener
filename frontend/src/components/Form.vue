<script>
export default {
  data(){
    return{
      urlToShorten: '',
      fullUrl: ''
    }
  },

  methods: {
    shortUrl(){
      console.log("shorten " + this.urlToShorten)
      this.fullUrl = 'TODO'
    }
  }
}
</script>

<template>
  <form @submit.prevent="shortUrl">
    <input type="text" placeholder="URL to shorten..." v-model="urlToShorten">
    <button>Shorten</button>
  </form>

  <input type="text" :value='fullUrl'>

</template>