<template>
  <table class="shutters-preview">
    <tbody>
      <tr>
        <ShutterPreview :shutterPosition="topLeftPosition" />
        <ShutterPreview :shutterPosition="topRightPosition" />
      </tr>
      <tr>
        <ShutterPreview :shutterPosition="middleLeftPosition" />
        <ShutterPreview :shutterPosition="middleRightPosition" />
      </tr>
      <tr>
        <ShutterPreview :shutterPosition="bottomLeftPosition" />
        <ShutterPreview :shutterPosition="bottomRightPosition" />
      </tr>
    </tbody>
  </table>
</template>

<script lang="ts">
import { defineComponent, PropType } from 'vue';
import { ShutterLocation, ShutterPosition } from '@/types';
import ShutterPreview from '@/components/ShutterPreview.vue';

export default defineComponent({
  name: 'ShuttersPreview',
  components: {
    ShutterPreview,
  },
  props: {
    openShutters: Array as PropType<ShutterLocation[]>,
  },
  methods: {
    getShutterPosition(location: ShutterLocation): ShutterPosition {
      if (this.openShutters?.includes(location)) {
        return ShutterPosition.Open;
      }
      return ShutterPosition.Closed;
    },
  },
  computed: {
    topLeftPosition(): ShutterPosition {
      return this.getShutterPosition(ShutterLocation.TopLeft);
    },
    topRightPosition(): ShutterPosition {
      return this.getShutterPosition(ShutterLocation.TopRight);
    },
    middleLeftPosition(): ShutterPosition {
      return this.getShutterPosition(ShutterLocation.MiddleLeft);
    },
    middleRightPosition(): ShutterPosition {
      return this.getShutterPosition(ShutterLocation.MiddleRight);
    },
    bottomLeftPosition(): ShutterPosition {
      return this.getShutterPosition(ShutterLocation.BottomLeft);
    },
    bottomRightPosition(): ShutterPosition {
      return this.getShutterPosition(ShutterLocation.BottomRight);
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
