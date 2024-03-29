import { IconContext } from '@phosphor-icons/react';
import cx from 'classnames';
import { ReactNode } from 'react';
import { Accent } from '../hooks/accent';

enum Kind {
  Highlight,
  Basic
}

type ComonProps = {
  className?: string,
  accent: Accent,
  contentClassName?: string,
  title: string,
  icon?: JSX.Element,
  children: ReactNode
};


export const HighlightCard = (props: ComonProps) => {
  switch (props.accent) {
    case Accent.Green:
      return <Card {...props} className={cx(props.className, "from-green-400 to-green-500 text-white")} kind={Kind.Highlight} />
    case Accent.Yellow:
      return <Card {...props} className={cx(props.className, "from-yellow-400 to-yellow-500 text-white")} kind={Kind.Highlight} />
    case Accent.Sky:
      return <Card {...props} className={cx(props.className, "from-sky-400 to-sky-500 text-white")} kind={Kind.Highlight} />
    case Accent.Indigo:
      return <Card {...props} className={cx(props.className, "from-indigo-400 to-indigo-500 text-white")} kind={Kind.Highlight} />
    case Accent.Purple:
      return <Card {...props} className={cx(props.className, "from-purple-400 to-purple-500 text-white")} kind={Kind.Highlight} />
    default:
      return <Card {...props} kind={Kind.Highlight} />
  }
}

export const BasicCard = (props: ComonProps) => {
  switch (props.accent) {
    case Accent.Green:
      return <Card {...props} className={cx(props.className, "from-white to-green-50 text-green-700")} kind={Kind.Basic} />
    case Accent.Green:
      return <Card {...props} className={cx(props.className, "from-white to-yellow-50 text-yellow-700")} kind={Kind.Basic} />
    case Accent.Sky:
      return <Card {...props} className={cx(props.className, "from-white to-sky-50 text-sky-700")} kind={Kind.Basic} />
    case Accent.Indigo:
      return <Card {...props} className={cx(props.className, "from-white to-indigo-50 text-indigo-700")} kind={Kind.Basic} />
    case Accent.Purple:
      return <Card {...props} className={cx(props.className, "from-white to-purple-50 text-purple-700")} kind={Kind.Basic} />
    default:
      return <Card {...props} kind={Kind.Basic} />
  }
}

type BaseProps = {
  kind: Kind,
};

const Card = ({ className, contentClassName, title, icon, kind, children }: ComonProps & BaseProps) => {

  return (
    <div className={cx(className, "relative flex flex-col gap-1 px-2 rounded bg-gradient-to-b h-24 overflow-hidden")}>
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
