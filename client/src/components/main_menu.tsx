import * as React from 'react';
import Tabs from '@mui/material/Tabs';
import Tab from '@mui/material/Tab';
import Box from '@mui/material/Box';
import ApiResponse from '../models/api_response';
import ApiList from '../models/api_list';
import CreateList from './create_list';
import Tasks from './tasks'

export default class MainMenu extends React.Component {

    state = {
        listId: 0,
        lists: [],
    }

    constructor(props: any) {
        super(props);
        this.state = { listId: 0, lists: [] };
        this.updateLists();
    }


    updateLists = () => {
        console.log("Update lists");
        fetch('/api/v1/lists')
            .then((res) => {
                console.log(`Response: ${res.status}`);
                console.log(`Response: ${res}`);
                return res.json();
            })
            .then((data: ApiResponse<ApiList[]>) => {
                console.log(data);
                if (data.status === 200 && data.data != null) {
                    console.log(data.data);
                    this.setState({
                        lists: data.data
                    });
                }
            });
    }

    handleChange = (_event: React.SyntheticEvent, newValue: number) => {
        console.log(`Change tab to ${newValue}`);
        this.setState({ listId: newValue });
    }

    render() {
        console.log("Render tabs");
        const tabs = this.state.lists.map((list: ApiList) => {
            return <Tab key={list.id} label={list.name} />
        });
        return (
            <Box>
                <Box
                    display="flex"
                    justifyContent="center"
                    alignItems="center"
                >
                    <Box
                        sx={{
                            maxWidth: { xs: 320, sm: 480 },
                        }}
                    >
                        <Tabs
                            value={this.state.listId}
                            onChange={this.handleChange}
                            variant="scrollable"
                            scrollButtons
                            allowScrollButtonsMobile
                            aria-label="scrollable force tabs example"
                        >
                            {tabs}
                            <CreateList onCallback={() => {
                                console.log("Update lists");
                                this.updateLists();
                                console.log("Finish update lists");
                            }} />
                        </Tabs>
                    </Box>
                </Box>
                <Box
                    display="flex"
                    justifyContent="center"
                    alignItems="center"
                >
                    <Box
                        sx={{
                            maxWidth: { xs: 320, sm: 480 },
                        }}
                    >
                        <Tasks listId={this.state.listId}/>
                    </Box>
                </Box>
            </Box>
        );
    }
}
