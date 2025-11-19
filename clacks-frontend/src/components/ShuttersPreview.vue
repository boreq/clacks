<template>
  <table class="shutters-preview">
      <tbody>
          <tr>
              <ShutterPreview :shutterPosition="topLeftPosition"></ShutterPreview>
              <ShutterPreview :shutterPosition="topRightPosition"></ShutterPreview>
          </tr>
          <tr>
              <ShutterPreview :shutterPosition="middleLeftPosition"></ShutterPreview>
              <ShutterPreview :shutterPosition="middleRightPosition"></ShutterPreview>
          </tr>
          <tr>
              <ShutterPreview :shutterPosition="bottomLeftPosition"></ShutterPreview>
              <ShutterPreview :shutterPosition="bottomRightPosition"></ShutterPreview>
          </tr>
      </tbody>
  </table>
</template>

<script lang="ts">
import { defineComponent, PropType } from 'vue';
import { ShutterPositions, ShutterPosition, ShutterLocation } from '@/types';
import ShutterPreview from '@/components/ShutterPreview.vue';

export default defineComponent({
  name: 'ShuttersPreview',
  components: {
      ShutterPreview,
  },
  props: {
    shutterPositions: Object as PropType<ShutterPositions>,
  },
  computed: {
    topLeftPosition(): ShutterPosition {
        return this.shutterPositions!.shutters[ShutterLocation.TopLeft];
    },
    topRightPosition(): ShutterPosition {
        return this.shutterPositions!.shutters[ShutterLocation.TopRight];
    },
    middleLeftPosition(): ShutterPosition {
        return this.shutterPositions!.shutters[ShutterLocation.MiddleLeft];
    },
    middleRightPosition(): ShutterPosition {
        return this.shutterPositions!.shutters[ShutterLocation.MiddleRight];
    },
    bottomLeftPosition(): ShutterPosition {
        return this.shutterPositions!.shutters[ShutterLocation.BottomLeft];
    },
    bottomRightPosition(): ShutterPosition {
        return this.shutterPositions!.shutters[ShutterLocation.BottomRight];
    },
  },
});
</script>

<style scoped lang="scss">
table {
    border-spacing: 1em;

    td {
        border: 5px solid $color-neutral;
        width: 10vh;
        height: 10vh;
    }
}
</style>
