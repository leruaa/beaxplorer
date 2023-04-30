import { DateTime, Duration } from "luxon";

type Props = { timestamp: number };

export default ({ timestamp }: Props) => {
    const dateTime = DateTime.fromSeconds(timestamp);
    const formatted = dateTime.toLocaleString(DateTime.DATETIME_SHORT_WITH_SECONDS);
    let duration = dateTime.diffNow(["days", "hours", "minutes", "seconds"]).negate();
    let obj = {};

    if (duration.days > 0) obj = { days: duration.days };
    else if (duration.hours > 0) obj = { hours: duration.hours };
    else if (duration.minutes > 0) obj = { minutes: duration.minutes };
    else obj = { seconds: duration.seconds };

    const relative = Duration.fromObject(obj, { locale: "en-US" }).toHuman();

    return (
        <span title={formatted}>{relative}</span>
    )
}