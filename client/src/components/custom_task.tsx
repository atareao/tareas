import * as React from 'react';
//import ListItem from '@mui/material/ListItem';
import Box from '@mui/material/Box';
import ListItemButton from '@mui/material/ListItemButton';
import ListItemText from '@mui/material/ListItemText';
import ApiTask from '../models/api_task';
import ApiResponse from '../models/api_response';
import TaskAltIcon from '@mui/icons-material/TaskAlt';
import CheckIcon from '@mui/icons-material/Check';
import PanoramaFishEyeIcon from '@mui/icons-material/PanoramaFishEye';
import DeleteForeverIcon from '@mui/icons-material/DeleteForever';
import Typography from '@mui/material/Typography';
import {
    LeadingActions,
    SwipeableListItem,
    SwipeAction,
    TrailingActions,
} from 'react-swipeable-list';
import 'react-swipeable-list/dist/styles.css';

/*
const leadingActions = () => (
    <LeadingActions>
        <SwipeAction
            onClick={() => console.info('swipe action triggered')}
        >
            <Box
                sx={{
                    display: 'flex',
                    alignItems: 'center',
                    flexDirection: 'column',
                }}>
                <Box>
                    <span className="icon">
                        <CheckIcon />
                    </span>
                </Box>
                <Box>
                    <Typography variant="caption" gutterBottom sx={{ display: 'block' }}>
                        Toggle
                    </Typography>
                </Box>
            </Box>
        </SwipeAction>
    </LeadingActions>
);
*/

const trailingActions = () => (
    <TrailingActions>
        <SwipeAction
            destructive={true}
            onClick={() => console.log('swipe action triggered')}
        >
            <Box
                sx={{
                    display: 'flex',
                    alignItems: 'center',
                    flexDirection: 'column',
                }}>
                <Box>
                    <span className="icon">
                        <DeleteForeverIcon />
                    </span>
                </Box>
                <Box>
                    <Typography variant="caption" gutterBottom sx={{ display: 'block' }}>
                        Delete
                    </Typography>
                </Box>
            </Box>
        </SwipeAction>
    </TrailingActions>
);

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
    leadingActions = () => (
        <LeadingActions>
            <SwipeAction
                onClick={() => {
                    console.log('swipe action triggered');
                    console.log(`this.state.task.done: ${this.state.task.done}`);
                }}
            >
                <Box
                    sx={{
                        display: 'flex',
                        alignItems: 'center',
                        flexDirection: 'column',
                    }}>
                    <Box>
                        <span className="icon">
                            <CheckIcon />
                        </span>
                    </Box>
                    <Box>
                        <Typography variant="caption" gutterBottom sx={{ display: 'block' }}>
                            Toggle
                        </Typography>
                    </Box>
                </Box>
            </SwipeAction>
        </LeadingActions>
    );

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
                if (data.status === 200 && data.data != null) {
                    console.log(data.data);
                    console.log("Updating task");
                    this.setState({ task: data.data });
                    console.log("Updated task");
                }
            });
    }

    render = () => {
        console.log(`Rendering task: ${this.state.task.name} - ${this.state.task.done}`);
        return (
            <>
                <SwipeableListItem
                    key={this.state.task.id}
                    leadingActions={this.leadingActions()}
                    trailingActions={trailingActions()}
                    onClick={() => console.log('list item clicked')}
                >
                    <ListItemButton onClick={this.toggle}>
                        {this.state.task.done ? <TaskAltIcon /> : <PanoramaFishEyeIcon />}
                        <ListItemText
                            sx={{ marginLeft: 2 }}
                            primary={this.state.task.name} />
                    </ListItemButton>
                </SwipeableListItem>
            </>
        );
    }
}
