import { Component, useState } from 'react';
import Button from '@mui/material/Button';

interface Props {
    label: string,
    selected: boolean,
    onclick: Function,
}

export default class CustomButton extends Component<Props> {
    label: string;
    index: number;
    selected: boolean;
    onclick: Function;


    constructor(props: any) {
        super(props);
        this.label = props.label;
        this.index = props.index;
        this.selected = props.selected;
        this.onclick = props.onclick;
    }

    render() {
        return (
            <Button onclick={this.onclick}
                    variant = {this.selected?'contained':'outlined'} >
                    { this.label }
                    </Button >
        );

    }
}
