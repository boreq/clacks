<template>
  <div class="home">
    <h1>
      https://onlyclacks.com
    </h1>

    <ShuttersPreview :shutterPositions="shutterPositions" class="current-shutters" />
    <CurrentMessagePreview :message="message" />

    <div class="queue-separator">
      <ChevronUp v-for="_ in 9" :key="_" />
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

    <div class="message-form" v-if="messageFormLoading">
      <div class="loading-indicator">
          <LoadingIndicator></LoadingIndicator>
      </div>
    </div>
    <div class="message-form" v-if="messageFormLoadingError">
      <div class="error">
        Error loading config?! Try refreshing or something, I don't know, I'm just an error message.
      </div>
    </div>
    <div class="message-form" v-if="!messageFormLoading && !messageFormLoadingError">
      <input type="text"
        placeholder="ABC..."
        v-model="newMessageText"
        @keydown.enter="submitMessageForm">
      <button @click="submitMessageForm">
        <ChevronUp />
        <span class="text">ADD TO QUEUE</span>
        <ChevronUp />
      </button>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import { ChevronUp } from 'lucide-vue-next';
import {
  ShutterPositions, ShutterLocation, ShutterPosition, CurrentMessage, Message,
} from '@/types';
import { API, ConfigResponse } from '@/api';

import ShuttersPreview from '@/components/ShuttersPreview.vue';
import CurrentMessagePreview from '@/components/CurrentMessagePreview.vue';
import LoadingIndicator from '@/components/LoadingIndicator.vue';

enum NewMessageFormState {
    LoadingConfig,
    LoadingConfigError,
    Ready,
    Submitting,
}

export default defineComponent({
  name: 'HomeView',
  components: {
    ShuttersPreview,
    CurrentMessagePreview,
    ChevronUp,
    LoadingIndicator,
  },
  created(): void {
    this.api.getConfig()
      .then((response) => {
        this.config = response.data;
        this.newMessageFormState = NewMessageFormState.Ready;
      })
      .catch(() => {
        this.newMessageFormState = NewMessageFormState.LoadingConfigError;
      });
  },
  data() {
    const shutterPositions: ShutterPositions = {
      shutters: {
        [ShutterLocation.TopLeft]: ShutterPosition.Open,
        [ShutterLocation.TopRight]: ShutterPosition.Open,
        [ShutterLocation.MiddleLeft]: ShutterPosition.Closed,
        [ShutterLocation.MiddleRight]: ShutterPosition.Open,
        [ShutterLocation.BottomLeft]: ShutterPosition.Closed,
        [ShutterLocation.BottomRight]: ShutterPosition.Open,
      },
    };

    const after = [...'GNUTERRYPRATCHETT'].map((c) => ({ text: c, encoding: shutterPositions }));

    const message: CurrentMessage = {
      before: [],
      current: undefined,
      after,
    };

    const queue: Message[] = [
      {
        parts: [...'GNUTERRYPRATCHETT'].map((c) => ({ text: c, encoding: shutterPositions })),
      },
      {
        parts: [...'FREESIDE'].map((c) => ({ text: c, encoding: shutterPositions })),
      },
    ];

    return {
      shutterPositions,
      message,
      queue,
      api: new API(),
      config: null as ConfigResponse | null,

      newMessageFormState: NewMessageFormState.LoadingConfig,
      newMessageText: '',
    };
  },
  watch: {
    newMessageText(newValue: string, oldValue: string): void {
      const correctedNewValue = newValue
        .toUpperCase()
        .split('')
        .filter((char) => this.config!.supportedCharacters.includes(char))
        .join('');
      if (correctedNewValue.length < this.config!.maxMessageLenInBytes) {
        this.newMessageText = correctedNewValue;
      } else {
        this.newMessageText = oldValue;
      }
    },
  },
  methods: {
    submitMessageForm(): void {
      if (this.newMessageFormState !== NewMessageFormState.Ready) {
        return;
      }

      if (this.newMessageText.length === 0) {
        return;
      }

      this.newMessageFormState = NewMessageFormState.Submitting;

      this.api.addMessageToQueue({
        message: this.newMessageText,
      }).then((response) => {
        if (response.status !== 200) {
          alert('error');
          return;
        }
        this.newMessageText = '';
      }).catch(() => {
        alert('something went wrong');
      });

      console.log('submit');
    },
  },
  computed: {
    messageFormLoading(): boolean {
      return this.newMessageFormState === NewMessageFormState.LoadingConfig;
    },
    messageFormLoadingError(): boolean {
      return this.newMessageFormState === NewMessageFormState.LoadingConfigError;
    },
    messageFormSubmitting(): boolean {
      return this.newMessageFormState === NewMessageFormState.Submitting;
    },
  },
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
    background-color: $color-dark;

    .loading-indicator {
        flex: 1;
        text-align: center;
        height: 100px;
    }

    .error {
        text-align: center;
        flex: 1;
    }

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
