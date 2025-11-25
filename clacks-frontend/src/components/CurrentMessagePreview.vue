<template>
  <div class="current-message-preview">
    <ul class="before">
      <li v-for="part in message.before" :key="part.text">
        {{ part.text }}
      </li>
    </ul>

    <div class="arrow">
      <ArrowLeft v-if="message.before.length" />
    </div>

    <div class="current">
      <span v-if="message.current">
        {{ message.current.text }}
      </span>
    </div>

    <div class="arrow">
      <ArrowLeft v-if="message.after.length" />
    </div>

    <ul class="after">
      <li v-for="part in message.after" :key="part.text">
        {{ part.text }}
      </li>
    </ul>
  </div>
</template>

<script lang="ts">
import { defineComponent, PropType } from 'vue';
import { ArrowLeft } from 'lucide-vue-next';
import { CurrentMessage } from '@/types';

export default defineComponent({
  name: 'CurrentMessagePreview',
  components: {
    ArrowLeft,
  },
  props: {
    message: Object as PropType<CurrentMessage>,
  },
});
</script>

<style scoped lang="scss">
.current-message-preview {
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
