import * as React from 'react';
import ListItem from '@mui/material/ListItem';
import ListItemButton from '@mui/material/ListItemButton';
import ListItemText from '@mui/material/ListItemText';
import CheckBox from '@mui/material/Checkbox';

interface CustomTaskProps {
    id: number,
    name: string,
    done: boolean,
}

export default class CustomTask extends React.Component<CustomTaskProps, CustomTaskProps> {

    constructor(props: CustomTaskProps) {
        super(props);
        this.state = {
            id: props.id,
            name: props.name,
            done: props.done,
        }
        console.log(`Constructor for task: ${this.state.name}`);
        this.setState({
            id: this.props.id,
            name: this.props.name,
            done: this.props.done,
        });
    }

    render = () => {
        console.log(`Rendering task: ${this.state.name}`)
        const checkBox = this.state.done ? <CheckBox checked /> : <CheckBox />;
        return (
            <>
                <ListItem disablePadding>
                    <ListItemButton>
                        {checkBox}
                        <ListItemText primary={this.state.name}/>
                    </ListItemButton>
                </ListItem>
            </>
        );
    }
}
