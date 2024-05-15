import axios from 'axios';

const { VITE_SERVER_URL: baseURL = 'DUMMY_SERVER_URL' } = import.meta.env;

const llama = axios.create({
    baseURL,
    headers: {
        'Content-Type': 'application/json'
    }
});

export const getResponse = async (message: string) => {
    try {
        const response = await llama.post('/reply', {
            prompt: message
        });
        const { reply } = response.data;
        return reply;
    } catch {
        return 'Sorry, I am having trouble connecting to the server.';
    }
};

export default llama;
