import * as React from 'react';
import Box from '@mui/material/Box';
import List from '@mui/material/List';
import TaskItem from './TaskItem';
import Task from '../models/task';

interface TabPanelProps {
    children?: React.ReactNode;
    index: number;
    value: number;
}

export default function CustomTabPanel(props: TabPanelProps) {
    const { children, value, index, ...other } = props;

    return (
        <div
            style={{ 
                display: 'flex',
                alignItems: 'center',
                justifyContent: 'center',
            }}
            role="tabpanel"
            hidden={value !== index}
            id={`simple-tabpanel-${index}`}
            aria-labelledby={`simple-tab-${index}`}
            {...other}
        >
            <Box>
                <nav aria-label="main mailbox folders">
                    <List>
                        <TaskItem task={new Task('Pan')}/>
                        <TaskItem task={new Task('Huevos')}/>
                        <TaskItem task={new Task('Leche')}/>
                    </List>
                </nav>
            </Box>
        </div>
    );
}

