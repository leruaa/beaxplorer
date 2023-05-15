import { ReactElement, ReactNode } from "react"
import cx from 'classnames';
import { useAccent, Accent } from "../hooks/accent";

export default ({ title, titleClassName, contentClassName, children }: { title: string, titleClassName?: string, contentClassName?: string, children: ReactNode }) => {

    let accent = useAccent();

    return (
        <div className={cx("px-2")}>
            <h3 className={cx(titleClassName, { "text-sky-800": accent == Accent.Sky, "text-indigo-800": accent == Accent.Indigo }, " opacity-50 uppercase font-bold")}>{title}</h3>
            <div className={cx(contentClassName, "text-gray-800")}>{children}</div>
        </div>
    )
}
