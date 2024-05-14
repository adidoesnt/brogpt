import React, { useEffect } from 'react';
import { Role } from '../constants/role';

export type MessageProps = {
    uid: string;
    message: string;
    from: Role;
};

export const Message = ({ message, from, uid }: MessageProps) => {
    return from === Role.USER ? (
        <div
            id={`message-${uid}`}
            key={uid}
            className="flex w-full justify-end rounded-md"
        >
            <p className="flex p-2 bg-blue-400 rounded-md">{message}</p>
        </div>
    ) : (
        <div
            id={`message-${uid}`}
            key={uid}
            className="flex p-2 w-full bg-green-400 justify-start rounded-md"
        >
            <p>{message}</p>
        </div>
    );
};

type MessagesProps = {
    messages: MessageProps[];
};

const Messages = React.forwardRef<HTMLDivElement, MessagesProps>(
    ({ messages }, ref) => {
        const messagesEndRef = ref as React.MutableRefObject<HTMLDivElement>;

        useEffect(() => {
            if (messagesEndRef?.current) {
                messagesEndRef?.current.scrollIntoView({ behavior: 'smooth' });
            }
        }, [messages.length, messagesEndRef]);

        return (
            <div
                id="messages"
                className="flex flex-col justify-end flex-grow relative w-full h-fit p-4 gap-4"
                ref={ref}
            >
                {messages.map((message) => (
                    <Message key={message.uid} {...message} />
                ))}
            </div>
        );
    }
);

export default Messages;
