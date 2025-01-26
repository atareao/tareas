import * as React from 'react';
import Button from '@mui/material/Button';
import TextField from '@mui/material/TextField';
import Dialog from '@mui/material/Dialog';
import DialogActions from '@mui/material/DialogActions';
import DialogContent from '@mui/material/DialogContent';
import DialogTitle from '@mui/material/DialogTitle';
import ApiResponse from '../models/api_response';
import AddCircleOutlineIcon from '@mui/icons-material/AddCircleOutline';
import ApiList from '../models/api_list';

interface CreateTaskProps {
    listId: number | null;
    onCallback: Function;
}
interface CreateTaskState {
    listId: number | null;
    open: boolean;
}

export default class CreateTask extends React.Component<CreateTaskProps, CreateTaskState> {

    constructor(props: CreateTaskProps) {
        super(props);
        console.log(`props: ${props.onCallback}`);
        this. state = {
            listId: this.props.listId,
            open: false,
        }
    }

    setListId = (listId: number) => {
        this.setState({ listId: listId });
    }

    handleOpen = () => {
        this.setState({ open: true });
    }

    handleClose = () => {
        this.setState({ open: false });
    }

    render() {
        return (
            <React.Fragment>
                <Button onClick={this.handleOpen}>
                    <AddCircleOutlineIcon />
                </Button>
                <Dialog
                    open={this.state.open}
                    onClose={this.handleClose}
                    PaperProps={{
                        component: 'form',
                        onSubmit: (event: React.FormEvent<HTMLFormElement>) => {
                            event.preventDefault();
                            const formData = new FormData(event.currentTarget);
                            const formJson = Object.fromEntries((formData as any).entries());
                            console.log(`/api/v1/tasks/${this.state.listId}`);
                            fetch(`/api/v1/tasks/${this.state.listId}`, {
                                method: 'POST',
                                headers: {
                                    'Content-Type': 'application/json',
                                },
                                body: JSON.stringify(formJson),
                            })
                            .then(response => response.json())
                            .then((data: ApiResponse<ApiList>) => {
                                console.log(data);
                                if(data.status === 201){
                                    if (data.data !== null) {
                                        console.log(data.data);
                                    }
                                    console.log("actualizar");
                                    this.props.onCallback();
                                    console.log("actualizado");
                                }
                            });
                            console.log(formJson);
                            const name = formJson.name;
                            console.log(name);
                            this.handleClose();
                        },
                    }}
                >
                    <DialogTitle>Create task</DialogTitle>
                    <DialogContent>
                        <TextField
                            autoFocus
                            required
                            margin="dense"
                            id="name"
                            name="name"
                            label="Task name"
                            type="text"
                            fullWidth
                            variant="standard"
                        />
                        <TextField
                            margin="dense"
                            id="list_id"
                            name="list_id"
                            label="list_id"
                            type="number"
                            value={this.state.listId}
                            fullWidth
                            variant="standard"
                            sx={{ display: 'none' }}
                        />
                    </DialogContent>
                    <DialogActions>
                        <Button onClick={this.handleClose}>Cancel</Button>
                        <Button type="submit">Create</Button>
                    </DialogActions>
                </Dialog>
            </React.Fragment>
        );
    }
}
