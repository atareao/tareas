import * as React from 'react';
import Box from '@mui/material/Box';
import List from '@mui/material/List';
import ListItem from '@mui/material/ListItem';
import ApiResponse from '../models/api_response';
import ApiTask from '../models/api_task';
import CreateTask from './create_task';
import CustomTask from './custom_task';

interface TasksState {
    listId: number | null,
    tasks: ApiTask[],
}

interface TasksProps {
    listId: number | null,
}

const replaceContent = (fromTasks: ApiTask[], toTasks: ApiTask[]): ApiTask[] => {
    fromTasks.length = 0;
    fromTasks.push(...toTasks);
    return fromTasks;
}

export default class Tasks extends React.Component<TasksProps, TasksState> {

    private createTask: React.RefObject<CreateTask>;


    constructor(props: TasksProps) {
        super(props);
        console.log(`props: ${props.listId}`);
        this.createTask = React.createRef();
        this.state = {
            listId: props.listId,
            tasks: [],
        }
    }

    useEffect = () => {
        console.log(`${this.state} has changed`);
    }

    setSelectedList = async (listId: number) => {
        await this.updateTasks(listId);
    };

    updateTasks = async (listId: number) => {
        console.log("======================");
        console.log("Update lists");
        console.log(`/api/v1/tasks/${listId}`);
        this.createTask.current?.setListId(listId);
        await fetch(`/api/v1/tasks/${listId}`)
            .then(async (res) => {
                console.log(`Response: ${res.status}`);
                console.log(`Response: ${res}`);
                return await res.json();
            })
            .then((data: ApiResponse<ApiTask[]>) => {
                console.log(data.data);
                if(data.data != null){
                    for(const item of data.data){
                        console.log(`Item: ${item.name}`);
                    }
                    if (data.status === 200 && data.data != null) {
                        console.log(this.state.tasks);
                        this.setState({
                            listId: listId,
                            tasks: replaceContent(this.state.tasks, data.data),
                        });
                        console.log(this.state.tasks);
                        console.log(`Tasks for ${listId}`);
                        for(const task of data.data){
                            console.log(`Task: ${task.id} - ${task.name}`);
                        }
                    }else{
                        this.setState({ tasks: [] });
                    }
                }
            });
    }
    componentDidMount = async () => {
        console.log(`Component did mount ${this.state.listId}`);
        return true;
    }

    componentDidUpdate = async (props: TasksProps) => {
        console.log(`Component did update ${props.listId} - ${this.state.listId}`);
        return true;
    }
    shouldComponentUpdate = (props: any) => {
        console.log(`shouldComponentUpdate ${props}`);
        return true;
    }
    forceRender = () => {
        console.log(`Force render ${this.state.listId}`);
        if(this.state.listId != null){
            this.setSelectedList(this.state.listId);
        }
    }

    render = () => {
        console.log(`Render list ${this.state.listId}`);
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
                            {this.state.tasks.map((task: ApiTask) => {
                                console.log(`Task: ${task.id} - ${task.name}`);
                                if(task.id != null && task.name != null && task.done != null){
                                    return (
                                        <CustomTask
                                            key={task.id}
                                            task={task}
                                        />
                                    )
                                }
                            })}
                            <ListItem>
                                <CreateTask
                                    ref={this.createTask}
                                    listId={this.state.listId} onCallback={() => {
                                        console.log("Done")
                                        this.forceRender();
                                    }} />
                            </ListItem>
                        </List>
                    </Box>
                </Box>
            </Box>
        );
    }
}
