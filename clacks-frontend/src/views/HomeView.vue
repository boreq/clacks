<template>
  <div class="home">
      <h1>
          https://onlyclacks.com
      </h1>

      <ShuttersPreview :shutterPositions="shutterPositions" class="current-shutters"></ShuttersPreview>
      <CurrentMessagePreview :message="message"></CurrentMessagePreview>

      <div class="queue-separator">
          <ChevronUp v-for="_ in 9" :key="_"></ChevronUp>
      </div>

      <div v-if="!queue || queue.length === 0" class="message">
          The queue is empty.
      </div>

      <ul class="queue">
          <li v-for="(message, index) in queue" :key="message.parts.map(v => v.text).join('-')">
              <ul class="message">
                  <li class="index">
                      {{ index + 1 }}.
                  </li>
                  <li v-for="part in message.parts" :key="part.text">
                      {{ part.text }}
                  </li>
              </ul>
          </li>
      </ul>

      <div class="message-form">
          <input type="text" placeholder="ABC..." v-model="newMessageText">
          <button>
              <ChevronUp></ChevronUp>
              <span class="text">ADD TO QUEUE</span>
              <ChevronUp></ChevronUp>
          </button>
      </div>
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import { ShutterPositions, ShutterLocation, ShutterPosition, CurrentMessage, Message } from '@/types';
import { ChevronUp } from 'lucide-vue-next';
import ShuttersPreview from '@/components/ShuttersPreview.vue'; 
import CurrentMessagePreview from '@/components/CurrentMessagePreview.vue';
import { API, ConfigResponse } from "@/api";

export default defineComponent({
  name: 'HomeView',
  components: {
      ShuttersPreview,
      CurrentMessagePreview,
      ChevronUp,
  },
  created(): any {
    this.api.getConfig()
        .then(response => {
          this.config = response.data;
        })
  },
  mounted(): void {
    window.setInterval(this.updateData, 2000);
  },
  data() {
    let shutterPositions: ShutterPositions = {
        shutters: {
            [ShutterLocation.TopLeft]: ShutterPosition.Open,
            [ShutterLocation.TopRight]: ShutterPosition.Open,
            [ShutterLocation.MiddleLeft]: ShutterPosition.Closed,
            [ShutterLocation.MiddleRight]: ShutterPosition.Open,
            [ShutterLocation.BottomLeft]: ShutterPosition.Closed,
            [ShutterLocation.BottomRight]: ShutterPosition.Open,
        }
    };


    let after = [..."GNUTERRYPRATCHETT"].map(c => {
        return {text: c, encoding: shutterPositions};
    });

    let message: CurrentMessage = {
        before: [],
        current: undefined,
        after: after,
    }

    let queue: Message[] = [
        {
            parts: [..."GNUTERRYPRATCHETT"].map(c => {
                return {text: c, encoding: shutterPositions};
            }),
        },
        {
            parts: [..."FREESIDE"].map(c => {
                return {text: c, encoding: shutterPositions};
            }),
        },
    ];

    return {
        shutterPositions,
        message,
        queue,
        api: new API(),
        config: null as ConfigResponse | null,
        newMessageText: '',
    };
  },
  watch: {
    newMessageText(newValue): void {
      this.newMessageText = this.newMessageText
          .toUpperCase()
          .split('')
          .filter((char, index) => this.config?.supportedCharacters.includes(char) && index < this.config?.maxMessageLenInBytes)
          .join('');
    },
  },
  methods: {
    updateData() {
        let shutterPositions: ShutterPositions = {
            shutters: {
                [ShutterLocation.TopLeft]: Math.random() < 0.5 ? ShutterPosition.Open : ShutterPosition.Closed,
                [ShutterLocation.TopRight]: Math.random() < 0.5 ? ShutterPosition.Open : ShutterPosition.Closed,
                [ShutterLocation.MiddleLeft]: Math.random() < 0.5 ? ShutterPosition.Open : ShutterPosition.Closed,
                [ShutterLocation.MiddleRight]: Math.random() < 0.5 ? ShutterPosition.Open : ShutterPosition.Closed,
                [ShutterLocation.BottomLeft]: Math.random() < 0.5 ? ShutterPosition.Open : ShutterPosition.Closed,
                [ShutterLocation.BottomRight]: Math.random() < 0.5 ? ShutterPosition.Open : ShutterPosition.Closed,
            }
        };

        this.shutterPositions = shutterPositions;

        if (this.message.current) {
            this.message.before.push(this.message.current);
            this.message.current = undefined;
            return;
        }

        if (!this.message.current && this.message.after.length > 0) {
            this.message.current = this.message.after.shift();
            return;
        }

        if (!this.message.current && this.message.after.length == 0) {
            this.message.after = this.message.before;
            this.message.before = [];
        }
    },
  }
});
</script>

<style scoped lang="scss">
h1 {
    color: $color-primary;
    text-transform: uppercase;
    font-weight: bold;
}

@media not (display-mode: fullscreen) {
    h1 {
        display: none;
    }
}

.current-shutters {
    margin: 5em auto 0;
}

.current-message{
    margin: 0 auto 5em;
}

.queue-separator {
    display: flex;
    flex-flow: row nowrap;
    color: $color-primary;
    margin: 5em 0;

    >* {
        flex: 1;
    }
}

.message-form {
    position: absolute;
    right: 0;
    left: 0;
    bottom: 0;
    padding: 1em;
    display: flex; 
    align-items: stretch;

    input, button {
        display: block;
        margin: 0; 
        display: 0;
        border: 1px solid $color-primary;
        background-color: transparent;
        color: $color-neutral;
        font-family: inherit;
        font-size: 2em;
        padding: .5em;
    }

    input {
        flex: 1;
        text-align: center;

        &:focus {
          outline: none;
        }
    }

    button {
        border-left: 0;
        user-select: none;

        &:hover {
            color: $color-dark;
            background-color: $color-primary;
            cursor: pointer;
        }

        &:active {
          color: $color-primary;
          background-color: $color-dark;
        }
    }
}

.queue {
    list-style-type: none;
    margin: 0;
    padding: 0;

    .message {
        list-style-type: none;
        margin: 0;
        padding: 0;

        .index {
            width: auto;
            color: $color-primary;
        }

        li {
            display: inline-block;
            margin: 0;
            padding: 0;
            font-weight: bold;
            font-size: 3em;
            width: 1em;
        }
    }
}

.message {
    padding: 2em 0;
    text-transform: uppercase;
}
</style>
