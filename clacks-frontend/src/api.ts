import axios, { AxiosResponse } from 'axios';

export class API {
    getConfig(): Promise<AxiosResponse<ConfigResponse>> {
        return axios.get<ConfigResponse>(`${process.env.VUE_APP_BACKEND_URL}/api/config`);
    }
}

export interface ConfigResponse {
    supportedCharacters: string[];
    maxMessageLenInBytes: number;
}
