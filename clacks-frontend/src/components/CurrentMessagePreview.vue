<template>
  <div class="current-message-preview" :class="{'changing-character': !message?.current}">
    <ul class="before">
      <li v-for="part in message?.before" :key="part.kind + part.character">
        <MessagePartPreview :message_part="part"></MessagePartPreview>
      </li>
    </ul>

    <div class="arrow">
      <ArrowLeft v-if="message?.before.length" />
    </div>

    <div class="current">
      <span v-if="message?.current">
        <MessagePartPreview :message_part="message?.current"></MessagePartPreview>
      </span>
    </div>

    <div class="arrow">
      <ArrowLeft v-if="message?.after.length" />
    </div>

    <ul class="after">
      <li v-for="part in message?.after" :key="part.kind + part.character">
        <MessagePartPreview :message_part="part"></MessagePartPreview>
      </li>
    </ul>
  </div>
</template>

<script lang="ts">
import { defineComponent, PropType } from 'vue';
import { ArrowLeft } from 'lucide-vue-next';
import { CurrentMessage } from '@/types';
import MessagePartPreview from '@/components/MessagePartPreview.vue';

export default defineComponent({
  name: 'CurrentMessagePreview',
  components: {
    MessagePartPreview,
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

    >* {
      display: flex;
      flex-flow: row wrap;
      align-items: center;
    }

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
            text-align: center;
        }
    }

    .arrow {
        width: 50px;
        justify-content: center;
    }

    .current {
      justify-content: center;
    }

    .before, .after {
        flex: 1;
    }

    .before {
      justify-content: end;
    }

    .after {
      justify-content: start;
    }

    .current {
        color: $color-primary;
        font-size: 12em;
        font-weight: bold;
        width: 300px;
        height: 300px;
    }

  &.changing-character {
    .arrow {
      color: $color-primary;
      animation: blink-animation .5s steps(1) infinite;
    }
  }
}

@keyframes blink-animation {
  0% {
    visibility: visible;
  }
  50% {
    visibility: hidden;
  }
  100% {
    visibility: visible;
  }
}
</style>
