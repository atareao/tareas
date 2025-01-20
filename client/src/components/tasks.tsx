import * as React from 'react';
import Box from '@mui/material/Box';
import List from '@mui/material/List';
import ListItem from '@mui/material/ListItem';
import ApiResponse from '../models/api_response';
import ApiTask from '../models/api_task';
import CreateTask from './create_task';
import CustomItem from './custom_item';

interface TasksState {
    listId: number,
    tasks: ApiTask[],
}

interface TasksProps {
    listId: number,
}

export default class Tasks extends React.Component<TasksProps, TasksState> {

    private createTask: React.RefObject<CreateTask>;

    state = {
        listId: 0,
        tasks: [],
    }

    constructor(props: TasksProps) {
        super(props);
        console.log(`props: ${props.listId}`);
        this.setState({ listId: props.listId}, this.updateList)
        this.createTask = React.createRef();
        this.updateList();
    }

    useEffect() {
        console.log(`${this.state} has changed`);
    }

    updateTasksList(listId: number) {
        console.log(`Update tasks list ${listId}`);
        this.setState({ listId: listId}, this.updateList)
        this.createTask.current?.setState({ listId: listId });
    }

    updateList() {
        console.log("Update lists");
        fetch(`/api/v1/tasks/${this.state.listId}`)
            .then((res) => {
                console.log(`Response: ${res.status}`);
                console.log(`Response: ${res}`);
                return res.json();
            })
            .then((data: ApiResponse<ApiTask[]>) => {
                console.log(data);
                if (data.status === 200 && data.data != null) {
                    console.log(data.data);
                    this.setState({ tasks: data.data });
                }
            });
    }

    render() {
        const items = this.state.tasks.map((task: ApiTask) => {
            return <CustomItem task={task} />
        });
        return (
            <Box sx={{ width: '100%', maxWidth: 360, bgcolor: 'background.paper' }}>
                    <List>
                        {items}
                        <ListItem>
                            <CreateTask
                                ref={this.createTask}
                                listId={this.state.listId} onCallback={() => {
                                console.log("Done")
                            }} />
                        </ListItem>
                    </List>
            </Box>
        );
    }
}
