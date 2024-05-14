export type InputProps = {
    value: string;
    onChange: (value: string) => void;
    onSubmit: () => void;
};

const Input = ({ value, onChange, onSubmit }: InputProps) => {
    const handleKeyDown = (event: React.KeyboardEvent) => {
        if (event.key === "Enter") {
            onSubmit();
        }
    };

    return (
        <div className="flex gap-2 w-full">
            <input
                type="text"
                value={value}
                onChange={(e) => onChange(e.target.value)}
                onKeyDown={handleKeyDown}
                className="border border-gray-300 p-2 w-full rounded-md"
            />
            <button
                onClick={onSubmit}
                className="bg-blue-500 text-white p-3 rounded-md"
            >
                Send
            </button>
        </div>
    );
};

export default Input;
