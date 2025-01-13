import Container from '@mui/material/Container';
import Typography from '@mui/material/Typography';
import Box from '@mui/material/Box';
import Link from '@mui/material/Link';
import BasicTabs from './components/BasicTabs';
import CustomTabs from './components/CustomTabs';
import List from './models/list';

function Copyright() {
    return (
        <Typography
            variant="body2"
            align="center"
            sx={{
                color: 'text.secondary',
            }}
        >
            {'Copyright © '}
            <Link color="inherit" href="https://mui.com/">
                Your Website
            </Link>{' '}
            {new Date().getFullYear()}.
        </Typography>
    );
}

export default function App() {
    const lists: List[] = [];
    lists.push(new List('List 1'));
    lists.push(new List('List 2'));
    lists.push(new List('List 3'));
    lists.push(new List('List 4'));
    return (
        <Container>
            <CustomTabs lists={lists}/>
            <BasicTabs />
            <Box sx={{ my: 4 }}>
                <Copyright />
            </Box>
        </Container>
    );
}
