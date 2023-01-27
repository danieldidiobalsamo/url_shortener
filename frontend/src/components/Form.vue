<script>
export default {
  props: {
    msg: String
  }
}
</script>

<template>
  <h1>{{ msg || 'test'}}</h1>
</template>