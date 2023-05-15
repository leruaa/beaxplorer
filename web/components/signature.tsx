import Trim from "../components/trim";

export default ({ value }: { value: string }) => {

    return (
        <Trim value={value} className="font-mono" regEx={/^(.{10}).*$/g} groups={"$1" + String.fromCharCode(8230) /* &hellip; */} />
    )
}