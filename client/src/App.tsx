import Container from '@mui/material/Container';
import Typography from '@mui/material/Typography';
import Box from '@mui/material/Box';
import Link from '@mui/material/Link';
import MainMenu from './components/main_menu';
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
    return (
        <Container>
            <Box sx={{ my: 4 }}>
                <MainMenu />
            </Box>
            <Box sx={{ my: 4 }}>
                <Copyright />
            </Box>
        </Container>
    );
}
