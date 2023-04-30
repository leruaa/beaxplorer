import cx from 'classnames';

type Props = { className?: string, color?: string, children: string | number | JSX.Element | JSX.Element[] };

export default ({ className, color, children }: Props) => {
    const colorVariants = {
        blue: 'bg-blue-50 text-blue-500',
        green: 'bg-green-50 text-green-500',
        amber: 'bg-amber-50 text-amber-500',
        slate: 'bg-slate-50 text-slate-500',
    }
    return (
        <span className={cx(className, colorVariants[color], "rounded", "font-semibold", "px-1")}>{children}</span>
    )
}