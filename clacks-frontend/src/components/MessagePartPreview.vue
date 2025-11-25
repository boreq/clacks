<template>
  <div class="message-part-preview">
    <div v-if="message_part?.kind == 'CHARACTER'">
      <div v-if="message_part?.character !== ' '">
        {{ message_part?.character }}
      </div>
      <div v-if="message_part?.character === ' '">
        SPACE
      </div>
    </div>
    <div v-if="message_part?.kind == 'END'">
      END
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent, PropType } from 'vue';
import { MessagePart } from '@/types';

export default defineComponent({
  name: 'CurrentMessagePreview',
  props: {
    message_part: Object as PropType<MessagePart>,
  },
});
</script>

<style scoped lang="scss">
.message-part-preview {
    display: flex;
    align-items: center;
    height: 150px;

    ul {
        list-style-type: none;
        margin: 0;
        padding: 0;

        li {
            font-size: 3em;
            font-weight: bold;
            display: inline-block;
            margin: 0;
            padding: 0;
            width: 1em;
            text-align: center;
        }
    }

    .arrow {
        width: 100px;
        text-align: center;
    }

    .before, .after {
        flex: 1;
    }

    .before {
        text-align: right;
    }

    .after {
        text-align: left;
    }

    .current {
        color: $color-primary;
        font-size: 12em;
        font-weight: bold;
        width: 100px;
    }
}
</style>
