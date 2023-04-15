import Trim from "../components/trim";

type RootProps = { value: string };

export default ({ value }: RootProps) => {

    return (
        <Trim value={value} className="font-mono" regEx={/^(.{6}).*(.{4})$/g} groups={"$1" + String.fromCharCode(8230) /* &hellip; */ + "$2"} />
    )
}