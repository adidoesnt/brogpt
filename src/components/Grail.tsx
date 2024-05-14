export type GrailProps = {
    header: JSX.Element;
    children: JSX.Element | JSX.Element[];
    footer: JSX.Element;
};

const Grail = ({ header, children, footer }: GrailProps) => {
    return (
        <div id="grail" className="grid grid-rows-[auto,85%,auto] w-[100dvw] h-[100dvh] p-4 items-center">
            <div id="header" className="flex justify-center items-center">{header}</div>
            <div id="middle" className="flex h-full">{children}</div>
            <div id="footer" className="flex justify-center items-center">{footer}</div>
        </div>
    );
};

export default Grail;
