import { DateTime } from "luxon";

type Props = { timestamp: number };

export default ({ timestamp }: Props) => {
    const dateTime = DateTime.fromSeconds(timestamp);
    const formatted = dateTime.toLocaleString(DateTime.DATETIME_SHORT_WITH_SECONDS);
    const relative = dateTime.toRelative({ unit: ["days", "hours", "minutes", "seconds"] });
    return (
        <span title={formatted}>{relative}</span>
    )
}