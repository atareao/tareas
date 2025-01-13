export default class Task {
    id?: number;
    list_id?: number;
    name?: string;
    position?: number;
    done: boolean = false;
    created_at?: Date;
    updated_at?: Date;

    constructor(name: string){
        this.name = name;
    }
}
