import cx from 'classnames';

type Props = { className?: string, children: JSX.Element | JSX.Element[] };

export default ({ className, children }: Props) => {
    return (
        <span className={cx(className, "text-sm", "rounded-sm", "font-semibold", "px-0.5")}>{children}</span>
    )
}