import { Component } from 'react';
import ListItem from '@mui/material/ListItem';
import ListItemButton from '@mui/material/ListItemButton';
import ListItemIcon from '@mui/material/ListItemIcon';
import CheckBoxOutlineBlankOutlinedIcon from '@mui/icons-material/CheckBoxOutlineBlankOutlined';
import CheckBoxOutlinedIcon from '@mui/icons-material/CheckBoxOutlined';
import Task from '../models/task';

interface TaskItemProps {
    task: Task,
}

export default class TaskItem extends Component<TaskItemProps> {
    render() {
        const {task} = this.props;
        return (
            <ListItem disablePadding>
                <ListItemButton>
                    <ListItemIcon>
                    { task.done ? <CheckBoxOutlinedIcon />: <CheckBoxOutlineBlankOutlinedIcon />}
                    </ListItemIcon>
                    { task.done ? <s>{task.name}</s>: <span>{task.name}</span> }
                </ListItemButton>
            </ListItem>
        );
    }
}

