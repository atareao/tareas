import * as React from 'react';
import ApiTask from '../models/api_task';
import ListItem from '@mui/material/ListItem';
import ListItemButton from '@mui/material/ListItemButton';
import ListItemText from '@mui/material/ListItemText';
import CheckBox from '@mui/material/Checkbox';

interface CustomTaskState {
    task: ApiTask,
}

interface CustomItemProps {
    task: ApiTask,
}

export default class CustomTask extends React.Component<CustomItemProps, CustomTaskState> {

    constructor(props: CustomItemProps) {
        super(props);
        this.state = {
            task: this.props.task,
        }
        console.log(`Constructor for task: ${this.state.task.name}`);
        this.setState({ task: this.props.task });
    }

    updateTask = (task: ApiTask) => {
        console.log(`Updating task: ${task}`);
        this.setState({ task: task });
    };

    render = () => {
        console.log(`Rendering task: ${this.state.task.name}`)
        const checkBox = this.state.task.done ? <CheckBox checked /> : <CheckBox />;
        return (
            <>
                <ListItem disablePadding>
                    <ListItemButton>
                        {checkBox}
                        <ListItemText primary={this.state.task.name}/>
                    </ListItemButton>
                </ListItem>
            </>
        );
    }
}
