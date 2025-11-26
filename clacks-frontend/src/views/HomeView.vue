<template>
  <div class="home">
    <h1>
      https://onlyclacks.com
    </h1>

    <h2>
      <a href="https://github.com/boreq/clacks">https://github.com/boreq/clacks</a>
      -
      <span>
        Am I broken? <Phone class="icon"></Phone> 2677
      </span>
    </h2>

    <div class="visualisation">
      <div class="loading-indicator" v-if="visualisationLoading">
        <LoadingIndicator></LoadingIndicator>
      </div>

      <div class="error" v-if="visualisationError">
        Error loading the visualisation?! Attempting to reestablish connection.
        Try refreshing or something, I don't know, I'm just an error message.
      </div>

      <ShuttersPreview
        :openShutters="update?.currentMessage?.current?.openShutters"
        class="current-shutters" />
      <CurrentMessagePreview :message="update?.currentMessage" />

      <div
        class="queue-separator"
        :class="{'changing-message': changingMessage}">
        <ChevronUp v-for="_ in 9" :key="_" />
      </div>

      <div v-if="update?.queue.length === 0" class="queue-call-to-action">
        <div>
          Try adding something to the queue!
        </div>
        <ArrowDown class="arrow"></ArrowDown>
      </div>

      <ul class="queue">
        <li
          v-for="(message, index) in update?.queue"
          :key="message.parts.map(v => `${v.kind}${v.character}`).join('-')"
          class="entry">
          <div class="index">
            {{ index + 1 }}.
          </div>
          <ul class="message">
            <li v-for="part in message.parts" :key="part.kind + part.character">
              <MessagePartPreview :message_part="part"></MessagePartPreview>
            </li>
          </ul>
        </li>
      </ul>
    </div>

    <div class="message-form" v-if="messageFormLoading">
      <div class="loading-indicator">
          <LoadingIndicator></LoadingIndicator>
      </div>
    </div>
    <div class="message-form" v-if="messageFormLoadingError">
      <div class="error">
        Error loading config?! Attempting to reestablish connection.
        Try refreshing or something, I don't know, I'm just an error message.
      </div>
    </div>
    <div class="message-form" v-if="!messageFormLoading && !messageFormLoadingError">
      <input type="text"
        placeholder="YOU C-MAIL MESSAGE HERE..."
        v-model="newMessageText"
        @keydown.enter="submitMessageForm"
      >
      <button @click="submitMessageForm">
        <ChevronUp />
        <span class="text">SEND</span>
        <ChevronUp />
        <LoadingIndicator v-if="messageFormSubmitting"></LoadingIndicator>
      </button>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import { ChevronUp, ArrowDown, Phone } from 'lucide-vue-next';
import { API, ConfigResponse, StateUpdate } from '@/api';

import ShuttersPreview from '@/components/ShuttersPreview.vue';
import CurrentMessagePreview from '@/components/CurrentMessagePreview.vue';
import LoadingIndicator from '@/components/LoadingIndicator.vue';
import MessagePartPreview from '@/components/MessagePartPreview.vue';

enum NewMessageFormState {
    LoadingConfig,
    LoadingConfigError,
    Ready,
    Submitting,
}

enum VisualisationState {
  Loading,
  Error,
  Ready,
}

export default defineComponent({
  name: 'HomeView',
  components: {
    ShuttersPreview,
    CurrentMessagePreview,
    LoadingIndicator,
    MessagePartPreview,

    ChevronUp,
    ArrowDown,
    Phone,
  },
  data() {
    return {
      api: new API(),

      update: null as StateUpdate | null,
      visualisationState: null as VisualisationState | null,

      config: null as ConfigResponse | null,
      newMessageFormState: null as NewMessageFormState | null,
      newMessageText: '',

      intervalID: null as number | null,
    };
  },
  created(): void {
    this.tryLoad();
    this.intervalID = window.setInterval(this.tryLoad, 1000);
  },
  unmounted(): void {
    if (this.intervalID) {
      window.clearInterval(this.intervalID);
    }
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
    tryLoad(): void {
      if (this.newMessageFormState === null
        || this.newMessageFormState === NewMessageFormState.LoadingConfigError) {
        this.newMessageFormState = NewMessageFormState.LoadingConfig;
        this.api.getConfig()
          .then((response) => {
            this.config = response.data;
            this.newMessageFormState = NewMessageFormState.Ready;
          })
          .catch(() => {
            this.newMessageFormState = NewMessageFormState.LoadingConfigError;
          });
      }

      if (this.visualisationState === null
          || this.visualisationState === VisualisationState.Error) {
        const socket = this.api.stateUpdatesWS();

        socket.addEventListener('message', (event) => {
          this.visualisationState = VisualisationState.Ready;
          this.update = JSON.parse(event.data);
        });

        socket.addEventListener('error', () => {
          this.visualisationState = VisualisationState.Error;
        });

        socket.addEventListener('close', () => {
          this.visualisationState = VisualisationState.Error;
        });
      }
    },
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
        this.newMessageFormState = NewMessageFormState.Ready;
      }).catch(() => {
        alert('something went wrong');
        this.newMessageFormState = NewMessageFormState.Ready;
      });
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
    visualisationLoading(): boolean {
      return this.visualisationState === VisualisationState.Loading;
    },
    visualisationError(): boolean {
      return this.visualisationState === VisualisationState.Error;
    },
    changingMessage(): boolean {
      return !this.update?.currentMessage && !!this.update?.queue && this.update?.queue.length > 0;
    },
  },
});
</script>

<style scoped lang="scss">
h1, h2 {
  text-transform: uppercase;
}

h1 {
  color: $color-primary;
  font-weight: bold;
}

h2 {
  color: $color-neutral;
  font-size: .8em;

  a {
    color: $color-primary;
  }

  .icon {
    width: .8em;
    height: .8em;
  }
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
    margin: 5em 0;

    >* {
        flex: 1;
    }

    &.changing-message {
        animation: changing-message-blink-animation .5s steps(1) infinite;
    }
}

.visualisation {
  position: relative;

  >.loading-indicator, >.error {
    background-color: $color-dark;
    position: absolute;
    left: 0;
    right: 0;
    top: 0;
    bottom: 0;
  }

  .error {
    font-size: 3em;
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
        position: relative;

        .text {
          display: inline-block;
          padding: 0 .5em;
        }

        .loading-indicator {
          position: absolute;
          left: 0;
          top: 0;
          right: 0;
          bottom: 0;
          height: 100%;
          background-color: $color-dark;
        }

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
    display: flex;
    align-items: center;
    flex-flow: column nowrap;
    font-size: 3em;

    .entry {
      display: flex;
      flex-flow: row nowrap;
      align-items: center;

      .index {
        width: auto;
        color: $color-primary;
      }

      .message {
        list-style-type: none;
        margin: 0;
        padding: 0;
        display: flex;
        flex-flow: row wrap;
        align-items: center;

        li {
          margin: 0;
          padding: 0;
          font-weight: bold;
        }
      }
    }

}

.queue-call-to-action {
  font-size: 4em;
  text-transform: uppercase;
  display: flex;
  flex-flow: column nowrap;
  align-items: center;
  animation: call-to-action-blink-animation 1s steps(5, start) infinite;

  .arrow {
    display: block;
    width: 2em;
    height: 2em;
  }
}

@keyframes call-to-action-blink-animation {
  to {
    visibility: hidden;
  }
}

@keyframes changing-message-blink-animation {
  0% {
    color: $color-dark;
  }
  50% {
    color: $color-primary;
  }
  100% {
    color: $color-dark;
  }
}
</style>
