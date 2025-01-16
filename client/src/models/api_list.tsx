export default class ApiList {
    id?: number;
    name?: string;
    position?: number;
    created_at?: Date;
    updated_at?: Date;

    constructor(name: string){
        this.name = name;
    }
}

