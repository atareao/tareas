import * as React from 'react';
import ListItem from '@mui/material/ListItem';
import ListItemButton from '@mui/material/ListItemButton';
import ListItemText from '@mui/material/ListItemText';
import CheckBox from '@mui/material/Checkbox';
import ApiTask from '../models/api_task';

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
