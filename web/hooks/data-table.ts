import { useState, useMemo } from "react";
import { getCoreRowModel, getSortedRowModel, PaginationState, SortingState, Table, useReactTable } from "@tanstack/react-table";
import { useQuery } from "react-query";
import { App } from "../pkg/web";

type Fetcher<T> = (app: App, pageIndex: number, pageSize: number, sortId: string, sortDesc: boolean, totalCount: number) => Promise<T[]>


export default function useDataTable<T>(app: App, key: string, fetcher: Fetcher<T>, columns, totalCount: number): Table<T> {

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

  const query = useQuery(
    [key, pageIndex, pageSize, sortId, sortDesc],
    () => fetcher(app, pageIndex, pageSize, sortId, sortDesc, totalCount)
  );

  return useReactTable(
    {
      columns: useMemo(() => columns, []),
      data: query.data ?? [],
      pageCount,
      state: {
        sorting, pagination,
      },
      onSortingChange: setSorting,
      onPaginationChange: setPagination,
      getCoreRowModel: getCoreRowModel(),
      getSortedRowModel: getSortedRowModel(),
      manualPagination: true,
    }
  );
}