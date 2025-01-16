import ApiList from './api_list';

export default class Response {
    status?: number;
    message?: string;
    data?: ApiList | ApiList[] | null;
}
