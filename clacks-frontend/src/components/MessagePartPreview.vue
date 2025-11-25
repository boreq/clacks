<template>
  <div class="message-part-preview">
    <div v-if="message_part?.kind == 'CHARACTER' && message_part.character !== ' '">
        {{ message_part?.character }}
    </div>
    <div v-if="message_part?.kind == 'CHARACTER' && message_part.character === ' '">
        <Space></Space>
    </div>
    <div v-if="message_part?.kind == 'END'" class="end">
      <div>
        E
      </div>
      <div>
        N
      </div>
      <div>
        D
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent, PropType } from 'vue';
import { MessagePart } from '@/types';
import { Space } from 'lucide-vue-next';

export default defineComponent({
  name: 'CurrentMessagePreview',
  components: {
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

  display: flex;
  align-items: center;
  justify-content: center;

  >div {
    line-height: 1;

    &.end {
      display: flex;
      flex-flow: column nowrap;
      align-items: center;
      font-size: .35em;
      line-height: 1;

      :nth-child(1) {
        transform: translateX(-1em);
      }

      :nth-child(3) {
        transform: translateX(1em);
      }
    }
  }
}
</style>
