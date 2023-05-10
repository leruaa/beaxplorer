import cx from "classnames";
import { SortDirection, Table as TanstackTable, flexRender } from "@tanstack/react-table";
import { MouseEventHandler, ReactNode } from "react";
import * as Table from "../components/table";

interface DataTableProps {
  title?: string,
  table: TanstackTable<any>;
  updatable?: boolean
}

export default ({ title, table, updatable }: DataTableProps) => {

  const state = table.getState();

  return (
    <div className="border border-gray-200 rounded">
      <div className="flex p-2">
        <h3 className="grow">{title}</h3>
        <span>
          Show
          &nbsp;
          <select
            value={state.pagination.pageSize}
            onChange={e => {
              table.setPageSize(Number(e.target.value))
            }}
          >
            {[10, 20, 30, 40, 50].map(pageSize => (
              <option key={pageSize} value={pageSize}>
                {pageSize}
              </option>
            ))}
          </select>
          &nbsp;
          entries
        </span>
      </div>
      <Table.Root>
        <thead>
          {// Loop over the header rows
            table.getHeaderGroups().map(headerGroup => (
              // Apply the header row props
              <tr key={headerGroup.id}>
                {// Loop over the headers in each row
                  headerGroup.headers.map(header => (
                    // Apply the header cell props
                    <Table.Header
                      key={header.id}
                      className={(header.column.columnDef.meta as any)?.className}
                      colSpan={header.colSpan}
                      canSort={header.column.getCanSort()}
                      isSorted={!!header.column.getIsSorted()}
                      onClick={header.column.getToggleSortingHandler()}
                    >
                      {flexRender(header.column.columnDef.header, header.getContext())}
                      {header.column.getCanSort() && (
                        <span className="inline-block align-bottom w-4 h-4 px-1">
                          <SortIcon isSorted={header.column.getIsSorted()} />
                        </span>
                      )}
                    </Table.Header>
                  ))}
              </tr>
            ))}
        </thead>
        {/* Apply the table body props */}
        <tbody>
          {// Loop over the table rows
            table.getRowModel().rows.map(row => (
              <tr key={row.id} className={cx(isStalled(table, row.index) ? "text-gray-400" : "text-gray-800")}>
                {!updatable || table.options.data[row.index].isLoaded ?
                  row.getVisibleCells().map(cell => (
                    <Table.Cell className={(cell.column.columnDef.meta as any)?.className} key={cell.id} isSorted={!!cell.column.getIsSorted()}>
                      {flexRender(cell.column.columnDef.cell, cell.getContext())}
                    </Table.Cell>
                  ))
                  : (
                    <td className="p-1.5 border-b border-gray-200" colSpan={table.options.columns.length}>
                      <div className="flex">
                        <span className="bg-slate-50 w-full rounded">&nbsp;</span>
                      </div>
                    </td>
                  )
                }
              </tr>
            ))
          }
        </tbody>
      </Table.Root>

      <div className="flex p-2 justify-end">
        <PaginationButton onClick={() => table.setPageIndex(0)} disabled={!table.getCanPreviousPage()}>
          First
        </PaginationButton>
        &nbsp;
        <PaginationButton onClick={() => table.previousPage()} disabled={!table.getCanPreviousPage()}>
          {'<'}
        </PaginationButton>
        &nbsp;
        <span>
          <input
            className="w-20"
            type="number"
            value={table.getState().pagination.pageIndex + 1}
            onChange={e => {
              const page = e.target.value ? Number(e.target.value) - 1 : 0
              table.setPageIndex(page)
            }}
          />
          &nbsp;of {table.getPageCount()}
        </span>
        &nbsp;
        <PaginationButton onClick={() => table.nextPage()} disabled={!table.getCanNextPage()}>
          {'>'}
        </PaginationButton>
        &nbsp;
        <PaginationButton onClick={() => table.setPageIndex(table.getPageCount() - 1)} disabled={!table.getCanNextPage()}>
          Last
        </PaginationButton>
      </div>
    </div>
  )
}

type PaginationButtonProps = { disabled?: boolean, children: ReactNode, onClick?: MouseEventHandler<HTMLButtonElement> };

const PaginationButton = ({ disabled, children, onClick }: PaginationButtonProps) => {
  return (
    <button className="text-blue-600 px-2 border border-blue-300 rounded hover:border-blue-600 disabled:text-gray-400 disabled:border-gray-300 disabled:cursor-not-allowed" onClick={onClick} disabled={disabled}>
      {children}
    </button>
  )
}

function isStalled(table: TanstackTable<any>, rowIndex: number): boolean {
  return table.options.data[rowIndex]?.isPreviousData;
}

const SortIcon = ({ isSorted }: { isSorted: false | SortDirection }) => {
  switch (isSorted) {
    case false:
      return (
        <svg xmlns="http://www.w3.org/2000/svg" className="h-4 w-4 opacity-40" viewBox="0 0 20 20" fill="currentColor">
          <path d="M5 12a1 1 0 102 0V6.414l1.293 1.293a1 1 0 001.414-1.414l-3-3a1 1 0 00-1.414 0l-3 3a1 1 0 001.414 1.414L5 6.414V12zM15 8a1 1 0 10-2 0v5.586l-1.293-1.293a1 1 0 00-1.414 1.414l3 3a1 1 0 001.414 0l3-3a1 1 0 00-1.414-1.414L15 13.586V8z" />
        </svg>
      )
    case "asc":
      return (
        <svg xmlns="http://www.w3.org/2000/svg" className="h-4 w-4" viewBox="0 0 20 20" fill="currentColor">
          <path fillRule="evenodd" d="M5.293 9.707a1 1 0 010-1.414l4-4a1 1 0 011.414 0l4 4a1 1 0 01-1.414 1.414L11 7.414V15a1 1 0 11-2 0V7.414L6.707 9.707a1 1 0 01-1.414 0z" clipRule="evenodd" />
        </svg>
      )
    case "desc":
      return (
        <svg xmlns="http://www.w3.org/2000/svg" className="h-4 w-4" viewBox="0 0 20 20" fill="currentColor">
          <path fillRule="evenodd" d="M14.707 10.293a1 1 0 010 1.414l-4 4a1 1 0 01-1.414 0l-4-4a1 1 0 111.414-1.414L9 12.586V5a1 1 0 012 0v7.586l2.293-2.293a1 1 0 011.414 0z" clipRule="evenodd" />
        </svg>
      )
  }
};