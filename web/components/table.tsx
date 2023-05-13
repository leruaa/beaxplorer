import { MouseEventHandler, ReactNode, forwardRef } from "react"
import cx from "classnames";

type RootProps = { children?: ReactNode }

export const Root = ({ children }: RootProps) => {
  return <table className="w-full table-auto">{children}</table>
}

enum Align {
  Left = "text-left",
  Center = "text-center",
  Right = "text-right",
}

type HeaderProps = {
  className?: string,
  align?: Align,
  colSpan?: number,
  canSort?: boolean,
  isSorted?: boolean,
  children?: ReactNode,
  onClick?: MouseEventHandler<HTMLTableHeaderCellElement>
}

export const Header = ({ className, align = Align.Left, colSpan, canSort, isSorted, children, onClick }: HeaderProps) => {
  return <th
    colSpan={colSpan}
    className={cx(className, align, "text-xs p-2 text-gray-600 whitespace-nowrap uppercase bg-gray-100", { "cursor-pointer": canSort }, { "text-black": isSorted })}
    onClick={onClick}
  >
    {children}
  </th>
}

export const RightAlignedHeader = ({ className, colSpan, canSort, isSorted, children, onClick }: HeaderProps) => {
  return <Header className={className} colSpan={colSpan} canSort={canSort} isSorted={isSorted} onClick={onClick} align={Align.Right}>{children}</Header>
}

type CellProps = {
  className?: string,
  align?: Align,
  isSorted?: boolean,
  children?: ReactNode
}

export const Cell = ({ className, align = Align.Left, isSorted, children }: CellProps) => {
  return <td className={cx(className, align, "tabular-nums py-1.5 px-2 border-b border-gray-200", { "bg-gray-50": isSorted })}>
    {children}
  </td>
};

export const RightAlignedCell = ({ className, isSorted, children }: CellProps) => {
  return <Cell className={className} isSorted={isSorted} align={Align.Right}>{children}</Cell>
};


