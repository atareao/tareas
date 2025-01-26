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
    }

    updateTask = (task: ApiTask) => {
        this.setState({ task: task });
    };

    render = () => {
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
