//import * as React from 'react';
import { createTheme } from '@mui/material/styles';
import { PageContainer } from '@toolpad/core/PageContainer';
import { AppProvider } from '@toolpad/core/AppProvider';
import { useDemoRouter } from '@toolpad/core/internal';
import Paper from '@mui/material/Paper';

const NAVIGATION = [
    { segment: '', title: 'Home' },
    { segment: 'orders', title: 'Orders' },
];

export default function App() {
    const router = useDemoRouter('/orders');
    const darkTheme = createTheme({
        palette: {
            mode: 'dark',
        },
    });

    return (
            <AppProvider navigation={NAVIGATION} router={router} theme={darkTheme}>
                <Paper sx={{ width: '100%' }}>
                    {/* preview-start */}
                    <PageContainer>Page content</PageContainer>
                    {/* preview-end */}
                </Paper>
            </AppProvider>
    );
}
