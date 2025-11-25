import axios, { AxiosResponse } from 'axios';

export class API {
  getConfig(): Promise<AxiosResponse<ConfigResponse>> {
    return axios.get<ConfigResponse>(`${process.env.VUE_APP_BACKEND_URL}/api/config`);
  }

  addMessageToQueue(request: AddMessageToQueueRequest): Promise<AxiosResponse<void>> {
    return axios.post<void>(`${process.env.VUE_APP_BACKEND_URL}/api/quueue`, request);
  }
}

export interface ConfigResponse {
    supportedCharacters: string[];
    maxMessageLenInBytes: number;
}

export interface AddMessageToQueueRequest {
    message: string;
}
