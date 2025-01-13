import * as React from 'react';
import ButtonGroup from '@mui/material/ButtonGroup';
import Button from '@mui/material/Button';
import List from '../models/list';

interface Props {
    lists: List[],
}

export default class CustomTabs extends React.Component {
    selected: number = 2;
    buttons: Button[];

    constructor(props: Props){
        super(props);
        this.state = {selected: 2};
        this.buttons = [];
        for (let i=0; i<props.lists.length; i++) {
            const list = props.lists[i];
            if(i === this.selected){
                this.buttons.push(
                    <Button 
                        onclick={() =>  {
                            this.setState({selected: i});
                        }}
                        variant="contained">
                        {list.name}
                    </Button>
                );
            }else{
                this.buttons.push(<Button variant="outlined">{list.name}</Button>);
            }
        }
    }
    render() {
        return (
            <ButtonGroup variant="contained" aria-label="Basic button group">
                {this.buttons}
            </ButtonGroup>
        );
    }
}


