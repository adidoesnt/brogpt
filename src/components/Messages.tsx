import { Role } from "../constants/role";

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

const Messages = ({ messages }: MessagesProps) => {
    return (
        <div
            id="messages"
            className="flex flex-col w-full py-4 justify-end gap-4 overflow-y-auto"
        >
            {messages.map((message) => (
                <Message {...message} />
            ))}
        </div>
    );
};

export default Messages;
