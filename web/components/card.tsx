import { ClockCountdown, Icon, IconContext } from '@phosphor-icons/react';
import cx from 'classnames';

type RootProps = { className?: string, titleClassName?: string, title: string, icon: JSX.Element, children: JSX.Element | JSX.Element[] };

export default ({ className, titleClassName, title, icon, children }: RootProps) => {

  return (
    <div className={cx(className, "relative flex flex-col gap-1 px-2 h-24 rounded text-white overflow-hidden")}>
      <div className="absolute -right-6 -top -bottom opacity-30">
        <IconContext.Provider
          value={{
            size: "6em",
            weight: "regular"
          }}
        >
          {icon}
        </IconContext.Provider>
      </div>
      <h3 className={cx(titleClassName, "uppercase font-bold")}>{title}</h3>
      <div className="opacity-90">
        {children}
      </div>
    </div>
  )
}
