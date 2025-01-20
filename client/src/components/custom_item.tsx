import * as React from 'react';
import ApiTask from '../models/api_task';
import ListItem from '@mui/material/ListItem';
import ListItemButton from '@mui/material/ListItemButton';
import ListItemText from '@mui/material/ListItemText';

interface CustomTaskState {
    task: ApiTask,
}

interface CustomItemProps {
    task: ApiTask,
}

export default class CustomItem extends React.Component<CustomItemProps, CustomTaskState> {

    constructor(props: CustomItemProps) {
        super(props);
        this.state = {
            task: this.props.task,
        }
    }
    render() {
        return (
            <>
                <ListItem disablePadding>
                    <ListItemButton>
                        <ListItemText primary={this.state.task.name}/>
                    </ListItemButton>
                </ListItem>
            </>
        );
    }
}
