<template>
  <div class="home">
    <div class="call-to-action">
      Send your own message!
    </div>

    <img :src="qrCodeDataURL" v-if="qrCodeDataURL">

    <div class="url">
      https://<br/>onlyclacks.com
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import QRCode from 'qrcode';

export default defineComponent({
  name: 'HomeView',
  data() {
    return {
      qrCodeDataURL: null as null | string,
    };
  },
  created(): void {
    QRCode.toDataURL(
      'https://onlyclacks.com',
      {
        errorCorrectionLevel: 'H',
        width: 500,
        color: {
          light: '#ffff00',
        },
      },
    ).then((dataURL) => {
      this.qrCodeDataURL = dataURL;
    });
  },
});
</script>

<style lang="scss" scoped>
.home {
  background-color: $color-primary;

  width: 100vw;
  height: 100vh;
  display: flex;
  flex-flow: column nowrap;
  align-items: center;
  justify-content: center;

  color: $color-dark;
  text-transform: uppercase;
  font-weight: bold;

  .call-to-action {
    font-size: 3em;
    margin-bottom: 1em;
  }

  .url {
    font-size: 2em;
  }

  img {
    display: block;
    width: 20em;
    margin: 0 auto;
  }
}
</style>
