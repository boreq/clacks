import axios, { AxiosResponse } from 'axios';
import { CurrentMessage, Message } from '@/types';

export class API {
  getConfig(): Promise<AxiosResponse<ConfigResponse>> {
    return axios.get<ConfigResponse>(`${process.env.VUE_APP_BACKEND_URL}/api/config`);
  }

  addMessageToQueue(request: AddMessageToQueueRequest): Promise<AxiosResponse<ErrorResponse>> {
    return axios.post<ErrorResponse>(`${process.env.VUE_APP_BACKEND_URL}/api/queue`, request);
  }

  stateUpdatesWS(): WebSocket {
    return new WebSocket(`${process.env.VUE_APP_BACKEND_URL}/api/state-updates`);
  }
}

export interface ConfigResponse {
    supportedCharacters: string[];
    maxMessageLenInBytes: number;
}

export interface AddMessageToQueueRequest {
    message: string;
}

export interface StateUpdate {
    currentMessage?: CurrentMessage;
    queue: Message[];
}

export interface ErrorResponse {
    message: string;
}
