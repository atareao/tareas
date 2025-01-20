export default class Response<T> {
    status?: number;
    message?: string;
    data?: T | null;
}
