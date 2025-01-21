import * as React from 'react';
import Box from '@mui/material/Box';
import List from '@mui/material/List';
import ListItem from '@mui/material/ListItem';
import ApiResponse from '../models/api_response';
import ApiTask from '../models/api_task';
import CreateTask from './create_task';
import CustomItem from './custom_item';

interface TasksState {
    tasks: ApiTask[],
}

interface TasksProps {
    listId: number | null,
}

export default class Tasks extends React.Component<TasksProps, TasksState> {

    private createTask: React.RefObject<CreateTask>;

    private listId: number | null;
    state = {
        tasks: [],
    }

    constructor(props: TasksProps) {
        super(props);
        console.log(`props: ${props.listId}`);
        this.listId = props.listId;
        this.createTask = React.createRef();
    }

    useEffect() {
        console.log(`${this.state} has changed`);
    }

    updateList(listId: number) {
        console.log("Update lists");
        console.log(`/api/v1/tasks/${listId}`);
        fetch(`/api/v1/tasks/${listId}`)
            .then((res) => {
                console.log(`Response: ${res.status}`);
                console.log(`Response: ${res}`);
                return res.json();
            })
            .then((data: ApiResponse<ApiTask[]>) => {
                console.log(data);
                if (data.status === 200 && data.data != null) {
                    this.setState({ tasks: data.data });
                }else{
                    this.setState({ tasks: [] });
                }
                console.log(this.state.tasks);
            });
    }

    render() {
        const items = this.state.tasks.map((task: ApiTask) => {
            return <CustomItem task={task} />
        });
        return (
            <Box
                display="flex"
                justifyContent="center"
                alignItems="center"
            >
                <Box
                    sx={{
                        maxWidth: { xs: 320, sm: 480 },
                    }}
                >
                    <Box sx={{ width: '100%', maxWidth: 360, bgcolor: 'background.paper' }}>
                        <List>
                            {items}
                            <ListItem>
                                <CreateTask
                                    ref={this.createTask}
                                    listId={this.listId} onCallback={() => {
                                        console.log("Done")
                                    }} />
                            </ListItem>
                        </List>
                    </Box>
                </Box>
            </Box>
        );
    }
}
