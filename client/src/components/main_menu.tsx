
import * as React from 'react';
import Tabs from '@mui/material/Tabs';
import Tab from '@mui/material/Tab';
import Box from '@mui/material/Box';
import Button from '@mui/material/Button';

export default class MainMenu extends React.Component {

    state = {
        value: 0,
    }

    constructor(props: any) {
        super(props);
        this.state = { value: 0 };
    }

    handleChange = (event: React.SyntheticEvent, newValue: number) => {
        this.setState({ value: newValue });
    }

    getButtonFirstList() {
        console.log("Create List");
        return (
            <Box
                display="flex"
                justifyContent="center"
                alignItems="center"
            >
                <Button
                    onClick={() => { console.log("Create List") }}
                    variant="contained"
                    color="primary"
                    size="small"
                >
                    Create First List
                </Button>
            </Box>
        );
    }

    getTabs() {
        return (
            <Box
                display="flex"
                justifyContent="center"
                alignItems="center"
            >
                <Box
                    sx={{
                        maxWidth: { xs: 320, sm: 480 },
                        bgcolor: 'background.paper'
                    }}
                >
                    <Tabs
                        value={this.state.value}
                        onChange={this.handleChange}
                        variant="scrollable"
                        scrollButtons
                        allowScrollButtonsMobile
                        aria-label="scrollable force tabs example"
                    >
                        <Tab onClick={() => { console.log("Item One") }} label="Item One" />
                        <Tab label="Item Two" />
                        <Tab label="Item Three" />
                        <Tab label="Item Four" />
                        <Tab label="Item Five" />
                        <Tab label="Item Six" />
                        <Tab label="Item Seven" />
                    </Tabs>
                </Box>
            </Box>
        );

    }

    render() {
        if (true) {
            return this.getButtonFirstList();
        }
        return this.getTabs();
    }
}
