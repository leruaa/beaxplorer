import { useEffect } from "react";
import cx from 'classnames';
import { Table, flexRender } from "@tanstack/react-table";

interface DataTableProps {
  table: Table<any>;
}

export default ({ table }: DataTableProps) => {

  const state = table.getState();

  return (
    // apply the table props
    <>
      <div className="pagination">
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
      <table>
        <thead>
          {// Loop over the header rows
            table.getHeaderGroups().map(headerGroup => (
              // Apply the header row props
              <tr key={headerGroup.id}>
                {// Loop over the headers in each row
                  headerGroup.headers.map(header => (
                    // Apply the header cell props
                    <th
                      key={header.id}
                      colSpan={header.colSpan}
                      className={cx({ sorting: header.column.getIsSorted() })}
                      onClick={header.column.getToggleSortingHandler()}
                    >
                      {flexRender(header.column.columnDef.header, header.getContext())}
                      <span className={`sort ${header.column.getIsSorted() ? header.column.getIsSorted() : 'neutral'}`}>
                      </span>
                    </th>
                  ))}
              </tr>
            ))}
        </thead>
        {/* Apply the table body props */}
        <tbody>
          {// Loop over the table rows
            table.getRowModel().rows.map(row => (
              <tr key={row.id}>
                {row.getVisibleCells().map(cell => (
                  <td key={cell.id}>
                    {flexRender(cell.column.columnDef.cell, cell.getContext())}
                  </td>
                )
                )}
              </tr>
            )
            )
          }
        </tbody>
      </table>

      <div className="pagination justify-end">
        <button onClick={() => table.setPageIndex(0)} disabled={!table.getCanPreviousPage()}>
          First
        </button>
        &nbsp;
        <button onClick={() => table.previousPage()} disabled={!table.getCanPreviousPage()}>
          {'<'}
        </button>
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
        <button onClick={() => table.nextPage()} disabled={!table.getCanNextPage()}>
          {'>'}
        </button>
        &nbsp;
        <button onClick={() => table.setPageIndex(table.getPageCount() - 1)} disabled={!table.getCanNextPage()}>
          Last
        </button>
      </div>
    </>
  )
}