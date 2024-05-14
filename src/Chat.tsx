import { useCallback, useState } from "react";
import Input from "./components/Input";
import Grail from "./components/Grail";

function Chat() {
    const [value, setValue] = useState("");
    const onSubmit = useCallback(() => {
        console.log(value);
        setValue("");
    }, [value]);

    const Header = <h1 className="text-3xl font-bold">BroGPT</h1>;
    const Footer = (
        <Input value={value} onChange={setValue} onSubmit={onSubmit} />
    );

    return (
        <Grail header={Header} footer={Footer}>
            <div className="flex w-full h-full items-center justify-center">
                Chat
            </div>
        </Grail>
    );
}

export default Chat;
