import { useState, useMemo } from "react";
import { getCoreRowModel, getSortedRowModel, PaginationState, SortingState, Table, useReactTable } from "@tanstack/react-table";
import { useQuery } from "@tanstack/react-query";
import { App, RangeKind, getRange as fetchRange } from "../pkg/web";

type Fetcher<T> = (app: App, id: bigint | string) => Promise<T>

async function fetchAll<T>(app: App, fetcher: Fetcher<T>, range: bigint[] | string[]): Promise<T[]> {
  let promises = [];

  for (let id of range) {
    promises.push(fetcher(app, id));
  }

  return Promise.all(promises);
}

export default function useDataTable<T>(app: App, plural: string, kind: RangeKind, fetcher: Fetcher<T>, columns, totalCount: number, defaultSort = "default"): Table<T> {

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
      sortId: sorting.length == 0 ? defaultSort : sorting[0].id,
      sortDesc: sorting.length == 0 ? false : sorting[0].desc
    }),
    [sorting]);

  const range = useQuery(
    ["range", plural, pageIndex, pageSize, sortId, sortDesc],
    () => fetchRange(app, {
      settings: {
        pageIndex,
        pageSize,
        sortId,
        sortDesc
      },
      plural,
      kind
    },
      totalCount)
  );

  const rangeKey = useMemo(() => range.data?.range.join("|"), [range]);

  const query = useQuery(
    ["models", plural, rangeKey],
    () => fetchAll(app, fetcher, range.data.range),
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
      manualPagination: true,
      manualSorting: true
    }
  );
}