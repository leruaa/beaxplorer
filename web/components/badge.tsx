import cx from 'classnames';

type Props = { className?: string, children: string | number | JSX.Element | JSX.Element[] };

export default ({ className, children }: Props) => {

    return (
        <span className={cx(className, "rounded", "font-semibold", "px-1")}>{children}</span>
    )
}