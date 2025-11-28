import { DisplayedError } from '@/types';
import { createStore, Store } from 'vuex';

export default createStore({
  state: {
    errors: [] as DisplayedError[],
  },
  getters: {
  },
  mutations: {
    addError(state, message: string): void {
      state.errors.push({
        message,
        createdAt: new Date(),
      });
    },
    clearOldErrors(state): void {
      state.errors = state.errors.filter((v) => !isOlderThan(v.createdAt, 10));
    },
  },
  actions: {
  },
  modules: {
  },
});

function isOlderThan(date: Date, seconds: number): boolean {
  return (Date.now() - new Date(date).getTime()) > seconds * 1000;
}

export type ClacksStore = Store<{
  errors: DisplayedError[],
}>;
