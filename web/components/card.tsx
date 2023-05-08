import { ClockCountdown, Icon, IconContext } from '@phosphor-icons/react';
import cx from 'classnames';
import { ReactNode } from 'react';

enum Kind {
  Highlight,
  Basic
}

type ComonProps = {
  className?: string,
  contentClassName?: string,
  title: string,
  icon?: JSX.Element,
  children: ReactNode
};


export const HighlightCard = (props: ComonProps) => {
  return <Card {...props} kind={Kind.Highlight} />
}

export const BasicCard = (props: ComonProps) => {
  return <Card {...props} kind={Kind.Basic} />
}

type BaseProps = {
  kind: Kind,
};

const Card = ({ className, contentClassName, title, icon, kind, children }: ComonProps & BaseProps) => {

  return (
    <div className={cx(className, "relative flex flex-col gap-1 px-2 rounded text-white overflow-hidden")}>
      {icon &&
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
      }
      <h3 className={cx({ "opacity-70": kind == Kind.Highlight, "opacity-50": kind != Kind.Highlight }, "uppercase font-bold")}>{title}</h3>
      <div className={cx(contentClassName, "opacity-90")}>
        {children}
      </div>
    </div>
  )
}
