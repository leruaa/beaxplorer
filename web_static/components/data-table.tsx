import { useEffect } from "react";
import cx from 'classnames';
import { useTable, usePagination, useSortBy } from "react-table";

export default ({ columns, data, fetchData, loading, pageIndex: initialPageIndex, pageCount: controlledPageCount, sortBy: initialSortBy }) => {
  const {
    getTableProps,
    getTableBodyProps,
    headerGroups,
    prepareRow,
    page,
    canPreviousPage,
    canNextPage,
    pageOptions,
    pageCount,
    gotoPage,
    nextPage,
    previousPage,
    setPageSize,
    state: { pageIndex, pageSize, sortBy },
  } = useTable(
    {
      columns,
      data,
      initialState: { pageIndex: initialPageIndex, sortBy: initialSortBy },
      manualPagination: true,
      pageCount: controlledPageCount,
      manualSortBy: true,
      disableSortRemove: true
    },
    useSortBy,
    usePagination
  )

  useEffect(() => {
    fetchData({ pageIndex, pageSize, sortBy })
  }, [fetchData, pageIndex, pageSize, sortBy]);

  return (
    // apply the table props
    <>
      <div className="pagination">
        <span>
          Show
          &nbsp;
          <select
            value={pageSize}
            onChange={e => {
              setPageSize(Number(e.target.value))
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
      <table {...getTableProps()}>
        <thead>
          {// Loop over the header rows
            headerGroups.map(headerGroup => (
              // Apply the header row props
              <tr {...headerGroup.getHeaderGroupProps()}>
                {// Loop over the headers in each row
                  headerGroup.headers.map(column => (
                    // Apply the header cell props
                    <th className={cx({ sorting: column.isSorted })} {...column.getHeaderProps(column.getSortByToggleProps())}>
                      {// Render the header
                        column.render('Header')}
                      <span className={`sort ${column.isSorted ? (column.isSortedDesc ? 'desc' : 'asc') : 'neutral'}`}>
                      </span>
                    </th>
                  ))}
              </tr>
            ))}
        </thead>
        {/* Apply the table body props */}
        <tbody {...getTableBodyProps()}>
          {// Loop over the table rows
            page.map(row => {
              // Prepare the row for display
              prepareRow(row)
              return (
                // Apply the row props
                <tr {...row.getRowProps()}>
                  {// Loop over the rows cells
                    row.cells.map(cell => {
                      // Apply the cell props
                      return (
                        <td className={cx({ sorting: cell.column.isSorted })} {...cell.getCellProps()}>
                          {// Render the cell contents
                            cell.render('Cell')}
                        </td>
                      )
                    })}
                </tr>
              )
            })}
        </tbody>
      </table>

      <div className="pagination justify-end">
        <button onClick={() => gotoPage(0)} disabled={!canPreviousPage}>
          First
        </button>
        &nbsp;
        <button onClick={() => previousPage()} disabled={!canPreviousPage}>
          {'<'}
        </button>
        &nbsp;
        <span>
          <input
            className="w-20"
            type="number"
            value={pageIndex + 1}
            onChange={e => {
              const page = e.target.value ? Number(e.target.value) - 1 : 0
              gotoPage(page)
            }}
          />
          &nbsp;of {pageOptions.length}
        </span>
        &nbsp;
        <button onClick={() => nextPage()} disabled={!canNextPage}>
          {'>'}
        </button>
        &nbsp;
        <button onClick={() => gotoPage(pageCount - 1)} disabled={!canNextPage}>
          Last
        </button>
      </div>
    </>
  )
}