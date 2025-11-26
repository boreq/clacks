<template>
  <div class="home">
    <div>
      Send your own message!
    </div>

    <img :src="qrCodeDataURL" v-if="qrCodeDataURL">
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

  width: 102vw;
  height: 102vh;
  display: flex;
  flex-flow: column nowrap;
  align-items: center;
  justify-content: center;

  margin-top: -1vh;
  margin-left: -1vw;

  div {
    font-size: 5em;
    color: $color-dark;
    margin-bottom: 1em;
    text-transform: uppercase;
    font-weight: bold;
  }

  img {
    display: block;
    width: 30em;
    margin: 0 auto;
  }
}
</style>
