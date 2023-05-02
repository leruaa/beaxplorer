import { DateTime } from "luxon";

type Props = { timestamp: number };

export default ({ timestamp }: Props) => {
    const dateTime = DateTime.fromSeconds(timestamp);
    const formatted = dateTime.setLocale('en-US').toLocaleString(DateTime.DATETIME_SHORT_WITH_SECONDS);

    return (
        <span>{formatted}</span>
    )
}