import { useState, useMemo, useEffect } from "react";
import { getCoreRowModel, getSortedRowModel, PaginationState, SortingState, Table, useReactTable } from "@tanstack/react-table";
import { useQuery } from "@tanstack/react-query";
import { App, getRange } from "../pkg/web";

type Fetcher<T> = (app: App, id: bigint) => Promise<T>

async function fetchAll<T>(app: App, fetcher: Fetcher<T>, range: BigUint64Array): Promise<T[]> {
  let promises = [];

  for (let id of range) {
    promises.push(fetcher(app, id));
  }

  return Promise.all(promises);
}

export default function useDataTable<T>(app: App, plural: string, fetcher: Fetcher<T>, columns, totalCount: number): Table<T> {

  const [sorting, setSorting] = useState<SortingState>([]);

  const [{ pageIndex, pageSize }, setPagination] =
    useState<PaginationState>({
      pageIndex: 0,
      pageSize: 10,
    });

  const pagination = useMemo(
    () => ({
      pageIndex,
      pageSize,
    }),
    [pageIndex, pageSize]
  );

  const pageCount = useMemo(
    () => Math.ceil(totalCount / pageSize),
    [pageSize]
  );

  const { sortId, sortDesc } = useMemo(
    () => ({
      sortId: sorting.length == 0 ? "default" : sorting[0].id,
      sortDesc: sorting.length == 0 ? false : sorting[0].desc
    }),
    [sorting]);

  const range = useQuery(
    ["range", plural, pageIndex, pageSize, sortId, sortDesc],
    () => getRange(app, plural, pageIndex, pageSize, sortId, sortDesc, totalCount)
  );

  const rangeKey = useMemo(() => range.data?.join("|"), [range]);

  const query = useQuery(
    ["models", plural, rangeKey],
    () => fetchAll(app, fetcher, range.data),
    {
      enabled: !!rangeKey,
      keepPreviousData: true
    }
  );

  const defaultData = useMemo(() => [], [])

  return useReactTable(
    {
      columns: useMemo(() => columns, []),
      data: query.data ?? defaultData,
      pageCount,
      state: {
        sorting, pagination,
      },
      onSortingChange: setSorting,
      onPaginationChange: setPagination,
      getCoreRowModel: getCoreRowModel(),
      getSortedRowModel: getSortedRowModel(),
      manualPagination: true
    }
  );
}