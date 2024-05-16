import { useContext } from 'react';
import { UserContext } from './context/user';

function Login() {
    const userContext = useContext(UserContext);
    const { setIsLoggedIn } = userContext!;

    const onSubmit = () => {
        setIsLoggedIn(true);
    };

    return (
        <div>
            <button onClick={onSubmit}>Log in</button>
        </div>
    );
}

export default Login;
