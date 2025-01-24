import * as React from 'react';
import Tabs from '@mui/material/Tabs';
import Tab from '@mui/material/Tab';
import Box from '@mui/material/Box';
import ApiResponse from '../models/api_response';
import ApiList from '../models/api_list';
import CreateList from './create_list';
import Tasks from './tasks'

interface MainMenuState {
    selectedTab: number,
    lists: ApiList[];
}


export default class MainMenu extends React.Component<{}, MainMenuState> {

    private tasks: React.RefObject<Tasks>;

    constructor() {
        super({});
        this.state = {
            selectedTab: 0,
            lists: [],
        };
        this.tasks = React.createRef();
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
                    this.setState({ lists: data.data });
                }
            });
    }

    getSelectedListId(tabNumber: number): number | null {
        if (this.state.lists.length > tabNumber) {
            const list = this.state.lists[tabNumber];
            console.log(list);
            if (list != null && list.id != null){
                console.log(`Selected list id: ${list.id}`);
                return list.id;
            }
        }
        return null;
    }

    handleChange = (_event: React.SyntheticEvent, newValue: number) => {
        console.log(`Change tab to ${newValue}`);
        this.setState({ selectedTab: newValue });
        const selectedListId = this.getSelectedListId(newValue);
        if(selectedListId != null){
            console.log(`Update list ${selectedListId}`);
            this.tasks.current?.updateTasks(selectedListId);
        }
        console.log(this.state);
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
                            value={this.state.selectedTab}
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
                <Tasks
                    ref={this.tasks}
                    listId={this.getSelectedListId(this.state.selectedTab)} />
            </Box>
        );
    }
}
