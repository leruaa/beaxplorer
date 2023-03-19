import Trim from "../components/trim";

export default ({ value, className = "" }) => {

    return (
        <Trim value={value} className={className} regEx={/^(.{8}).*(.{8})$/g} groups={"$1" + String.fromCharCode(8230) /* &hellip; */ + "$2"} />
    )
}