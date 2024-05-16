import { useContext } from 'react';
import { UserContext } from './context/user';
import Login from './Login';
import Chat from './Chat';

function App() {
    const userContext = useContext(UserContext);
    return userContext?.isLoggedIn ? <Chat /> : <Login />;
}

export default App;
