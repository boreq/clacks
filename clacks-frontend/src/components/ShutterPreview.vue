<template>
  <td :class="{ 'shutter-preview': true, open, closed }" />
</template>

<script lang="ts">
import { defineComponent, PropType } from 'vue';
import { ShutterPosition } from '@/types';

export default defineComponent({
  name: 'ShutterPreview',
  props: {
    shutterPosition: String as PropType<ShutterPosition>,
  },
  computed: {
    open(): boolean {
      return this.shutterPosition === ShutterPosition.Open;
    },
    closed(): boolean {
      return this.shutterPosition === ShutterPosition.Closed;
    },
  },
});
</script>

<style scoped lang="scss">
.shutter-preview {
    &.open {
        background-color: $color-neutral;
    }
}
</style>
