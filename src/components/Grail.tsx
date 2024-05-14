export type GrailProps = {
    header: JSX.Element;
    children: JSX.Element | JSX.Element[];
    footer: JSX.Element;
};

const Grail = ({ header, children, footer }: GrailProps) => {
    return (
        <div id="grail" className="grid grid-rows-[50px,1fr,50px] w-[100dvw] h-[100dvh] p-4 items-center">
            <div id="header" className="flex justify-center items-center">{header}</div>
            <div id="middle" className="flex flex-col h-full overflow-y-auto">{children}</div>
            <div id="footer" className="flex justify-center items-center">{footer}</div>
        </div>
    );
};

export default Grail;
