import * as React from 'react';
import Box from '@mui/material/Box';
import List from '@mui/material/List';
import ListItem from '@mui/material/ListItem';
import ListItemButton from '@mui/material/ListItemButton';
import ListItemText from '@mui/material/ListItemText';
import AddCircleOutlineIcon from '@mui/icons-material/AddCircleOutline';
import ApiResponse from '../models/api_response';
import ApiTask from '../models/api_task';

interface TasksProps {
    listId: number,
}

export default class Tasks extends React.Component<TasksProps, any> {

    state = {
        listId: 0,
        tasks: [],
    }

    constructor(props: TasksProps) {
        super(props);
        console.log(`props: ${props.listId}`);
        this.setState({ listId: props.listId}, this.updateList)
        this.updateList();
    }

    useEffect() {
        console.log(`${this.state} has changed`);
    }

    updateTasksList(listId: number) {
        this.setState({ listId: listId}, this.updateList)
    }

    updateList() {
        console.log("Update lists");
        fetch(`/api/v1/tasks/${this.state.listId}`)
            .then((res) => {
                console.log(`Response: ${res.status}`);
                console.log(`Response: ${res}`);
                return res.json();
            })
            .then((data: ApiResponse<ApiTask>) => {
                console.log(data);
                if (data.status === 200) {
                    console.log(data.data);
                    this.setState({ tasks: data.data });
                }
            });
    }

    render() {
        return (
            <Box sx={{ width: '100%', maxWidth: 360, bgcolor: 'background.paper' }}>
                    <List>
                        <ListItem disablePadding>
                            <ListItemButton>
                                <ListItemText primary="Trash" />
                            </ListItemButton>
                        </ListItem>
                        <ListItem disablePadding>
                            <ListItemButton component="a" href="#simple-list">
                                <ListItemText primary="Spam" />
                            </ListItemButton>
                        </ListItem>
                        <ListItem>
                            <ListItemButton>
                                <AddCircleOutlineIcon />
                            </ListItemButton>
                        </ListItem>
                    </List>
            </Box>
        );
    }
}
