<template>
  <div class="message-part-preview">
    <div v-if="message_part?.kind == 'CHARACTER' && message_part.character !== ' '">
        {{ message_part?.character }}
    </div>
    <div v-if="message_part?.kind == 'CHARACTER' && message_part.character === ' '">
        <Space></Space>
    </div>
    <div v-if="message_part?.kind == 'END'" class="end">
      <div class="line">
        <div>
          S
        </div>
        <div>
          T
        </div>
      </div>
      <div class="line">
        <div>
          O
        </div>
        <div>
          P
        </div>
      </div>
    </div>

    <div class="shutters-wrapper">
      <ShuttersPreview
        :openShutters="message_part?.openShutters"
        class="shutters">
      </ShuttersPreview>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent, PropType } from 'vue';
import { MessagePart } from '@/types';
import { Space } from 'lucide-vue-next';
import ShuttersPreview from '@/components/ShuttersPreview.vue';

export default defineComponent({
  name: 'CurrentMessagePreview',
  components: {
    ShuttersPreview,
    Space,
  },
  props: {
    message_part: Object as PropType<MessagePart>,
  },
});
</script>

<style scoped lang="scss">
.message-part-preview {
  margin: .1em;
  border: 1px solid $color-neutral;
  width: 1.2em;
  height: 1.2em;
  position: relative;

  display: flex;
  align-items: center;
  justify-content: center;

  overflow: hidden;

  >div {
    line-height: 1;

    &.end {
      display: flex;
      flex-flow: column nowrap;
      align-items: center;
      font-size: .5em;
      line-height: 1;

      .line {
        display: flex;
        flex-flow: row nowrap;
        align-items: center;
        justify-content: center;
      }
    }
  }

  .shutters-wrapper {
    display: none;

    position: absolute;
    top: 0;
    bottom: 0;
    left: 0;
    right: 0;

    align-items: center;
    justify-content: center;

    background-color: $color-dark;

    :deep(.shutters-preview) {
      border-spacing: .1em;

      td {
        border-width: 1px;
        width: .2em;
        height: .2em;
        padding: 0;
      }
    }
  }

  &:hover {
    .shutters-wrapper {
      display: flex;
    }
  }
}
</style>
