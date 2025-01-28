import * as React from 'react';
import ListItem from '@mui/material/ListItem';
import ListItemButton from '@mui/material/ListItemButton';
import ListItemText from '@mui/material/ListItemText';
import ApiTask from '../models/api_task';
import ApiResponse from '../models/api_response';
import TaskAltIcon from '@mui/icons-material/TaskAlt';
import PanoramaFishEyeIcon from '@mui/icons-material/PanoramaFishEye';

interface CustomTaskProps {
    task: ApiTask,
}

export default class CustomTask extends React.Component<CustomTaskProps, CustomTaskProps> {

    constructor(props: CustomTaskProps) {
        super(props);
        this.state = {
            task: props.task,
        }
        console.log(`Constructor for task: ${this.props.task.name}`);
        this.setState({
            task: props.task,
        });
    }

    toggle = async () => {
        console.log(`Toggling task: ${this.state.task.name}`);
        const task = this.state.task;
        task.done = !task.done;
        await fetch("/api/v1/tasks", {
            method: 'PUT',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify(task),
        })
        .then(async (response) => await response.json())
        .then((data: ApiResponse<ApiTask>) => {
            console.log(data);
            if(data.status === 200 && data.data != null){
                console.log(data.data);
                console.log("Updating task");
                this.setState({task: data.data});
                console.log("Updated task");
            }
        });
    }

    render = () => {
        console.log(`Rendering task: ${this.state.task.name} - ${this.state.task.done}`);
        return (
            <>
                <ListItem disablePadding>
                    <ListItemButton onClick={this.toggle}>
                        {this.state.task.done ? <TaskAltIcon /> : <PanoramaFishEyeIcon />}
                        <ListItemText
                            sx ={{marginLeft: 2}}
                            primary={this.state.task.name}/>
                    </ListItemButton>
                </ListItem>
            </>
        );
    }
}
