import * as React from 'react';
import Button from '@mui/material/Button';
import TextField from '@mui/material/TextField';
import Dialog from '@mui/material/Dialog';
import DialogActions from '@mui/material/DialogActions';
import DialogContent from '@mui/material/DialogContent';
import DialogTitle from '@mui/material/DialogTitle';
import ApiResponse from '../models/api_response';
import AddCircleOutlineIcon from '@mui/icons-material/AddCircleOutline';


export default class CreateList extends React.Component {

    state = {
        open: false,
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
                            fetch('/api/v1/lists', {
                                method: 'POST',
                                headers: {
                                    'Content-Type': 'application/json',
                                },
                                body: JSON.stringify(formJson),
                            })
                            .then((data: ApiResponse) => {
                                console.log(data);
                                if(data.status === 200){
                                    console.log(data.data);
                                }
                            });
                            console.log(formJson);
                            const name = formJson.name;
                            console.log(name);
                            this.handleClose();
                        },
                    }}
                >
                    <DialogTitle>Create list</DialogTitle>
                    <DialogContent>
                        <TextField
                            autoFocus
                            required
                            margin="dense"
                            id="name"
                            name="name"
                            label="List name"
                            type="text"
                            fullWidth
                            variant="standard"
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
