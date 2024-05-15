import { useContext } from 'react';
import { UserContext } from './context/user';
import Login from './Login';
import Chat from './Chat';

function App() {
    const userContext = useContext(UserContext);
    return userContext?.isLoggedIn ? <Login /> : <Chat />;
}

export default App;
