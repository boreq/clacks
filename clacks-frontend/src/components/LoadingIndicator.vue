<template>
  <div class="loading-indicator">
      <img src="/cursor-loading.png"/>
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';

export default defineComponent({
  name: 'LoadingIndicator',
});
</script>

<style scoped lang="scss">
.loading-indicator {
  img {
    height: 100%;
    animation: turn 2s linear infinite;
  }

  @keyframes turn {
    0% {
      transform: rotate(0deg);
    }
    80% {
      transform: rotate(0deg);
    }
    100% {
      transform: rotate(180deg);
    }
  }
}
</style>
