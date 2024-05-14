import { useCallback, useRef, useState } from 'react';
import { v4 } from 'uuid';
import Input from './components/Input';
import Grail from './components/Grail';
import Messages, { MessageProps } from './components/Messages';
import { Role } from './constants/role';
import { getResponse } from './utils/llama';

function Chat() {
    const messagesEndRef = useRef<HTMLDivElement>(null);
    const [value, setValue] = useState('');
    const [messages, setMessages] = useState<Array<MessageProps>>([]);
    const onSubmit = useCallback(async () => {
        if (value.trim() === '') return;
        setMessages((prev) => [
            ...prev,
            {
                from: Role.USER,
                message: value,
                uid: v4()
            }
        ]);
        const savedValue = value;
        setValue('');
        const reply = await getResponse(savedValue);
        setMessages((prev) => [
            ...prev,
            {
                from: Role.SYSTEM,
                message: reply,
                uid: v4()
            }
        ]);
    }, [value]);

    const Header = <h1 className="text-3xl font-bold">BroGPT</h1>;
    const Footer = (
        <Input value={value} onChange={setValue} onSubmit={onSubmit} />
    );

    return (
        <Grail header={Header} footer={Footer}>
            <Messages ref={messagesEndRef} messages={messages} />
            <div ref={messagesEndRef} />
        </Grail>
    );
}

export default Chat;
