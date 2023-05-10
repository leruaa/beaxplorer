import { MouseEventHandler, ReactNode } from "react"
import cx from "classnames";

type RootProps = { children?: ReactNode }

export const Root = ({ children }: RootProps) => {
  return <table className="w-full table-auto">{children}</table>
}

type HeaderProps = {
  key?: string,
  colSpan?: number,
  canSort?: boolean,
  isSorted?: boolean,
  children?: ReactNode,
  onClick?: MouseEventHandler<HTMLTableHeaderCellElement>
}

export const Header = ({ key, colSpan, canSort, isSorted, children, onClick }: HeaderProps) => {
  return <th
    key={key}
    colSpan={colSpan}
    className={cx("text-xs text-right px-1.5 py-2 text-gray-600 uppercase bg-gray-100", { "cursor-pointer": canSort }, { "text-black": isSorted })}
    onClick={onClick}
  >
    {children}
  </th>
}

type CellProps = {
  key?: string,
  isSorted?: boolean,
  children?: ReactNode
}

export const Cell = ({ key, isSorted, children }: CellProps) => {
  return <td key={key} className={cx("text-right tabular-nums py-1.5 pr-4 border-b border-gray-200", { "bg-gray-50": isSorted })}>
    {children}
  </td>
}

