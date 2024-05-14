export type GrailProps = {
    header: JSX.Element;
    children: JSX.Element | JSX.Element[];
    footer: JSX.Element;
};

const Grail = ({ header, children, footer }: GrailProps) => {
    return (
        <div className="grid grid-rows-[50px,auto,50px] w-[100dvw] h-[100dvh] p-4 items-center">
            <div className="flex justify-center items-center">{header}</div>
            <div className="flex h-full">{children}</div>
            <div className="flex justify-center items-center">{footer}</div>
        </div>
    );
};

export default Grail;
