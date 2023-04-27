import { useState, useMemo, useEffect } from "react";
import { getCoreRowModel, getPaginationRowModel, getSortedRowModel, PaginationState, SortingState, Table, useReactTable } from "@tanstack/react-table";
import { useQueries, useQuery } from "@tanstack/react-query";
import { App, RangeInput, RangeKind } from "../pkg/web";

type Deserializer<T> = (buffer: ArrayBuffer, id: bigint | string) => T;

type PathRetriever = (app: App, input: RangeInput, totalCount: number) => Promise<[bigint | string, string][]>;

export default function useDataTable<T>(app: App, plural: string, kind: RangeKind, deserializer: Deserializer<T>, pathRetriever: PathRetriever, columns, totalCount: number, defaultSort = "default"): Table<T> {

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

  const paths = useQuery(
    ["paths", plural, kind, pageIndex, pageSize, sortId, sortDesc],
    () => pathRetriever(app, {
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

  const queries = useQueries({
    queries: paths.data ? paths.data.map((path) => {
      return {
        queryKey: [path[1]],
        queryFn: () => fetch(path[1])
          .then(r => r.blob())
          .then(b => b.arrayBuffer())
          .then(a => deserializer(a, path[0])),
      }
    }) : [],
  });

  const defaultData = useMemo(() => [], [])

  return useReactTable(
    {
      columns: useMemo(() => columns, []),
      data: queries.map((q => q.data)),
      pageCount,
      state: {
        sorting, pagination
      },
      onSortingChange: setSorting,
      onPaginationChange: setPagination,
      getCoreRowModel: getCoreRowModel(),
      getSortedRowModel: getSortedRowModel(),
      getPaginationRowModel: getPaginationRowModel(),
      manualPagination: kind.kind != "epoch",
      manualSorting: kind.kind != "epoch"
    }
  );
}